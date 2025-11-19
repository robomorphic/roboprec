use anyhow::{Context, Result};
use std::vec;
use crate::ir::program::add_input_vector;

use crate::{
    algorithms::{
        forward_kinematics::{FKResult, forward_kinematics},
        rnea_derivatives::rneaderivatives,
        robots::{
            panda::{panda, panda_get_bounds},
            roarm_m2::{roarm_m2, roarm_m2_get_bounds},
        },
    },
    analysis::{analysis::analysis, real::Real},
    config::Config,
    helpers::{cos_extremes, sin_extremes},
    ir::{
        program::{register_matrix_output, register_vector_output},
    },
};

pub(super) fn fk_7dof(config: Config) -> Result<()> {
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
        add_input_vector("qsin", minmax_sin.clone(), vec![0.0; DOF]);
    let qcos =
        add_input_vector("qcos", minmax_cos.clone(), vec![0.0; DOF]);
    let v = add_input_vector("v", v_ranges.clone(), vec![0.0; DOF]);
    let a = add_input_vector("a", a_ranges.clone(), vec![0.0; DOF]);

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

    analysis(config).with_context(|| "Failed to analyze program")?;

    Ok(())
}

#[allow(dead_code)]
pub(super) fn rnea_deriv_4dof(config: Config) -> Result<()> {
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
        add_input_vector("qsin", minmax_sin.clone(), vec![0.0; DOF]);
    let qcos =
        add_input_vector("qcos", minmax_cos.clone(), vec![0.0; DOF]);
    let v = add_input_vector("v", v_ranges.clone(), vec![0.0; DOF]);
    let a = add_input_vector("a", a_ranges.clone(), vec![0.0; DOF]);

    let result = rneaderivatives(qcos, qsin, v, a, &roarm_m2());

    let (mut rnea_partial_da, mut rnea_partial_dv, mut rnea_partial_dq, _) = result;
    register_matrix_output(&mut rnea_partial_da, "rnea_partial_da");
    register_matrix_output(&mut rnea_partial_dv, "rnea_partial_dv");
    register_matrix_output(&mut rnea_partial_dq, "rnea_partial_dq");

    analysis(config).with_context(|| "Failed to analyze program")?;

    Ok(())
}

#[allow(dead_code)]
pub(super) fn rnea_deriv_7dof(config: Config) -> Result<()> {
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
        add_input_vector("qsin", minmax_sin.clone(), vec![0.0; DOF]);
    let qcos =
        add_input_vector("qcos", minmax_cos.clone(), vec![0.0; DOF]);
    let v = add_input_vector("v", v_ranges.clone(), vec![0.0; DOF]);
    let a = add_input_vector("a", a_ranges.clone(), vec![0.0; DOF]);

    let result = rneaderivatives(qcos, qsin, v, a, &panda());

    let (_rnea_partial_da, _rnea_partial_dv, _rnea_partial_dq, _) = result;

    analysis(config).with_context(|| "Failed to analyze program")?;

    Ok(())
}
