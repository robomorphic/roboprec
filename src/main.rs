mod algorithms;
mod analysis;
mod codegen;
mod config;
mod helpers;
mod ir; // intermediate representation
mod logger;
mod tests;
mod types;

use anyhow::{Context, Result};
use std::vec;

// Re-export the types and macros at the crate root
pub use types::matrix::Matrix;
pub use types::scalar::Scalar;
pub use types::vector::Vector;

use crate::{
    algorithms::{
        forward_kinematics::{FKResult, forward_kinematics},
        rnea_derivatives::rneaderivatives,
        robots::{
            panda::{panda, panda_get_bounds},
            roarm_m2::{roarm_m2, roarm_m2_get_bounds},
        },
    },
    analysis::{analysis::analysis_main, real::Real},
    helpers::{cos_extremes, sin_extremes},
    ir::{
        helper::clear_all_names,
        program::{clear_program, get_program, register_matrix_output, register_scalar_output, register_vector_output},
    },
};

fn fk_par() -> Result<()> {
    const DOF: usize = 7;

    let joint_bounds = panda_get_bounds();
    let v_ranges = vec![(Real::from_f64(-0.5), Real::from_f64(0.5)); DOF];
    let a_ranges = vec![(Real::from_f64(-1.0), Real::from_f64(1.0)); DOF];

    let minmax_sin = joint_bounds
        .iter()
        .map(|min_max| {
            let (sin_min, sin_max) = sin_extremes(min_max.0.to_f64(), min_max.1.to_f64());
            (Real::from_f64(sin_min), Real::from_f64(sin_max))
        })
        .collect::<Vec<(Real, Real)>>();
    let minmax_cos = joint_bounds
        .iter()
        .map(|min_max| {
            let (cos_min, cos_max) = cos_extremes(min_max.0.to_f64(), min_max.1.to_f64());
            (Real::from_f64(cos_min), Real::from_f64(cos_max))
        })
        .collect::<Vec<(Real, Real)>>();

    let qsin =
        get_program().add_input_vector("qsin", minmax_sin.clone(), vec![0.0; DOF]);
    let qcos =
        get_program().add_input_vector("qcos", minmax_cos.clone(), vec![0.0; DOF]);
    let v = get_program().add_input_vector("v", v_ranges.clone(), vec![0.0; DOF]);
    let a = get_program().add_input_vector("a", a_ranges.clone(), vec![0.0; DOF]);

    let result = forward_kinematics(qcos, qsin, v, a, &panda());

    let FKResult {
        mut omi_translations,
        mut omi_rotations,
        mut all_v,
        mut all_a,
    } = result;
    omi_translations.iter_mut().enumerate().for_each(|(i, t)| {
        register_vector_output(t, &format!("omi_translation_{}", i));
    });
    omi_rotations.iter_mut().enumerate().for_each(|(i, r)| {
        register_matrix_output(r, &format!("omi_rotation_{}", i));
    });

    all_v.iter_mut().enumerate().for_each(|(i, v)| {
        register_vector_output(v, &format!("all_v_{}", i));
    });

    all_a.iter_mut().enumerate().for_each(|(i, a)| {
        register_vector_output(a, &format!("all_a_{}", i));
    });

    analysis_main().with_context(|| "Failed to analyze program")?;

    Ok(())
}

#[allow(dead_code)]
fn rnea_deriv_par() -> Result<()> {
    const DOF: usize = 4;

    let joint_bounds = roarm_m2_get_bounds();
    let v_ranges = vec![(Real::from_f64(-0.5), Real::from_f64(0.5)); DOF];
    let a_ranges = vec![(Real::from_f64(-1.0), Real::from_f64(1.0)); DOF];

    let minmax_sin = joint_bounds
        .iter()
        .map(|min_max| {
            let (sin_min, sin_max) = sin_extremes(min_max.0.to_f64(), min_max.1.to_f64());
            (Real::from_f64(sin_min), Real::from_f64(sin_max))
        })
        .collect::<Vec<(Real, Real)>>();
    let minmax_cos = joint_bounds
        .iter()
        .map(|min_max| {
            let (cos_min, cos_max) = cos_extremes(min_max.0.to_f64(), min_max.1.to_f64());
            (Real::from_f64(cos_min), Real::from_f64(cos_max))
        })
        .collect::<Vec<(Real, Real)>>();

    let qsin =
        get_program().add_input_vector("qsin", minmax_sin.clone(), vec![0.0; DOF]);
    let qcos =
        get_program().add_input_vector("qcos", minmax_cos.clone(), vec![0.0; DOF]);
    let v = get_program().add_input_vector("v", v_ranges.clone(), vec![0.0; DOF]);
    let a = get_program().add_input_vector("a", a_ranges.clone(), vec![0.0; DOF]);

    let result = rneaderivatives(qcos, qsin, v, a, &roarm_m2());

    let (mut rnea_partial_da, mut rnea_partial_dv, mut rnea_partial_dq, _) = result;
    register_matrix_output(&mut rnea_partial_da, "rnea_partial_da");
    register_matrix_output(&mut rnea_partial_dv, "rnea_partial_dv");
    register_matrix_output(&mut rnea_partial_dq, "rnea_partial_dq");

    analysis_main().with_context(|| "Failed to analyze program")?;

    Ok(())
}

#[allow(dead_code)]
fn rnea_deriv_par_7dof() -> Result<()> {
    const DOF: usize = 7;

    let v_ranges = vec![(Real::from_f64(-0.5), Real::from_f64(0.5)); DOF];
    let a_ranges = vec![(Real::from_f64(-1.0), Real::from_f64(1.0)); DOF];

    let joint_bounds = panda_get_bounds();
    let minmax_sin = joint_bounds
        .iter()
        .map(|min_max| {
            let (sin_min, sin_max) = sin_extremes(min_max.0.to_f64(), min_max.1.to_f64());
            (Real::from_f64(sin_min), Real::from_f64(sin_max))
        })
        .collect::<Vec<(Real, Real)>>();
    let minmax_cos = joint_bounds
        .iter()
        .map(|min_max| {
            let (cos_min, cos_max) = cos_extremes(min_max.0.to_f64(), min_max.1.to_f64());
            (Real::from_f64(cos_min), Real::from_f64(cos_max))
        })
        .collect::<Vec<(Real, Real)>>();

    let qsin =
        get_program().add_input_vector("qsin", minmax_sin.clone(), vec![0.0; DOF]);
    let qcos =
        get_program().add_input_vector("qcos", minmax_cos.clone(), vec![0.0; DOF]);
    let v = get_program().add_input_vector("v", v_ranges.clone(), vec![0.0; DOF]);
    let a = get_program().add_input_vector("a", a_ranges.clone(), vec![0.0; DOF]);

    let result = rneaderivatives(qcos, qsin, v, a, &panda());

    let (_rnea_partial_da, _rnea_partial_dv, _rnea_partial_dq, _) = result;

    analysis_main().with_context(|| "Failed to analyze program")?;

    Ok(())
}

fn main() -> Result<()> {
    //fk_par()?;
    //rnea_deriv_par()?;
    //rnea_deriv_par_7dof();

    let input_range: (Real, Real) = (Real::from_f64(0.0), Real::from_f64(1.0));
    let default_val = 0.5;
    let x = &get_program().add_input_scalar("x", input_range, default_val);

    // Do operation
    let mut res = x * x * x;

    // Register output
    register_scalar_output(&mut res, "output");

    // Do the analysis
    analysis_main()?;

    // If you want, you can check the output values in rust
    assert!(res.value.to_f64() == default_val.powi(3));



    Ok(())
}
