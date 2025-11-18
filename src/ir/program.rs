use indexmap::IndexMap;
use log::info;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::{
    Matrix, Scalar, Vector,
    analysis::{daisy::DaisyRange, real::Real},
    ir::{expr::Expr, identifier::Identifier},
};

// TODO: add error
#[derive(Debug, Clone)]
pub struct Input {
    pub range: (Real, Real),
}

#[derive(Debug, Clone)]
pub enum ProgramInput {
    Scalar { info: Input },
    Vector { info: Vec<Input> },
    Matrix { info: Vec<Vec<Input>> },
}

#[derive(Debug, Clone)]
pub struct Output {
    pub id: Identifier,
    pub range: (Real, Real),
    pub error: (Real, Real),
}

#[derive(Debug, Clone)]
pub enum ProgramOutput {
    Scalar { info: Output },
    Vector { info: Vec<Output> },
    Matrix { info: Vec<Vec<Output>> },
}

#[derive(Debug, Clone)]
pub struct Program {
    inputs: IndexMap<Identifier, ProgramInput>,
    outputs: IndexMap<Identifier, ProgramOutput>,
    body: Vec<Expr>,
}

pub fn register_scalar_output(output: &mut Scalar, name: &str) {
    *output = output.define(name.to_string());
    get_program().outputs.insert(
        output.id.clone(),
        ProgramOutput::Scalar {
            info: Output {
                // These are trash values
                id: output.id.clone(),
                range: (Real::zero(), Real::zero()),
                error: (Real::zero(), Real::zero()),
            },
        },
    );
}

pub fn register_vector_output(output: &mut Vector, name: &str) {
    *output = output.define(name.to_string());
    get_program().outputs.insert(
        output.id.clone(),
        ProgramOutput::Vector {
            info: output
                .value
                .iter()
                .map(|_| Output {
                    // These are trash values
                    id: output.id.clone(),
                    range: (Real::zero(), Real::zero()),
                    error: (Real::zero(), Real::zero()),
                })
                .collect(),
        },
    );
}

pub fn register_matrix_output(output: &mut Matrix, name: &str) {
    *output = output.define(name.to_string());
    get_program().outputs.insert(
        output.id.clone(),
        ProgramOutput::Matrix {
            info: output
                .value
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|_| Output {
                            // These are trash values
                            id: output.id.clone(),
                            range: (Real::zero(), Real::zero()),
                            error: (Real::zero(), Real::zero()),
                        })
                        .collect()
                })
                .collect(),
        },
    );
    // Set output to be new_output
}

impl Program {
    pub fn new() -> Self {
        Self {
            inputs: IndexMap::new(),
            outputs: IndexMap::new(),
            body: Vec::new(),
        }
    }

    pub fn add_input_scalar(
        &mut self,
        name: &str,
        range: (Real, Real),
        default_value: f64,
    ) -> Scalar {
        let new_id = Identifier::new_scalar(&name);
        let info = Input { range };
        self.inputs
            .insert(new_id.clone(), ProgramInput::Scalar { info });

        Scalar {
            id: new_id,
            value: Real::from_f64(default_value),
        }
    }

    pub fn add_input_vector(
        &mut self,
        name: &str,
        range: Vec<(Real, Real)>,
        default_value: Vec<f64>,
    ) -> Vector {
        let size = default_value.len();
        let new_id = Identifier::new_vector(&name, size);

        let info = range.into_iter().map(|range| Input { range }).collect();
        self.inputs
            .insert(new_id.clone(), ProgramInput::Vector { info });

        Vector {
            id: new_id,
            value: default_value.into_iter().map(Real::from_f64).collect(),
        }
    }

    pub fn add_input_matrix(
        &mut self,
        name: &str,
        range: Vec<Vec<(Real, Real)>>,
        default_value: Vec<Vec<f64>>,
    ) -> Matrix {
        let size0 = range.len();
        let size1 = range[0].len();
        let new_id = Identifier::new_matrix(&name, size0, size1);
        let info = range
            .into_iter()
            .map(|row_range| row_range.into_iter().map(|range| Input { range }).collect())
            .collect();
        self.inputs
            .insert(new_id.clone(), ProgramInput::Matrix { info });

        Matrix {
            id: new_id,
            value: default_value
                .into_iter()
                .map(|row| row.into_iter().map(Real::from_f64).collect())
                .collect(),
        }
    }

    pub fn add_expr(&mut self, expr: Expr) {
        self.body.push(expr);
    }

    pub fn get_inputs(&self) -> &IndexMap<Identifier, ProgramInput> {
        &self.inputs
    }

    pub fn get_outputs(&self) -> &IndexMap<Identifier, ProgramOutput> {
        &self.outputs
    }

    pub fn get_body(&self) -> &Vec<Expr> {
        &self.body
    }

    pub fn set_inputs(&mut self, inputs: &IndexMap<Identifier, ProgramInput>) {
        self.inputs = inputs.clone();
    }

    pub fn set_outputs(&mut self, outputs: &IndexMap<Identifier, ProgramOutput>) {
        self.outputs = outputs.clone();
    }

    pub fn set_body(&mut self, body: &[Expr]) {
        self.body = body.to_vec();
    }

    pub fn get_curr_intervals(&self) -> IndexMap<Identifier, (Real, Real)> {
        let mut res = IndexMap::new();

        for (id, input) in &self.inputs {
            match input {
                ProgramInput::Scalar { info } => {
                    res.insert(id.clone(), info.range.clone());
                }
                ProgramInput::Vector { .. } => {
                    panic!("You should have call unroll before get_curr_intervals")
                }
                ProgramInput::Matrix { .. } => {
                    panic!("You should have call unroll before get_curr_intervals")
                }
            }
        }

        res
    }
}

static PROGRAM: Lazy<Mutex<Program>> = Lazy::new(|| Mutex::new(Program::new()));

pub fn get_program() -> std::sync::MutexGuard<'static, Program> {
    PROGRAM.lock().unwrap()
}

pub fn set_program(program: Program) {
    let mut prog = get_program();
    *prog = program;
}

pub fn clear_program() {
    let mut program = get_program();
    program.inputs.clear();
    program.outputs.clear();
    program.body.clear();
}

pub fn update_program_outputs(
    program: &mut Program,
    range_results: &IndexMap<String, DaisyRange>,
    error_results: &IndexMap<String, f64>,
) {
    let curr_outputs = program.get_outputs().clone();
    let mut new_outputs = IndexMap::new();
    for (id, output) in curr_outputs {
        match output {
            ProgramOutput::Scalar { info: prev_info } => {
                let range = range_results.get(&prev_info.id.name).unwrap();
                let range_real = (
                    Real::from_f64(range.lower),
                    Real::from_f64(range.upper),
                );
                let error = error_results.get(&prev_info.id.name).unwrap();
                let error_real = (
                    Real::from_f64(-error.abs()),
                    Real::from_f64(error.abs()),
                );
                let new_output = ProgramOutput::Scalar {
                    info: Output {
                        id: prev_info.id.clone(),
                        range: range_real,
                        error: error_real,
                    },
                };
                new_outputs.insert(id.clone(), new_output);
            }
            ProgramOutput::Vector { info: prev_infos } => {
                let mut new_info = vec![];
                for prev_info in prev_infos {
                    let range = range_results.get(&prev_info.id.name).unwrap();
                    let range_real = (
                        Real::from_f64(range.lower),
                        Real::from_f64(range.upper),
                    );
                    let error = error_results.get(&prev_info.id.name).unwrap();
                    let error_real = (
                        Real::from_f64(-error.abs()),
                        Real::from_f64(error.abs()),
                    );
                    new_info.push(Output {
                        id: prev_info.id.clone(),
                        range: range_real,
                        error: error_real,
                    });
                }
                let new_output = ProgramOutput::Vector { info: new_info };
                new_outputs.insert(id.clone(), new_output);
            }
            ProgramOutput::Matrix { info: prev_infos } => {
                let mut new_info = vec![];
                for row in prev_infos {
                    let mut new_row = vec![];
                    for prev_info in row {
                        let range = range_results.get(&prev_info.id.name).unwrap();
                        let range_real = (
                            Real::from_f64(range.lower),
                            Real::from_f64(range.upper),
                        );
                        let error = error_results.get(&prev_info.id.name).unwrap();
                        let error_real = (
                            Real::from_f64(-error.abs()),
                            Real::from_f64(error.abs()),
                        );
                        new_row.push(Output {
                            id: prev_info.id.clone(),
                            range: range_real,
                            error: error_real,
                        });
                    }
                    new_info.push(new_row);
                }
                let new_output = ProgramOutput::Matrix { info: new_info };
                new_outputs.insert(id.clone(), new_output);
            }
        }
    }

    program.set_outputs(&new_outputs);
}


pub fn report_analysis_ranges(program: &Program) {
    info!("Reporting analysis ranges:");
    for (id, output) in program.get_outputs() {
        match output {
            ProgramOutput::Scalar { info } => {
                let lower = info.range.0.to_f64();
                let upper = info.range.1.to_f64();
                info!(
                    "Output: {}, Range: {:?}",
                    id.name(),
                    (lower, upper)
                );
            }
            ProgramOutput::Vector { info: infos } => {
                // do a pretty print, with float .8 decimal places
                info!("Output vector ranges: {}", id.name());
                for (i, info) in infos.iter().enumerate() {
                    let lower = info.range.0.to_f64();
                    let upper = info.range.1.to_f64();
                    info!("  [{}]: {:?}", i, (lower, upper));
                }
            }
            ProgramOutput::Matrix { info: infos } => {
                // do a pretty print, with float .8 decimal places
                info!("Output matrix ranges: {}", id.name());
                for (row_idx, row) in infos.iter().enumerate() {
                    let mut row_str = format!("  [{}]: ", row_idx);
                    for (col_idx, info) in row.iter().enumerate() {
                        let lower = info.range.0.to_f64();
                        let upper = info.range.1.to_f64();
                        row_str.push_str(&format!("[{}]: {:30?}  ", col_idx, (lower, upper)));
                    }
                    info!("{}", row_str);
                }
            }
        }
    }
}

pub fn report_analysis_errors(program: &Program) {
    // report errors of all outputs
    info!("Reporting analysis errors:");
    for (id, output) in program.get_outputs() {
        match output {
            ProgramOutput::Scalar { info } => {
                let lower = info.error.0.to_f64();
                info!(
                    "Output: {}, Error: +/-{:?}",
                    id.name(),
                    lower.abs()
                );
            }
            ProgramOutput::Vector { info: infos } => {
                // do a pretty print, with float .8 decimal places
                info!("Output vector errors: {}", id.name());
                for (i, info) in infos.iter().enumerate() {
                    let lower = info.error.0.to_f64();
                    info!("  [{}]: +/-{:?}", i, lower.abs());
                }
            }
            ProgramOutput::Matrix { info: infos } => {
                // do a pretty print, with float .8 decimal places
                info!("Output matrix errors: {}", id.name());
                for (row_idx, row) in infos.iter().enumerate() {
                    let mut row_str = format!("  [{}]: ", row_idx);
                    for (col_idx, info) in row.iter().enumerate() {
                        let lower = info.error.0.to_f64();
                        row_str.push_str(&format!("[{}]: +/-{:30?}  ", col_idx, lower.abs()));
                    }
                    info!("{}", row_str);
                }
            }
        }
    }
}


pub fn report_worst_values(
    range_results: &IndexMap<String, DaisyRange>,
    program: &Program,
) {
    let mut worst_range_min: Option<Real> = None;
    let mut worst_range_max: Option<Real> = None;
    let mut worst_error_min: Option<Real> = None;
    let mut worst_error_max: Option<Real> = None;

    // iterate over range_results to find worst min and max
    for (_id, range) in range_results {
        let min = Real::from_f64(range.lower);
        let max = Real::from_f64(range.upper);

        worst_range_min = match worst_range_min {
            Some(current_worst) => Some(current_worst.min(min)),
            None => Some(min),
        };
        worst_range_max = match worst_range_max {
            Some(current_worst) => Some(current_worst.max(max)),
            None => Some(max),
        };
    }

    let mut update_errors = |error: &(Real, Real)| {
        let min = &error.0;
        let max = &error.1;
        worst_error_min = match &worst_error_min {
            Some(current_worst) => Some(current_worst.min(min)),
            None => Some(min.clone()),
        };
        worst_error_max = match &worst_error_max {
            Some(current_worst) => Some(current_worst.max(max)),
            None => Some(max.clone()),
        };
    };

    // we only care about the output value's errors
    for (_, output) in program.get_outputs() {
        match output {
            ProgramOutput::Scalar { info } => {
                update_errors(&info.error);
            }
            ProgramOutput::Vector { info: infos } => {
                // do a pretty print, with float .8 decimal places
                for info in infos.iter() {
                    update_errors(&info.error);
                }
            }
            ProgramOutput::Matrix { info: infos } => {
                // do a pretty print, with float .8 decimal places
                for row in infos.iter() {
                    for info in row.iter() {
                        update_errors(&info.error);
                    }
                }
            }
        }
    }

    info!("Worst Range Min: {:?}", worst_range_min.unwrap().to_f64());
    info!("Worst Range Max: {:?}", worst_range_max.unwrap().to_f64());
    info!("Worst Error Min: {:?}", worst_error_min.unwrap().to_f64());
    info!("Worst Error Max: {:?}", worst_error_max.unwrap().to_f64());
}
