use super::robots::robot_info::RobotInfo;
use roboprec::{Scalar, Vector, Matrix};

fn act_motion_inv(
    translation: Vector,
    rotation: Matrix,
    current: Vector,
    parent: Vector,
) -> Vector {
    let linear_parent = Vector!([parent.at(0), parent.at(1), parent.at(2);]);
    let angular_parent = Vector!([parent.at(3), parent.at(4), parent.at(5);]);
    let linear_current = Vector!([current.at(0), current.at(1), current.at(2);]);
    let angular_current = Vector!([current.at(3), current.at(4), current.at(5);]);

    let act_inv1 = translation.cross(&angular_parent);
    let act_inv2 = &linear_parent - &act_inv1;
    let act_inv3 = rotation.transpose();
    let act_inv4 = act_inv3.matmul_vec(&act_inv2);
    let new_linear = &linear_current + &act_inv4;
    let act_inv5 = &act_inv3.matmul_vec(&angular_parent);
    let new_angular = &angular_current + act_inv5;
    Vector!([
        new_linear.at(0),
        new_linear.at(1),
        new_linear.at(2),
        new_angular.at(0),
        new_angular.at(1),
        new_angular.at(2);
    ])
}

fn alpha_cross_linear(s: &Scalar, vin: Vector) -> Vector {
    let alpha_cross1 = &(-s) * &vin.at(1);
    let alpha_cross2 = s * &vin.at(0);
    Vector!([alpha_cross1, alpha_cross2, Scalar!(0.0);])
}

fn alpha_cross_angular(s: &Scalar, vin: Vector) -> Vector {
    let alpha_cross1 = &(-s) * &vin.at(1);
    let alpha_cross2 = s * &vin.at(0);
    Vector!([alpha_cross1, alpha_cross2, Scalar!(0.0);])
}

fn alpha_cross(s: Scalar, vin: Vector) -> Vector {
    let vin_linear = Vector!([
        vin.at(0),
        vin.at(1),
        vin.at(2);
    ]);
    let vin_angular = Vector!([
        vin.at(3),
        vin.at(4),
        vin.at(5);
    ]);
    let alpha_cross_linear = alpha_cross_linear(&s, vin_linear);
    let alpha_cross_angular = alpha_cross_angular(&s, vin_angular);
    Vector!([
        alpha_cross_linear.at(0),
        alpha_cross_linear.at(1),
        alpha_cross_linear.at(2),
        alpha_cross_angular.at(0),
        alpha_cross_angular.at(1),
        alpha_cross_angular.at(2);
    ])
}

fn act_inv(translation: Vector, rotation: Matrix, parent: Vector) -> Vector {
    let linear_parent = Vector!([parent.at(0), parent.at(1), parent.at(2);]);
    let angular_parent = Vector!([parent.at(3), parent.at(4), parent.at(5);]);
    let act_inv1 = translation.cross(&angular_parent);
    let act_inv2 = &linear_parent - &act_inv1;
    let act_inv3 = rotation.transpose();
    let act_inv4 = &act_inv3.matmul_vec(&act_inv2);
    let act_inv5 = &act_inv3.matmul_vec(&angular_parent);

    Vector!([
        act_inv4.at(0),
        act_inv4.at(1),
        act_inv4.at(2),
        act_inv5.at(0),
        act_inv5.at(1),
        act_inv5.at(2);
    ])
}

pub fn forward_kinematics_helper(
    qcos: Vector,
    qsin: Vector,
    all_joint_v: Vector,
    all_joint_a: Vector,

    all_v: &mut Vec<Vector>,
    all_a: &mut Vec<Vector>,

    limi_translations: &[Vector],
    limi_rotations: &mut Vec<Matrix>,

    omi_translations: &mut Vec<Vector>,
    omi_rotations: &mut Vec<Matrix>,

    joint_index: usize,

    robot_info: &RobotInfo,
) {
    let rotation_matrix = Matrix!([
        qcos.at(joint_index), -qsin.at(joint_index), Scalar!(0.0);
        qsin.at(joint_index), qcos.at(joint_index), Scalar!(0.0);
        Scalar!(0.0), Scalar!(0.0), Scalar!(1.0)
    ])
    .define(format!("limi_rotation_{}", joint_index));

    let limi_rotation = (robot_info.calc_limi)(rotation_matrix, joint_index);

    limi_rotations.push(limi_rotation.clone());
    let limi_translation = limi_translations[joint_index].clone();

    match joint_index {
        0 => {
            omi_rotations.push(limi_rotation.clone());
            omi_translations.push(limi_translation.clone());
            all_v.push(Vector!([
                Scalar!(0.0),
                Scalar!(0.0),
                Scalar!(0.0),
                Scalar!(0.0),
                Scalar!(0.0),
                all_joint_v.at(joint_index);
            ]));
        }
        _ => {
            // the multiplication between oMi and liMi is defined as:
            //{ return SE3Tpl(rot*m2.rotation()
            //    ,translation()+rotation()*m2.translation());}
            let omi_rotation_i = omi_rotations[joint_index - 1].matmul(&limi_rotation);
            let omi_translation_to_add =
                &omi_rotations[joint_index - 1].matmul_vec(&limi_translation);
            let omi_translation_i = &omi_translations[joint_index - 1] + omi_translation_to_add;
            //oMis.push((omi_rotation_i.clone(), omi_translation_i.clone()));
            omi_rotations.push(omi_rotation_i);
            omi_translations.push(omi_translation_i);
            let temp_v = Vector!([
                Scalar!(0.0),
                Scalar!(0.0),
                Scalar!(0.0),
                Scalar!(0.0),
                Scalar!(0.0),
                all_joint_v.at(joint_index);
            ]);
            let new_v = act_motion_inv(
                limi_translation.clone(),
                limi_rotation.clone(),
                temp_v,
                all_v[joint_index - 1].clone(),
            );
            all_v.push(new_v);
        }
    }

    // data.a[i]  = jdata.S() * jmodel.jointVelocitySelector(a) + jdata.c() + (data.v[i] ^ jdata.v()) ;
    let minus_m_w = -&all_joint_v.at(joint_index);
    let temp_a1 = alpha_cross(minus_m_w, all_v[joint_index].clone());

    // jdata.S() * jmodel.jointVelocitySelector(a);
    let temp_a2 = &all_joint_a.at(joint_index) + &temp_a1.at(5);

    let temp_a3 = Vector!([
        temp_a1.at(0),
        temp_a1.at(1),
        temp_a1.at(2),
        temp_a1.at(3),
        temp_a1.at(4),
        temp_a2;
    ]);

    match joint_index {
        0 => {
            all_a.push(temp_a3);
        }
        _ => {
            let add_a = act_inv(
                limi_translation,
                limi_rotation,
                all_a[joint_index - 1].clone(),
            );
            let new_a = &temp_a3 + &add_a;
            all_a.push(new_a);
        }
    }
}

pub struct FKResult {
    pub omi_translations: Vec<Vector>,
    pub omi_rotations: Vec<Matrix>,
    pub all_v: Vec<Vector>,
    pub all_a: Vec<Vector>,
}

pub fn forward_kinematics(
    qcos: Vector,
    qsin: Vector,
    v: Vector,
    a: Vector,
    robot_info: &RobotInfo,
) -> FKResult {
    let n_joints = robot_info.n_joints;
    let limi_translations = robot_info.limi_translations.clone();

    let mut all_v: Vec<Vector> = Vec::new();
    let mut all_a: Vec<Vector> = Vec::new();
    let mut limi_rotations: Vec<Matrix> = Vec::new();
    let mut omi_translations: Vec<Vector> = Vec::new();
    let mut omi_rotations: Vec<Matrix> = Vec::new();

    for i in 0..n_joints {
        forward_kinematics_helper(
            qcos.clone(),
            qsin.clone(),
            v.clone(),
            a.clone(),
            &mut all_v,
            &mut all_a,
            &limi_translations,
            &mut limi_rotations,
            &mut omi_translations,
            &mut omi_rotations,
            i,
            robot_info,
        );
    }

    FKResult {
        omi_translations,
        omi_rotations,
        all_v,
        all_a,
    }
}
