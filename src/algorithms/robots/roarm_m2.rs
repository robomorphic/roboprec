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
        // URDF: link3_to_gripper_link: rpy="-1.5708 0 -1.5708"
        (-1.5708, 0.0, -1.5708)
    } else {
        panic!("Unsupported joint_index for calc_limi: {}", joint_index);
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
    let r_fix = rpy_to_matrix_from_trig_components(cos_r, sin_r, cos_p, sin_p, cos_y, sin_y)
        .define(format!("R_fix_{}", joint_index));

    // The final rotation is R_fix * R_joint(theta_i)
    //let result_matrix =
    //    r_fix.matmul(&rotation_matrix_joint.define(format!("R_joint_{}", joint_index)));
    r_fix.matmul(&rotation_matrix_joint)
}

// Daisy uses these values, therefore if we use more approximate values, we will get different results
#[allow(clippy::approx_constant)]
#[allow(dead_code)]
pub fn roarm_m2_get_bounds() -> Vec<(Real, Real)> {
    let joint_bounds = [
        (-3.1416, 3.1416), // base_link_to_link1
        (-1.5708, 1.5708), // link1_to_link2
        (-1.0, 2.95),      // link2_to_link3
        (0.0, 1.5),        // link3_to_gripper_link
    ];

    joint_bounds
        .map(|(min, max)| (Real::from_f64(min), Real::from_f64(max)))
        .to_vec()
}

#[allow(dead_code)]
pub fn roarm_m2() -> RobotInfo {
    let limi_translations = vec![
        Vector!([0.0100000008759151, 0.0, 0.123059270461044])
            .define("limi_translation_0".to_string()), // base_link_to_link1
        Vector!([0.0, 0.0, 0.0]).define("limi_translation_1".to_string()), // link1_to_link2
        Vector!([0.236815132922094, 0.0300023995170449, 0.0])
            .define("limi_translation_2".to_string()), // link2_to_link3
        Vector!([0.002906, -0.21599, -0.00066683]).define("limi_translation_3".to_string()), // link3_to_gripper_link
    ];

    let n_joints = 4;

    let levers = vec![
        Vector!([0.0, 0.0, -0.015]).define("lever_0".to_string()), // link1
        Vector!([0.122, 0.0, 0.0]).define("lever_1".to_string()),  // link2
        Vector!([0.002, -0.13687, 0.0059]).define("lever_2".to_string()), // link3
        Vector!([0.029, 0.0027, -0.00078]).define("lever_3".to_string()), // gripper_link
    ];

    let masses = Vector!([0.0729177, 0.0703216, 0.0269773, 0.0028708]).define("masses".to_string()); // link1, link2, link3, gripper_link

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
            Scalar!(0.000199591), Scalar!(0.0), Scalar!(0.0);
            Scalar!(0.0), Scalar!(8.01674E-06), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.0), Scalar!(0.000196918)
        ])
        .define("inertia_2".to_string()),
        // Inertia for gripper_link from URDF
        Matrix!([
            Scalar!(5.23216E-07), Scalar!(0.0), Scalar!(0.0);
            Scalar!(0.0), Scalar!(1.82071E-06), Scalar!(0.0);
            Scalar!(0.0), Scalar!(0.0), Scalar!(1.60231E-06)
        ])
        .define("inertia_3".to_string()),
    ];

    // All revolute joints in the URDF specify <axis xyz="0 0 1"/> (Z-axis)
    let joint_axes = vec![2, 2, 2, 2];

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
