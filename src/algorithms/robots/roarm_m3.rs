use super::robot_info::RobotInfo;

use crate::{
    Matrix, Scalar, Vector, algorithms::robots::helper::rpy_to_matrix_from_trig_components,
    analysis::real::Real,
};

// Daisy uses these values, therefore if we use more approximate values, we will get different results
#[allow(clippy::approx_constant)]
pub fn calc_limi(
    // Made pub to match potential usage in RobotInfo
    rotation_matrix_joint: Matrix, // This is R_Z(theta_i) for the current joint's motion
    joint_index: usize,
) -> Matrix {
    // Determine RPY angle values (as f64 for matching) based on joint_index
    // These are the literal values from the URDF rpy attributes
    let (roll_angle_literal, pitch_angle_literal, yaw_angle_literal) = if joint_index == 0 {
        // URDF: base_link_to_link1: rpy="0 0 0"
        (0.0, 0.0, 0.0)
    } else if joint_index == 1 {
        // URDF: link1_to_link2: rpy="-1.5708 -1.5708 0"
        (-1.5708, -1.5708, 0.0)
    } else if joint_index == 2 {
        // URDF: link2_to_link3: rpy="0 0 1.5708"
        (0.0, 0.0, 1.5708)
    } else if joint_index == 3 {
        // URDF: link3_to_link4: rpy="0 0 0"
        (0.0, 0.0, 0.0)
    } else if joint_index == 4 {
        // URDF: link4_to_link5: rpy="1.5708 1.5708 0"
        (1.5708, 1.5708, 0.0)
    } else {
        panic!(
            "Unsupported joint_index for roarm_m3 calc_limi: {}",
            joint_index
        );
    };

    // Determine cos_r, sin_r based on roll_angle_literal, using direct numerical sin/cos values
    let (cos_r, sin_r) = match roll_angle_literal {
        0.0 => (Scalar!(1.0), Scalar!(0.0)), // cos(0), sin(0)
        1.5708 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.5708 => (
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
        1.5708 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.5708 => (
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
        1.5708 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.5708 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(-0.9999999932519606),
        ), // cos(-1.5708), sin(-1.5708)
        _ => panic!(
            "Internal error: Yaw angle literal {} not handled directly for sin/cos",
            yaw_angle_literal
        ),
    };

    // Construct the R_fix matrix using the selected trig components
    let r_fix = rpy_to_matrix_from_trig_components(cos_r, sin_r, cos_p, sin_p, cos_y, sin_y);

    // The final rotation is R_fix * R_joint(theta_i)
    r_fix.matmul(&rotation_matrix_joint)
}

#[allow(clippy::approx_constant)]
#[allow(dead_code)]
pub fn roarm_m2_get_bounds() -> Vec<(Real, Real)> {
    let joint_bounds = [
        (-3.1416, 3.1416),
        (-1.5708, 1.5708),
        (-1.0, 2.95),
        (-1.5708, 1.5708),
        (-3.1416, 3.1416),
    ];

    joint_bounds
        .map(|(min, max)| (Real::from_f64(min), Real::from_f64(max)))
        .to_vec()
}

#[allow(dead_code)]
pub fn roarm_m2() -> RobotInfo {
    let limi_translations = vec![
        Vector!([0.0, 0.0, 0.0]).define("limi_translation_0".to_string()), // base_link_to_link1
        Vector!([0.0, 0.0, 0.051959]).define("limi_translation_1".to_string()), // link1_to_link2
        Vector!([0.236815, 0.030002, 0.0]).define("limi_translation_2".to_string()), // link2_to_link3
        Vector!([0.0, -0.144586, 0.0]).define("limi_translation_3".to_string()), // link3_to_gripper_link
        Vector!([0.015147, -0.053653, 0.0]).define("limi_translation_4".to_string()),
    ];

    let n_joints = 5;

    let levers = vec![
        Vector!([0.0, 0.0, 0.037]).define("lever_0.".to_string()),
        Vector!([0.119, 0.012247, -0.0001]).define("lever_1".to_string()),
        Vector!([-0.0015, -0.0765, 0.00505]).define("lever_2".to_string()),
        Vector!([-0.0015, -0.022, -0.00075]).define("lever_3".to_string()),
        Vector!([-0.0078, 0.0, 0.0595]).define("lever_4".to_string()),
    ];

    let masses = Vector!([0.0729177, 0.0703216, 0.021608, 0.0099933, 0.0153928])
        .define("masses".to_string());

    let inertias = vec![
        // Inertia for link1 from URDF
        Matrix!([
            Scalar!(4.68465E-05), Scalar!(0.0), Scalar!(0.0);
            Scalar!(0.0), Scalar!(3.32107E-05), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.0), Scalar!(5.01023E-05);
        ])
        .define("inertia_0".to_string()),
        // Inertia for link2 from URDF
        Matrix!([
            Scalar!(5.46501E-05), Scalar!(0.0), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.000423091), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.0), Scalar!(0.000404557)
        ])
        .define("inertia_1".to_string()),
        // Inertia for link3 from URDF
        Matrix!([
            Scalar!(0.0000637699), Scalar!(0.0), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.00000367026), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.0), Scalar!(0.0000628965)
        ])
        .define("inertia_2".to_string()),
        // Inertia for gripper_link from URDF
        Matrix!([
            Scalar!(0.00000453979), Scalar!(0.0), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.00000378103), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.0), Scalar!(0.00000560238)
        ])
        .define("inertia_3".to_string()),
        Matrix!([
            Scalar!(0.0000202869), Scalar!(0.0), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.0000214427), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.0), Scalar!(0.00000439495)
        ])
        .define("inertia_4".to_string()),
    ];

    // All revolute joints in the URDF specify <axis xyz="0 0 1"/> (Z-axis)
    let joint_axes = vec![2, 2, 2, 2, 2];

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
