use anyhow::Result;
use log::info;
use std::io::Write;

use crate::{
    ir::{
        expr::{Expr, Opr, OprBinary, OprUnary},
        program::{Program, ProgramInput},
    },
};

pub fn generate_daisy_dsl(program: &Program) -> Result<()> {
    info!("Generating Daisy DSL code...");
    let inputs = program.get_inputs();
    let body = program.get_body();
    let func_name = crate::config::CODEGEN_FILENAME;

    let mut generated_code = String::new();

    generated_code.push_str("import daisy.lang._\n");
    generated_code.push_str("import Real._\n");
    generated_code.push_str(format!("object {} {{\ndef {}(\n", func_name, func_name).as_str());

    // print inputs
    for (i, (id, input)) in inputs.iter().enumerate() {
        match input {
            ProgramInput::Scalar { .. } => {
                generated_code.push_str(format!("  {}: Real\n", id.name()).as_str());
                if i < inputs.len() - 1 {
                    generated_code.push_str(", ");
                }
            }
            _ => {
                anyhow::bail!("Input: {:#?} should have been unrolled", input)
            }
        }
    }
    // TODO: be careful with output here!
    generated_code.push_str("): Real = {{\n");

    // now print ranges
    generated_code.push_str("require (\n");
    for (i, (id, input)) in inputs.iter().enumerate() {
        match input {
            ProgramInput::Scalar { info } => {
                // format: name < range[0] && name < range[1]
                generated_code.push_str(
                    format!(
                        "{} > {} && {} < {} \n",
                        id.name(),
                        info.range.0.to_f64(),
                        id.name(),
                        info.range.1.to_f64()
                    )
                    .as_str(),
                );
            }
            ProgramInput::Vector { info } => {
                // need to unroll
                let size = info.len();
                for (j, element) in info.iter().enumerate() {
                    generated_code.push_str(
                        (format!(
                            "{}_{} > {} && {}_{} < {}\n",
                            id.name(),
                            j,
                            element.range.0.to_f64(),
                            id.name(),
                            j,
                            element.range.1.to_f64()
                        ))
                        .as_str(),
                    );
                    if j < size - 1 || i < inputs.len() - 1 {
                        generated_code.push_str(" && ");
                    }
                }
            }
            ProgramInput::Matrix { info } => {
                // need to unroll
                let rows = info.len();
                let cols = info[0].len();
                for (j, row) in info.iter().enumerate() {
                    for (k, element) in row.iter().enumerate() {
                        generated_code.push_str(
                            format!(
                                "{}_{}_{} > {} && {}_{}_{} < {}\n",
                                id.name(),
                                j,
                                k,
                                element.range.0.to_f64(),
                                id.name(),
                                j,
                                k,
                                element.range.1.to_f64()
                            )
                            .as_str(),
                        );
                        if k < cols - 1 || j < rows - 1 || i < inputs.len() - 1 {
                            generated_code.push_str(" && ");
                        }
                    }
                }
            }
        }
        // if not the last input, add a " && "
        if i < inputs.len() - 1 {
            generated_code.push_str(" && ");
        }
    }
    generated_code.push_str(")\n");

    // print body
    for expr in body.iter() {
        let expr_str: String = match expr {
            Expr::Let {
                id: var_id,
                opr: rhs,
            } => match rhs {
                Opr::ConstantScalar { value } => {
                    format!("val {}: Real = {}\n", var_id.name(), value.to_f64())
                }
                Opr::Unary { opr1, opr_type } => match opr_type {
                    OprUnary::Neg => format!("val {} = -{}\n", var_id.name(), opr1.name()),
                    OprUnary::Assign | OprUnary::AssignNoOpt => {
                        format!("val {} = {}\n", var_id.name(), opr1.name())
                    }
                    OprUnary::Sine => format!("val {} = sin({})\n", var_id.name(), opr1.name()),
                    OprUnary::Cosine => format!("val {} = sin({})\n", var_id.name(), opr1.name()),
                    _ => anyhow::bail!("The operation {:#?} should have been unrolled", rhs),
                },
                Opr::Binary {
                    opr1,
                    opr2,
                    opr_type,
                } => {
                    let opr = match opr_type {
                        OprBinary::Add => "+",
                        OprBinary::Sub => "-",
                        OprBinary::Mul => "*",
                        OprBinary::Div => "/",
                        _ => anyhow::bail!("The operation {:#?} should have been unrolled", rhs),
                    };

                    format!(
                        "val {} = {} {} {}\n",
                        var_id.name(),
                        opr1.name(),
                        opr,
                        opr2.name()
                    )
                }
                _ => anyhow::bail!("The operation {:#?} should have been unrolled", rhs),
            },
        };
        generated_code.push_str(&expr_str);
    }

    // get the last expression as output
    if let Some(last_expr) = body.last() {
        match last_expr {
            Expr::Let { id, .. } => {
                generated_code.push_str(format!("{}\n", id.name()).as_str());
            }
        }
    } else {
        anyhow::bail!("Program body is empty, cannot determine output");
    }

    generated_code.push_str("}}}\n");

    // Create directory and write to file
    let folder = format!("{}/daisy", crate::config::CODEGEN_DIR);
    std::fs::create_dir_all(&folder).expect("Failed to create codegen directory for Daisy DSL");
    let filename = format!("{}/{}.scala", folder, crate::config::CODEGEN_FILENAME);

    let mut file = match std::fs::File::create(filename.clone()) {
        Ok(f) => f,
        Err(e) => anyhow::bail!("Unable to create file {}: {}", filename, e),
    };

    file.write_all(generated_code.as_bytes())
        .expect("Unable to write to file");

    Ok(())
}
