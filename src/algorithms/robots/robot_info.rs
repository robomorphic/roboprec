use roboprec::{Matrix, Vector};

pub struct RobotInfo {
    pub n_joints: usize,
    pub limi_translations: Vec<Vector>,
    pub calc_limi: Box<dyn Fn(Matrix, usize) -> Matrix>,
    pub joint_axes: Vec<usize>,
    pub levers: Vec<Vector>,
    pub masses: Vector,
    pub inertias: Vec<Matrix>,
}
