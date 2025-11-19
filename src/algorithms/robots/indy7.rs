use super::robot_info::RobotInfo;

use roboprec::{Matrix, Scalar, Vector, Real};
use crate::algorithms::robots::helper::rpy_to_matrix_from_trig_components;

// clippy is right, but I want to keep the constants as is to match Daisy and URDF values
// We should change it in the future
#[allow(clippy::approx_constant)]
pub fn calc_limi(
    rotation_matrix_joint: Matrix, // This is R_Z(theta_i) for the current joint's motion
    joint_index: usize,
) -> Matrix {
    // Determine RPY angle values (as f64 for matching) based on joint_index
    // These are the literal values from the Indy7 URDF rpy attributes
    let (roll_angle_literal, pitch_angle_literal, yaw_angle_literal) = if joint_index == 0 {
        // URDF: joint0: rpy="0 0 0"
        (0.0, 0.0, 0.0)
    } else if joint_index == 1 {
        // URDF: joint1: rpy="1.570796327 1.570796327 0"
        (1.570796327, 1.570796327, 0.0)
    } else if joint_index == 2 {
        // URDF: joint2: rpy="0 0 0"
        (0.0, 0.0, 0.0)
    } else if joint_index == 3 {
        // URDF: joint3: rpy="-1.570796327 0 1.570796327"
        (-1.570796327, 0.0, 1.570796327)
    } else if joint_index == 4 {
        // URDF: joint4: rpy="1.570796327 1.570796327 0"
        (1.570796327, 1.570796327, 0.0)
    } else if joint_index == 5 {
        // URDF: joint5: rpy="-1.570796327 0 1.570796327"
        (-1.570796327, 0.0, 1.570796327)
    } else {
        panic!(
            "Unsupported joint_index for Indy7 calc_limi: {}",
            joint_index
        );
    };

    // Determine cos_r, sin_r based on roll_angle_literal, using direct numerical sin/cos values
    let (cos_r, sin_r) = match roll_angle_literal {
        0.0 => (Scalar!(1.0), Scalar!(0.0)), // cos(0), sin(0)
        1.570796327 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.570796327 => (
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
        1.570796327 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.570796327 => (
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
        1.570796327 => (
            Scalar!(-0.000003673205100000002),
            Scalar!(0.9999999932519606),
        ), // cos(1.5708), sin(1.5708)
        -1.570796327 => (
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
pub fn indy7_get_bounds() -> Vec<(Real, Real)> {
    let joint_bounds = [
        (-3.0543261909900767, 3.0543261909900767),
        (-3.0543261909900767, 3.0543261909900767),
        (-3.0543261909900767, 3.0543261909900767),
        (-3.0543261909900767, 3.0543261909900767),
        (-3.0543261909900767, 3.0543261909900767),
        (-3.7524578917878086, 3.7524578917878086),
    ];

    joint_bounds
        .map(|(min, max)| (Real::from_f64(min), Real::from_f64(max)))
        .to_vec()
}

#[allow(dead_code)]
pub fn indy7() -> RobotInfo {
    let limi_translations = vec![
        Vector!([0.0, 0.0, 0.0775]).define("limi_translation_0".to_string()),
        Vector!([0.0, -0.109, 0.222]).define("limi_translation_1".to_string()),
        Vector!([-0.45, 0.0, -0.0305]).define("limi_translation_2".to_string()),
        Vector!([-0.267, 0.0, -0.075]).define("limi_translation_3".to_string()),
        Vector!([0.0, -0.114, 0.083]).define("limi_translation_4".to_string()),
        Vector!([-0.168, 0.0, 0.069]).define("limi_translation_5".to_string()),
    ];

    let n_joints = 6;

    let levers = vec![
        Vector!([-0.00023749, -0.04310313, 0.13245396]).define("lever_0".to_string()),
        Vector!([-0.29616699, 2.254e-05, 0.04483069]).define("lever_1".to_string()),
        Vector!([-0.16804016, 0.00021421, -0.07000383]).define("lever_2".to_string()),
        Vector!([-0.00026847, -0.0709844, 0.07649128]).define("lever_3".to_string()),
        Vector!([-0.09796232, -0.00023114, 0.06445892]).define("lever_4".to_string()),
        Vector!([8.147e-05, -0.00046556, 0.03079097]).define("lever_5".to_string()),
    ];

    let masses = Vector!([
        11.44444535,
        5.84766553,
        2.68206064,
        2.12987371,
        2.22412271,
        0.38254932
    ])
    .define("masses".to_string());

    let inertias = vec![
        Matrix!([
            Scalar!(0.35065005), Scalar!(0.00011931), Scalar!(-0.00037553);
            Scalar!(0.00011931), Scalar!(0.304798), Scalar!(-0.10984447);
            Scalar!(-0.00037553), Scalar!(-0.10984447), Scalar!(0.06003147)
        ])
        .define("inertia_0".to_string()),
        Matrix!([
            Scalar!(0.03599743), Scalar!(-4.693e-05), Scalar!(-0.05240346);
            Scalar!(-4.693e-05), Scalar!(0.72293306), Scalar!(1.76e-06);
            Scalar!(-0.05240346), Scalar!(1.76e-06), Scalar!(0.70024119)
        ])
        .define("inertia_1".to_string()),
        Matrix!([
            Scalar!(0.0161721), Scalar!(-0.00011817), Scalar!(0.03341882);
            Scalar!(-0.00011817), Scalar!(0.11364055), Scalar!(-4.371e-05);
            Scalar!(0.03341882), Scalar!(-4.371e-05), Scalar!(0.10022522)
        ])
        .define("inertia_2".to_string()),
        Matrix!([
            Scalar!(0.02798891), Scalar!(3.893e-05), Scalar!(-4.768e-05);
            Scalar!(3.893e-05), Scalar!(0.01443076), Scalar!(-0.01266296);
            Scalar!(-4.768e-05), Scalar!(-0.01266296), Scalar!(0.01496211)
        ])
        .define("inertia_3".to_string()),
        Matrix!([
            Scalar!(0.01105297), Scalar!(5.517e-05), Scalar!(-0.01481977);
            Scalar!(5.517e-05), Scalar!(0.03698291), Scalar!(-3.74e-05);
            Scalar!(-0.01481977), Scalar!(-3.74e-05), Scalar!(0.02754795)
        ])
        .define("inertia_4".to_string()),
        Matrix!([
            Scalar!(0.00078982), Scalar!(-3.4e-07), Scalar!(8.3e-07);
            Scalar!(-3.4e-07), Scalar!(0.00079764), Scalar!(-5.08e-06);
            Scalar!(8.3e-07), Scalar!(-5.08e-06), Scalar!(0.00058319)
        ])
        .define("inertia_5".to_string()),
    ];

    let joint_axes = vec![2, 2, 2, 2, 2, 2];

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
