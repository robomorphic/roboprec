use roboprec::{Matrix, Scalar};

// Creates rotation matrix from pre-calculated trigonometric components
// RPY order is ZYX intrinsic: Yaw (Z), Pitch (Y), Roll (X)
pub(super) fn rpy_to_matrix_from_trig_components(
    cos_r: Scalar,
    sin_r: Scalar, // cos(roll), sin(roll)
    cos_p: Scalar,
    sin_p: Scalar, // cos(pitch), sin(pitch)
    cos_y: Scalar,
    sin_y: Scalar, // cos(yaw), sin(yaw)
) -> Matrix {
    let r11 = &cos_y * &cos_p;
    let r12 = &(&(&cos_y * &sin_p) * &sin_r) - &(&sin_y * &cos_r);
    let r13 = &(&cos_y * &sin_p) * &cos_r + (&sin_y * &sin_r);

    let r21 = &sin_y * &cos_p;
    let r22 = (&(&sin_y * &sin_p) * &sin_r) + (&cos_y * &cos_r);
    let r23 = &(&(&sin_y * &sin_p) * &cos_r) - &(&cos_y * &sin_r);

    let r31 = -&sin_p;
    let r32 = &cos_p * &sin_r;
    let r33 = &cos_p * &cos_r;

    Matrix!([
        r11, r12, r13;
        r21, r22, r23;
        r31, r32, r33;
    ])
}
