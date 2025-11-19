use anyhow::Result;
use core::panic;
use indexmap::IndexMap;
use log::error;
use log::info;
use std::io::Write;

use crate::{
    analysis::real::Real,
    ir::{
        expr::{Expr, Opr, OprBinary, OprUnary},
        precision::Precision,
        program::{Program, ProgramInput, ProgramOutput},
    },
};

fn precision_to_type(precision: &Precision) -> String {
    match precision {
        Precision::Fixed {
            total_bits,
            fractional_bits: _,
        } => {
            if total_bits <= &8 {
                "int8_t".to_string()
            } else if total_bits <= &16 {
                "int16_t".to_string()
            } else if total_bits <= &32 {
                "int32_t".to_string()
            } else if total_bits <= &64 {
                "int64_t".to_string()
            } else {
                panic!("Fixed precision with more than 64 bits is not supported in C codegen");
            }
        }
        Precision::Float32 => "float".to_string(),
        Precision::Float64 => "double".to_string(),
    }
}

fn fixed_precision_to_next_type(precision: &Precision) -> Result<String> {
    match precision {
        Precision::Fixed {
            total_bits,
            fractional_bits: _,
        } => {
            if total_bits <= &8 {
                Ok("int16_t".to_string())
            } else if total_bits <= &16 {
                Ok("int32_t".to_string())
            } else if total_bits <= &32 {
                Ok("int64_t".to_string())
            } else if total_bits <= &64 {
                anyhow::bail!("Next precision beyond 64 bits is not supported in C codegen");
            } else {
                anyhow::bail!(
                    "Fixed precision with more than 64 bits is not supported in C codegen"
                );
            }
        }
        Precision::Float32 => Ok("float".to_string()),
        Precision::Float64 => Ok("double".to_string()),
    }
}

fn value_precision_to_str(value: &Real, precision: &Precision) -> String {
    match precision {
        Precision::Fixed {
            total_bits: _,
            fractional_bits,
        } => {
            // first we need to scale the number
            let scale = Real::from_i64(2i64.pow(*fractional_bits as u32));
            let scaled_value = value * &scale;

            format!("{}", scaled_value.nearest_integer())
        }
        Precision::Float32 | Precision::Float64 => {
            panic!("C does not support floating point, fix and test first!")
        }
    }
}

pub fn generate_c(program: &Program, precisions: &IndexMap<String, Precision>) -> Result<()> {
    info!("Generating C code...");
    // create a string for the file, so we can write to it at once
    let inputs = program.get_inputs();
    let body = program.get_body();
    let outputs = program.get_outputs();
    let func_name = crate::config::CODEGEN_FILENAME;

    let mut generated_code = String::new();

    // first prints
    generated_code.push_str("#include <math.h>\n");

    // first print the function return type
    generated_code.push_str("\ntypedef struct {\n");
    for (id, output) in outputs {
        match output {
            ProgramOutput::Scalar { info } => {
                let name = id.name();
                let precision = precisions.get(&name.clone()).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Precision for output variable {} not found in precisions map",
                        name
                    )
                })?;
                generated_code.push_str(
                    format!(
                        "    {} {};\n",
                        precision_to_type(&precision),
                        id.name() // in this case we use id.name, in the function body we'll use info.id.name
                    )
                    .as_str(),
                );
            }
            ProgramOutput::Vector { info: infos } => {
                for (i, info) in infos.iter().enumerate() {
                    let name = info.id.name();
                    let precision = precisions.get(&name.clone()).ok_or_else(|| {
                        anyhow::anyhow!(
                            "Precision for output variable {} not found in precisions map",
                            name
                        )
                    })?;
                    generated_code.push_str(
                        format!(
                            "    {} {}_{};\n",
                            precision_to_type(&precision),
                            id.name(),
                            i
                        )
                        .as_str(),
                    );
                }
            }
            ProgramOutput::Matrix { info: infos } => {
                for (i, row) in infos.iter().enumerate() {
                    for (j, info) in row.iter().enumerate() {
                        let name = info.id.name();
                        let precision = precisions.get(&name.clone()).ok_or_else(|| {
                            anyhow::anyhow!(
                                "Precision for output variable {} not found in precisions map",
                                name
                            )
                        })?;
                        generated_code.push_str(
                            format!(
                                "    {} {}_{}_{};\n",
                                precision_to_type(&precision),
                                id.name(),
                                i,
                                j
                            )
                            .as_str(),
                        );
                    }
                }
            }
        }
    }
    generated_code.push_str(format!("}} {}_output_t;\n", func_name).as_str());

    // then print the function signature
    generated_code.push_str(format!("\n{}_output_t {}(\n", func_name, func_name).as_str());

    for (i, (id, input)) in inputs.iter().enumerate() {
        match input {
            ProgramInput::Scalar { info } => {
                let name = id.name();
                let precision = precisions.get(&name.clone()).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Precision for input variable {} not found in precisions map",
                        name
                    )
                })?;
                generated_code.push_str(
                    format!("    {} {}", precision_to_type(&precision), id.name()).as_str(),
                );
                if i != inputs.len() - 1 {
                    generated_code.push_str(",\n");
                }
            }
            ProgramInput::Vector { .. } | ProgramInput::Matrix { .. } => {
                panic!("Vector and Matrix should have been unrolled before codegen")
            }
        }
    }
    generated_code.push_str("\n) {\n");

    // now write the body
    for expr in body {
        match expr {
            Expr::Let {
                id: var_id,
                opr: rhs,
            } => {
                let name = var_id.name();
                let precision = precisions.get(&name.clone()).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Precision for variable {} not found in precisions map",
                        name
                    )
                })?;
                let rhs = match rhs {
                    Opr::ConstantScalar { value } => value_precision_to_str(value, precision),
                    Opr::ConstructScalar { id } => id.name().to_string(),
                    Opr::Unary { opr1, opr_type } => match opr_type {
                        OprUnary::Neg => {
                            format!("-({})", opr1.name())
                        }
                        OprUnary::Assign | OprUnary::AssignNoOpt => opr1.name().to_string(),
                        OprUnary::Sine => {
                            if let Precision::Fixed { .. } = precision {
                                generated_code.push_str(
                                    "   // Sine with fixed precision is not supported in C codegen",
                                );
                                error!("Sine with fixed precision is not supported in C codegen");
                            };
                            format!("sin({})", opr1.name())
                        }
                        OprUnary::Cosine => {
                            if let Precision::Fixed { .. } = precision {
                                generated_code.push_str(
                                        "   // Cosine with fixed precision is not supported in C codegen"  
                                    );
                                error!("Cosine with fixed precision is not supported in C codegen");
                            };
                            format!("cos({})", opr1.name())
                        }
                        _ => {
                            panic!("The operation {:#?} should have been unrolled", rhs)
                        }
                    },
                    Opr::Binary {
                        opr1,
                        opr2,
                        opr_type,
                    } => {
                        let opr1_name = opr1.name();
                        let opr2_name = opr2.name();
                        let precision1 = precisions.get(&opr1_name.clone()).ok_or_else(|| {
                            panic!(
                                "Precision for variable {} not found in precisions map",
                                opr1.name()
                            )
                        })?;
                        let precision2 = precisions.get(&opr2_name.clone()).ok_or_else(|| {
                            panic!(
                                "Precision for variable {} not found in precisions map",
                                opr2.name()
                            )
                        })?;
                        match opr_type {
                            OprBinary::Add => {
                                match (precision, precision1, precision2) {
                                    (
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fbgoal,
                                        },
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fb1,
                                        },
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fb2,
                                        },
                                    ) => {
                                        // for both, we need to cast to the goal precision
                                        // the format is precision var_id = (var_1, shift, or cast as needed) + (var_2, shift, or cast as needed)
                                        // TODO: when do we need to cast?
                                        // if the fractional bits are different, we need to shift
                                        let opr1 = if fb1 < fbgoal {
                                            format!("({} << {})", opr1.name(), fbgoal - fb1)
                                        } else if fb1 > fbgoal {
                                            format!("({} >> {})", opr1.name(), fb1 - fbgoal)
                                        } else {
                                            opr1.name().to_string()
                                        };
                                        let opr2 = if fb2 < fbgoal {
                                            format!("({} << {})", opr2.name(), fbgoal - fb2)
                                        } else if fb2 > fbgoal {
                                            format!("({} >> {})", opr2.name(), fb2 - fbgoal)
                                        } else {
                                            opr2.name().to_string()
                                        };
                                        format!("({} + {})", opr1, opr2)
                                    }
                                    (
                                        Precision::Float32,
                                        Precision::Float32,
                                        Precision::Float32,
                                    )
                                    | (
                                        Precision::Float64,
                                        Precision::Float64,
                                        Precision::Float64,
                                    ) => {
                                        panic!(
                                            "C codegen does not support floating point, fix and test first!"
                                        )
                                    }
                                    _ => panic!(
                                        "Mixed types are not supported in C codegen, {:#?} + {:#?} are mixed",
                                        precision1, precision2
                                    ),
                                }
                            }
                            OprBinary::Sub => {
                                match (precision, precision1, precision2) {
                                    (
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fbgoal,
                                        },
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fb1,
                                        },
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fb2,
                                        },
                                    ) => {
                                        // for both, we need to cast to the goal precision
                                        // the format is precision var_id = (var_1, shift, or cast as needed) + (var_2, shift, or cast as needed)
                                        // TODO: when do we need to cast?
                                        let opr1 = if fb1 < fbgoal {
                                            format!("({} << {})", opr1.name(), fbgoal - fb1)
                                        } else if fb1 > fbgoal {
                                            format!("({} >> {})", opr1.name(), fb1 - fbgoal)
                                        } else {
                                            opr1.name().to_string()
                                        };
                                        let opr2 = if fb2 < fbgoal {
                                            format!("({} << {})", opr2.name(), fbgoal - fb2)
                                        } else if fb2 > fbgoal {
                                            format!("({} >> {})", opr2.name(), fb2 - fbgoal)
                                        } else {
                                            opr2.name().to_string()
                                        };
                                        format!("({} - {})", opr1, opr2)
                                    }
                                    (
                                        Precision::Float32,
                                        Precision::Float32,
                                        Precision::Float32,
                                    )
                                    | (
                                        Precision::Float64,
                                        Precision::Float64,
                                        Precision::Float64,
                                    ) => {
                                        panic!(
                                            "C codegen does not support floating point, fix and test first!"
                                        )
                                    }
                                    _ => panic!(
                                        "Mixed types are not supported in C codegen, {:#?} - {:#?} are mixed",
                                        precision1, precision2
                                    ),
                                }
                            }
                            OprBinary::Mul => {
                                match (precision, precision1, precision2) {
                                    (
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fbgoal,
                                        },
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fb1,
                                        },
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fb2,
                                        },
                                    ) => {
                                        // for both, we need to cast to the goal precision
                                        // the format is precision var_id = (var_1, shift, or cast as needed) + (var_2, shift, or cast as needed)
                                        // TODO: when do we need to cast?
                                        // Daisy does explicit casting, so let's do it here too
                                        // cast to original precision first
                                        let str1 = format!("({}) ", precision_to_type(precision));

                                        let str2 = format!(
                                            "(((({}) ({})",
                                            fixed_precision_to_next_type(precision1)?,
                                            opr1.name()
                                        );

                                        let str3 = " * ";

                                        let str4 = format!(
                                            "({}) ({}))",
                                            fixed_precision_to_next_type(precision2)?,
                                            opr2.name()
                                        );

                                        // now shift to right by fractional bits
                                        // shift by flhs + frhs - fres
                                        let str5 = format!(" >> {}))", fb1 + fb2 - fbgoal);
                                        //let str5 = format!(" >> {}))", fb2);
                                        format!("{}{}{}{}", str1, str2, str3, str4 + &str5)
                                    }
                                    (
                                        Precision::Float32,
                                        Precision::Float32,
                                        Precision::Float32,
                                    )
                                    | (
                                        Precision::Float64,
                                        Precision::Float64,
                                        Precision::Float64,
                                    ) => {
                                        panic!(
                                            "C codegen does not support floating point, fix and test first!"
                                        )
                                    }
                                    _ => panic!(
                                        "Mixed types are not supported in C codegen, {:#?} - {:#?} are mixed",
                                        precision1, precision2
                                    ),
                                }
                            }
                            OprBinary::Div => {
                                match (precision, precision1, precision2) {
                                    (
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fbgoal,
                                        },
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fb1,
                                        },
                                        Precision::Fixed {
                                            total_bits: _,
                                            fractional_bits: fb2,
                                        },
                                    ) => {
                                        // for both, we need to cast to the goal precision
                                        // the format is precision var_id = (var_1, shift, or cast as needed) + (var_2, shift, or cast as needed)
                                        // the first value is casted to next precision and shifted by fractional bits
                                        let str1 = format!(
                                            "((({}) ({}) << {})",
                                            fixed_precision_to_next_type(precision1)?,
                                            opr1.name(),
                                            fbgoal + fb2 - fb1 // shift left by fres + frhs - flhs
                                        );

                                        let str2 = " / ";

                                        let str3 = format!("{})", opr2.name());
                                        format!("{}{}{}", str1, str2, str3)
                                    }
                                    (
                                        Precision::Float32,
                                        Precision::Float32,
                                        Precision::Float32,
                                    )
                                    | (
                                        Precision::Float64,
                                        Precision::Float64,
                                        Precision::Float64,
                                    ) => {
                                        panic!(
                                            "C codegen does not support floating point, fix and test first!"
                                        )
                                    }
                                    _ => panic!(
                                        "Mixed types are not supported in C codegen, {:#?} - {:#?} are mixed",
                                        precision1, precision2
                                    ),
                                }
                            }
                            _ => {
                                panic!("The operation {:#?} should have been unrolled", rhs)
                            }
                        }
                    }
                    _ => {
                        panic!("The operation {:#?} should have been unrolled", rhs)
                    }
                };

                generated_code.push_str(
                    format!(
                        "    {} {} = {};\n",
                        precision_to_type(precision),
                        var_id.name(),
                        rhs
                    )
                    .as_str(),
                );
            }
        };
    }

    // now, print the return statement
    // it is a struct containing all the outputs
    generated_code.push_str("\n    return {\n");
    for (_, output) in outputs {
        match output {
            ProgramOutput::Scalar { info } => {
                generated_code.push_str(
                    format!(
                        "        {},\n",
                        info.id.name() // in this case we use id.name, in the function body we'll use info.id.name
                    )
                    .as_str(),
                );
            }
            ProgramOutput::Vector { info: infos } => {
                for info in infos.iter() {
                    generated_code.push_str(format!("        {},\n", info.id.name()).as_str());
                }
            }
            ProgramOutput::Matrix { info: infos } => {
                for row in infos.iter() {
                    for info in row.iter() {
                        generated_code.push_str(format!("        {},\n", info.id.name()).as_str());
                    }
                }
            }
        }
    }
    generated_code.push_str("    };\n");

    generated_code.push_str("}\n");

    let folder = format!("{}/C", crate::config::CODEGEN_DIR);
    std::fs::create_dir_all(&folder).expect("Failed to create codegen directory for C");
    let filename = format!("{}/{}.cpp", folder, crate::config::CODEGEN_FILENAME);
    let mut file = match std::fs::File::create(filename.clone()) {
        Ok(f) => f,
        Err(e) => anyhow::bail!("Unable to create file {}: {}", filename, e),
    };

    file.write_all(generated_code.as_bytes())
        .expect("Unable to write to file");

    Ok(())
}
