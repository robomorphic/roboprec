use super::robot_info::RobotInfo;

use crate::{
    Matrix, Scalar, Vector, algorithms::robots::helper::rpy_to_matrix_from_trig_components,
    analysis::real::Real,
};

// clippy is right, but I want to keep the constants as is to match Daisy and URDF values
// We should change it in the future
#[allow(clippy::approx_constant)]
pub fn calc_limi(
    rotation_matrix_joint: Matrix, // This is R_Z(theta_i) for the current joint's motion
    joint_index: usize,
) -> Matrix {
    // Determine RPY angle values (as f64 for matching) based on joint_index
    // These are the literal values from the Panda URDF rpy attributes
    let (roll_angle_literal, pitch_angle_literal, yaw_angle_literal) = if joint_index == 0 {
        // URDF: panda_joint1: rpy="0 0 0"
        (0.0, 0.0, 0.0)
    } else if joint_index == 1 {
        // URDF: panda_joint2: rpy="-1.5707963267948966 0 0"
        (-1.5707963267948966, 0.0, 0.0)
    } else if joint_index == 2 {
        // URDF: panda_joint3: rpy="1.5707963267948966 0 0"
        (1.5707963267948966, 0.0, 0.0)
    } else if joint_index == 3 {
        // URDF: panda_joint4: rpy="1.5707963267948966 0 0"
        (1.5707963267948966, 0.0, 0.0)
    } else if joint_index == 4 {
        // URDF: panda_joint5: rpy="-1.5707963267948966 0 0"
        (-1.5707963267948966, 0.0, 0.0)
    } else if joint_index == 5 {
        // URDF: panda_joint6: rpy="1.5707963267948966 0 0"
        (1.5707963267948966, 0.0, 0.0)
    } else if joint_index == 6 {
        // URDF: panda_joint7: rpy="1.5707963267948966 0 0"
        (1.5707963267948966, 0.0, 0.0)
    } else {
        panic!(
            "Unsupported joint_index for Panda calc_limi: {}",
            joint_index
        );
    };

    // Determine cos_r, sin_r based on roll_angle_literal, using direct numerical sin/cos values
    let (cos_r, sin_r) = match roll_angle_literal {
        0.0 => (Scalar!(1.0), Scalar!(0.0)), // cos(0), sin(0)
        1.5707963267948966 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.5707963267948966 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(-0.9999999932519606),
        ), // cos(-1.5708), sin(-1.5708)
        _ => panic!(
            "Internal error: Roll angle literal {} not handled directly for sin/cos",
            roll_angle_literal
        ),
    };

    // Determine cos_p, sin_p based on pitch_angle_literal
    let (cos_p, sin_p) = match pitch_angle_literal {
        0.0 => (Scalar!(1.0), Scalar!(0.0)), // cos(0), sin(0)
        1.5707963267948966 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.5707963267948966 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(-0.9999999932519606),
        ), // cos(-1.5708), sin(-1.5708)
        _ => panic!(
            "Internal error: Pitch angle literal {} not handled directly for sin/cos",
            pitch_angle_literal
        ),
    };

    // Determine cos_y, sin_y based on yaw_angle_literal
    let (cos_y, sin_y) = match yaw_angle_literal {
        0.0 => (Scalar!(1.0), Scalar!(0.0)), // cos(0), sin(0)
        1.5707963267948966 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.5707963267948966 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(-0.9999999932519606),
        ), // cos(-1.5708), sin(-1.5708)
        _ => panic!(
            "Internal error: Yaw angle literal {} not handled directly for sin/cos",
            yaw_angle_literal
        ),
    };

    // Construct the R_fix matrix using the selected trig components
    let r_fix = rpy_to_matrix_from_trig_components(cos_r, sin_r, cos_p, sin_p, cos_y, sin_y)
        .define(format!("R_fix_{}", joint_index));

    // The final rotation is R_fix * R_joint(theta_i)
    // Assuming your Matrix<3,3> type has `impl Mul for Matrix<3,3>`
    //let result_matrix =
    //    r_fix.matmul(&rotation_matrix_joint.define(format!("R_joint_{}", joint_index)));
    r_fix.matmul(&rotation_matrix_joint)
}

#[allow(dead_code)]
pub fn panda_get_bounds() -> Vec<(Real, Real)> {
    let joint_bounds = [
        (-2.8973, 2.8973),
        (-1.7628, 1.7628),
        (-2.8973, 2.8973),
        (-3.0718, -0.0698),
        (-2.8973, 2.8973),
        (-0.0175, 3.7525),
        (-2.8973, 2.8973),
    ];

    joint_bounds
        .map(|(min, max)| (Real::from_f64(min), Real::from_f64(max)))
        .to_vec()
}

#[allow(dead_code)]
pub fn panda() -> RobotInfo {
    let limi_translations = vec![
        Vector!([0.0, 0.0, 0.333]).define("limi_translation_0".to_string()),
        Vector!([0.0, 0.0, 0.0]).define("limi_translation_1".to_string()),
        Vector!([0.0, -0.316, 0.0]).define("limi_translation_2".to_string()),
        Vector!([0.0825, 0.0, 0.0]).define("limi_translation_3".to_string()),
        Vector!([-0.0825, 0.384, 0.0]).define("limi_translation_4".to_string()),
        Vector!([0.0, 0.0, 0.0]).define("limi_translation_5".to_string()),
        Vector!([0.088, 0.0, 0.0]).define("limi_translation_6".to_string()),
    ];

    let n_joints = 7;

    let levers = vec![
        Vector!([0.003875, 0.002081, -0.04762]).define("lever_0".to_string()),
        Vector!([-0.003141, -0.02872, 0.003495]).define("lever_1".to_string()),
        Vector!([0.027518, 0.039252, -0.066502]).define("lever_2".to_string()),
        Vector!([-0.05317, 0.104419, 0.027454]).define("lever_3".to_string()),
        Vector!([-0.011953, 0.041065, -0.038437]).define("lever_4".to_string()),
        Vector!([0.060149, -0.014117, -0.010517]).define("lever_5".to_string()),
        Vector!([0.010517, -0.004252, 0.061597]).define("lever_6".to_string()),
    ];

    let masses = Vector!([
        4.970684, 0.646926, 3.228604, 3.587895, 1.225946, 1.666555, 0.735522
    ])
    .define("masses".to_string());

    let inertias = vec![
        Matrix!([
            Scalar!(0.70337), Scalar!(-0.000139), Scalar!(0.006772);
            Scalar!(-0.000139), Scalar!(0.70661), Scalar!(0.019169);
            Scalar!(0.006772), Scalar!(0.019169), Scalar!(0.009117)
        ])
        .define("inertia_0".to_string()),
        Matrix!([
            Scalar!(0.007962), Scalar!(-0.003925), Scalar!(0.010254);
            Scalar!(-0.003925), Scalar!(0.02811), Scalar!(0.000704);
            Scalar!(0.010254), Scalar!(0.000704), Scalar!(0.025995)
        ])
        .define("inertia_1".to_string()),
        Matrix!([
            Scalar!(0.037242), Scalar!(-0.004761), Scalar!(-0.011396);
            Scalar!(-0.004761), Scalar!(0.036155), Scalar!(-0.012805);
            Scalar!(-0.011396), Scalar!(-0.012805), Scalar!(0.01083)
        ])
        .define("inertia_2".to_string()),
        Matrix!([
            Scalar!(0.025853), Scalar!(0.007796), Scalar!(-0.001332);
            Scalar!(0.007796), Scalar!(0.019552), Scalar!(0.008641);
            Scalar!(-0.001332), Scalar!(0.008641), Scalar!(0.028323)
        ])
        .define("inertia_3".to_string()),
        Matrix!([
            Scalar!(0.035549), Scalar!(-0.002117), Scalar!(-0.004037);
            Scalar!(-0.002117), Scalar!(0.029474), Scalar!(0.000229);
            Scalar!(-0.004037), Scalar!(0.000229), Scalar!(0.008627)
        ])
        .define("inertia_4".to_string()),
        Matrix!([
            Scalar!(0.001964), Scalar!(0.000109), Scalar!(-0.001158);
            Scalar!(0.000109), Scalar!(0.004354), Scalar!(0.000341);
            Scalar!(-0.001158), Scalar!(0.000341), Scalar!(0.005433)
        ])
        .define("inertia_5".to_string()),
        Matrix!([
            Scalar!(0.012516), Scalar!(-0.000428), Scalar!(-0.001196);
            Scalar!(-0.000428), Scalar!(0.010027), Scalar!(-0.000741);
            Scalar!(-0.001196), Scalar!(-0.000741), Scalar!(0.004815)
        ])
        .define("inertia_6".to_string()),
    ];

    let joint_axes = vec![2, 2, 2, 2, 2, 2, 2]; // All joints are <axis xyz="0 0 1"/> (Z-axis)

    RobotInfo {
        n_joints,
        limi_translations,
        calc_limi: Box::new(calc_limi),
        joint_axes,
        levers,
        masses,
        inertias,
    }
}
