use anyhow::Result;
use log::info;

use crate::{
    analysis::daisy::{write_errors_to_file, write_precisions_to_file, write_ranges_to_file},
    codegen::{c::generate_c, c_with_conversion::generate_c_with_conversion, daisy_dsl::generate_daisy_dsl}, 
    config::Config,
    ir::{
        precision::Precision,
        program::{Program, get_program, report_analysis_errors, report_analysis_ranges, report_worst_values, update_program_outputs},
        unroll::unroll_ir,
    }, logger::setup_logger
};

/// This one runs error analysis and returns its results
/// The range results should be slightly different than analysis_range_only,
/// Because in this version we also care about roundoff errors
pub fn analysis_main(config: Config) -> Result<Program> {
    let log_file_path = match setup_logger() {
        Ok(path) => path,
        Err(e) => anyhow::bail!("Failed to set up logger: {}", e),
    };
    let start_time = std::time::Instant::now();

    info!("Current precision: {:#?}", config.precision);

    // Perform unrolling here
    let mut program = unroll_ir(&get_program());

    println!("Starting worst case analysis...");
    // create folder if not exist
    let folder = &config.codegen_dir;
    std::fs::create_dir_all(folder).unwrap();
    // TODO: call daisy here
    match generate_daisy_dsl(&program, &config) {
        Ok(_) => (),
        Err(e) => anyhow::bail!("Code generation failed: {}", e),
    }

    // Then, we run daisy with the generated code
    // TODO: Have a proper way to specify daisy path
    let daisy_directory = "./daisy/";
    // Run "rm daisy_directory + "ranges.txt" to remove previous results
    std::fs::remove_file(format!("{}ranges.txt", daisy_directory)).ok();
    std::fs::remove_file(format!("{}errors.txt", daisy_directory)).ok();
    std::fs::remove_file(format!("{}precisions.txt", daisy_directory)).ok();

    // Then, we run
    // os.system(f"cd ../daisy && ./daisy --codegen --lang=C --precision={precision} --rangeMethod=interval --errorMethod=interval ../quanta/{file_path}")
    // file_path is the path to the generated daisy code
    // TODO: add a choice for ap_fixed
    // TODO: add a choice for precision
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let daisy_directory = manifest_dir.join("daisy");
    let daisy_binary = daisy_directory.join("daisy");

    let scala_file = config.codegen_dir
        .join("daisy")
        .join(format!("{}.scala", config.codegen_filename));
    let scala_file = std::fs::canonicalize(scala_file)?;

    // before running, run mkdir daisy_directory + "output"
    std::fs::create_dir_all(daisy_directory.join("output"))?;

    let daisy_status = std::process::Command::new(&daisy_binary)
        .current_dir(&daisy_directory)
        .args([
            "--codegen",
            "--lang=C", // TODO: add ap_fixed option
            "--apfixed",
        ])
        .arg(format!("--precision={}", config.precision))
        .args([
            "--rangeMethod=interval",
            "--errorMethod=interval",
        ])
        .arg(&scala_file)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;

    if !daisy_status.success() {
        anyhow::bail!("Daisy analysis failed");
    }

    let range_results =
        crate::analysis::daisy::parse_daisy_ranges(daisy_directory.join("ranges.txt"))?;
    let error_results =
        crate::analysis::daisy::parse_daisy_errors(daisy_directory.join("errors.txt"))?;
    let precision_results =
        crate::analysis::daisy::parse_daisy_precisions(daisy_directory.join("precisions.txt"), &range_results)?;
    
    // after getting results, we can generate C now!
    generate_c(&program, &precision_results, &config)?;
    generate_c_with_conversion(&program, &precision_results, &config)?;
    
    update_program_outputs(
        &mut program,
        &range_results,
        &error_results,
    );


    report_analysis_ranges(&program);
    report_analysis_errors(&program);
    report_worst_values(&range_results, &program);

    // Finally, we copy the codegen to our output directory and log the new file path to user
    // from daisy_directory + "output" + scala_file.name() to config.output_dir + scala_file.name()
    let input_file = daisy_directory.join("output").join("codegen.cpp"); // TODO: make this dynamic
    let output_dir = config.codegen_dir.join("apfixed");
    // generate output directory if not exist
    std::fs::create_dir_all(&output_dir)?;
    let output_file = output_dir.join("codegen.cpp"); // TODO: make this dynamic
    std::fs::copy(
        input_file,
        &output_file,
    )?;

    let output_dir = config.codegen_dir.join("analysis_data");
    std::fs::create_dir_all(&output_dir)?;
    // Write all ranges and all errors in output directory, too
    let ranges_output_file = output_dir.join("analysis_ranges.txt");
    let errors_output_file = output_dir.join("analysis_errors.txt");
    let precisions_output_file = output_dir.join("analysis_precisions.txt");
    write_ranges_to_file(&range_results, &ranges_output_file)?;
    write_errors_to_file(&error_results, &errors_output_file)?;
    write_precisions_to_file(&precision_results, &precisions_output_file)?;
    
    let duration = start_time.elapsed();
    println!("Total analysis time: {:?}", duration);
    println!("Logs are saved in {}", log_file_path.display());
    println!("Codegen output saved in {}", output_file.display());
    println!("Analysis ranges saved in {}", ranges_output_file.display());
    println!("Analysis errors saved in {}", errors_output_file.display());
    println!("Analysis precisions saved in {}", precisions_output_file.display());

    Ok(program)
}
