use super::robots::robot_info::RobotInfo;
use crate::{Matrix, Scalar, Vector};

fn act_inv(
    translation: Vector,
    rotation: Matrix,
    linear: &Vector,
    angular: &Vector,
    linear_parent: Vector,
    angular_parent: Vector,
    joint_index: usize,
) -> (Vector, Vector) {
    let act_inv1 = translation
        .cross(&angular_parent)
        .define(format!("act_inv_fun_1_{}", joint_index));
    let act_inv2 = (&linear_parent - &act_inv1).define(format!("act_inv_fun_2_{}", joint_index));
    let act_inv3 = rotation
        .transpose()
        .define(format!("act_inv_fun_3_{}", joint_index));
    let act_inv4 =
        (act_inv3.matmul_vec(&act_inv2)).define(format!("act_inv_fun_4_{}", joint_index));
    let act_inv5 =
        (act_inv3.matmul_vec(&angular_parent)).define(format!("act_inv_fun_5_{}", joint_index));

    (linear + &act_inv4, angular + &act_inv5)
}

fn alpha_cross_linear(s: &Scalar, vin: &Vector) -> Vector {
    let alpha_cross1 = &(-s) * &vin.at(1);
    let alpha_cross2 = s * &vin.at(0);
    Vector!([alpha_cross1, alpha_cross2, Scalar!(0.0);])
}

fn alpha_cross_angular(s: &Scalar, vin: &Vector) -> Vector {
    let alpha_cross1 = &(-s) * &vin.at(1);
    let alpha_cross2 = s * &vin.at(0);
    Vector!([alpha_cross1, alpha_cross2, Scalar!(0.0);])
}

fn rhs_mult(inertia: &Matrix, vin: &Vector) -> Vector {
    let vout_0 = &inertia.at(0, 0) * &vin.at(0)
        + &inertia.at(0, 1) * &vin.at(1)
        + &inertia.at(0, 2) * &vin.at(2);
    let vout_1 = &inertia.at(0, 1) * &vin.at(0)
        + &inertia.at(1, 1) * &vin.at(1)
        + &inertia.at(1, 2) * &vin.at(2);
    let vout_2 = &inertia.at(0, 2) * &vin.at(0)
        + &inertia.at(1, 2) * &vin.at(1)
        + &inertia.at(2, 2) * &vin.at(2);

    //Vector!([vout_0, vout_1, vout_2;]).define(format!("rhs_mult_{}", joint_index))
    Vector!([vout_0, vout_1, vout_2;])
}

fn act(rotation: Matrix, translation: Vector, f: Vector) -> Vector {
    let f_linear = Vector!([f.at(0), f.at(1), f.at(2);]);
    let f_angular = Vector!([f.at(3), f.at(4), f.at(5);]);

    let new_f_linear = &rotation.matmul_vec(&f_linear);
    let new_f_angular = &rotation.matmul_vec(&f_angular);

    let f_angular_cross = translation.cross(new_f_linear);

    let new_f_angular = new_f_angular + f_angular_cross;

    Vector!([
        new_f_linear.at(0),
        new_f_linear.at(1),
        new_f_linear.at(2),
        new_f_angular.at(0),
        new_f_angular.at(1),
        new_f_angular.at(2);
    ])
}

fn first_pass(
    qsin: Scalar,
    qcos: Scalar,
    data_v: &Vector,
    v: &Vector,
    a: &Vector,
    parent_v: &Vector,
    parent_a_gf: &Vector,
    limi_translations: &[Vector],
    mut limi_rotations: Vec<Matrix>,
    joint_index: usize,
    levers: &[Vector],
    masses: &Vector,
    inertias: &[Matrix],
    robot_info: &RobotInfo,
) -> (Vec<Matrix>, Vector, Vector, Vector, Vector) {
    let rotation_matrix = Matrix!([
        qcos.clone(), -&qsin, Scalar!(0.0);
        qsin, qcos, Scalar!(0.0);
        Scalar!(0.0), Scalar!(0.0), Scalar!(1.0);
    ]);

    let limi_rotation = (robot_info.calc_limi)(rotation_matrix, joint_index);
    limi_rotations.push(limi_rotation.clone());
    let limi_translation = limi_translations[joint_index].clone();

    let mut new_v_linear = Vector!([
        data_v.at(0), data_v.at(1), data_v.at(2);
    ]);

    let mut new_v_angular = Vector!([
        data_v.at(3), data_v.at(4), v.at(joint_index);
    ]);

    let parent_v_linear = Vector!([
        parent_v.at(0), parent_v.at(1), parent_v.at(2);
    ]);
    let parent_v_angular = Vector!([
        parent_v.at(3), parent_v.at(4), parent_v.at(5);
    ]);

    let parent_a_gf_linear = Vector!([
        parent_a_gf.at(0), parent_a_gf.at(1), parent_a_gf.at(2);
    ]);
    let parent_a_gf_angular = Vector!([
        parent_a_gf.at(3), parent_a_gf.at(4), parent_a_gf.at(5);
    ]);

    // if parent > 0
    if joint_index > 0 {
        //data.v[i] += data.liMi[i].actInv(data.v[parent]);
        (new_v_linear, new_v_angular) = act_inv(
            limi_translation.clone(),
            limi_rotation.clone(),
            &new_v_linear,
            &new_v_angular,
            parent_v_linear,
            parent_v_angular,
            joint_index,
        );
    }
    new_v_linear = new_v_linear.define(format!("new_v_linear_{}", joint_index));
    new_v_angular = new_v_angular.define(format!("new_v_angular_{}", joint_index));

    //data.a_gf[i] = jdata.c() + (data.v[i] ^ jdata.v());
    // ^ operator is actually implemented in pinocchio/include/pinocchio/spatial/cartesian-axis.hpp inline void CartesianAxis<2>::alphaCross
    // vout_[0] = -s*vin[1]; vout_[1] = s*vin[0]; vout_[2] = 0.;
    let minus_m_w = -&(v.at(joint_index));
    let alpha_cross_linear = alpha_cross_linear(&minus_m_w, &new_v_linear);
    let alpha_cross_angular = alpha_cross_angular(&minus_m_w, &new_v_angular);

    let new_a_gf = Vector!([
        alpha_cross_linear.at(0),
        alpha_cross_linear.at(1),
        alpha_cross_linear.at(2),
        alpha_cross_angular.at(0),
        alpha_cross_angular.at(1),
        alpha_cross_angular.at(2);
    ])
    .define(format!("new_a_gf_{}", joint_index));

    // data.a_gf[i] += jdata.S() * jmodel.jointVelocitySelector(a);
    // jointVelocitySelector(a) is only a[joint_id]
    // I couldn't print out info about jdata.S() easily but it is ConstraintRevoluteTpl, and I believe the only thing this line does is
    // data.a_gf[i][5] = jmodel.jointVelocitySelector(a)

    let new_a_gf_up1 = &a.at(joint_index) + &new_a_gf.at(5);

    let new_a_gf2_linear = Vector!([
        new_a_gf.at(0), new_a_gf.at(1), new_a_gf.at(2);
    ])
    .define(format!("new_a_gf2_linear_{}", joint_index));
    let new_a_gf2_angular = Vector!([
        new_a_gf.at(3), new_a_gf.at(4), new_a_gf_up1;
    ])
    .define(format!("new_a_gf2_angular_{}", joint_index));

    let (new_a_gf_up2_linear, new_a_gf_up2_angular) = act_inv(
        limi_translation.clone(),
        limi_rotation.clone(),
        &new_a_gf2_linear,
        &new_a_gf2_angular,
        parent_a_gf_linear,
        parent_a_gf_angular,
        joint_index,
    );

    let new_a_gf_up3 = Vector!([
        new_a_gf_up2_linear.at(0),
        new_a_gf_up2_linear.at(1),
        new_a_gf_up2_linear.at(2),
        new_a_gf_up2_angular.at(0),
        new_a_gf_up2_angular.at(1),
        new_a_gf_up2_angular.at(2);
    ])
    .define(format!("new_a_gf_up_final_{}", joint_index));

    // this line updates spatial momenta
    // model.inertias[i].__mult__(data.v[i],data.h[i]);
    //let data_h = Vector!(0.0, 0.0, 0.0, 0.0, 0.0, 0.0).define(format!("data_h_{}", joint_id));
    // data.v[i] is new_v at this point
    // firstly mass * (v.linear - lever.cross(v.angular))
    let h_linear_1 = &levers[joint_index]
        .cross(&new_v_angular)
        .define(format!("h_linear_1_{}", joint_index));
    let h_linear_2 = &new_v_linear - h_linear_1;
    let h_linear = &masses.at(joint_index) * &h_linear_2;

    // next line is Symmetric3::rhsMult(inertia(),v.angular(),f.angular());
    let h_angular = rhs_mult(&inertias[joint_index].clone(), &new_v_angular);

    // next line is f.angular() += lever().cross(f.linear());
    let h_angular_1 = levers[joint_index].cross(&h_linear);
    let h_angular_2 = &h_angular + &h_angular_1;

    // next line is model.inertias[i].__mult__(data.a_gf[i],data.f[i]);
    // firstly mass * (a_gf.linear - lever.cross(a_gf.angular))
    let f_linear_1 = levers[joint_index].cross(&new_a_gf_up2_angular);
    let f_linear_2 = &new_a_gf_up2_linear - &f_linear_1;
    let f_linear_3 = &masses.at(joint_index) * &f_linear_2;

    // next line is Symmetric3::rhsMult(inertia(),a_gf.angular(),f.angular());
    let f_angular = rhs_mult(&inertias[joint_index].clone(), &new_a_gf_up2_angular);

    // next line is f.angular() += lever().cross(f.linear());
    let f_angular_1 = levers[joint_index].cross(&f_linear_3);
    let f_angular_2 = &f_angular + &f_angular_1;

    // the cross here is not the regular cross product since the vectors are 6D
    // it is implemented in pinocchio/include/pinocchio/spatial/motion-dense.hpp cross_impl,
    // and it calls a motionAction, which is implemented in pinocchio/include/pinocchio/spatial/force-dense.hpp motionAction
    // final line is data.f[i] += data.v[i].cross(data.h[i]);
    /*
    void motionAction(const MotionDense<M1> & v, ForceDense<M2> & fout) const
    {
      std::cout << "ForceDense::motionAction" << std::endl;
      fout.linear().noalias() = v.angular().cross(linear());
      fout.angular().noalias() = v.angular().cross(angular())+v.linear().cross(linear());
    }
    */
    let f_linear_4_temp = &new_v_angular.cross(&h_linear);
    let f_linear_4 = &f_linear_3 + f_linear_4_temp;

    let f_angular_3_temp = &new_v_angular.cross(&h_angular_2);
    let f_angular_3 = &f_angular_2 + f_angular_3_temp;
    let f_angular_4_temp = &new_v_linear.cross(&h_linear);
    let f_angular_4 = &f_angular_3 + f_angular_4_temp;

    let h = Vector!([
        h_linear.at(0),
        h_linear.at(1),
        h_linear.at(2),
        h_angular_2.at(0),
        h_angular_2.at(1),
        h_angular_2.at(2);
    ])
    .define(format!("h_{}", joint_index));

    let f = Vector!([
        f_linear_4.at(0),
        f_linear_4.at(1),
        f_linear_4.at(2),
        f_angular_4.at(0),
        f_angular_4.at(1),
        f_angular_4.at(2);
    ])
    .define(format!("f_{}", joint_index));

    let new_v = Vector!([
        new_v_linear.at(0),
        new_v_linear.at(1),
        new_v_linear.at(2),
        new_v_angular.at(0),
        new_v_angular.at(1),
        new_v_angular.at(2);
    ])
    .define(format!("new_v_{}", joint_index));

    (limi_rotations, new_v, new_a_gf_up3, h, f)
}

fn sec_pass(
    mut all_f: Vec<Vector>,
    limi_rotations: Vec<Matrix>,
    limi_translations:&[Vector],
    n_joints: usize,
) -> (Vec<Vector>, Vector) {
    // jmodel.jointVelocitySelector(data.tau) = jdata.S().transpose()*data.f[i];
    // I again couldn't print out info about jdata.S(),
    // but at least for panda it works like this:
    // before data.tau:        0
    //     0
    //     0
    //     0
    // 0.381501
    // 0.471101
    // data.f[i]:
    //      linear = -39.5124  42.6572  22.4058
    //      angular = 4.46655 1.28701  5.7779
    //
    // after data.tau:        0
    //     0
    //     0
    // 5.7779
    // 0.381501
    // 0.471101
    // in each iteration, we set data.tau[joint_id] = data.f[i].angular[2];
    // the behaviour of S() ConstraintTpl was similar in forward pass

    let mut data_taus: Vec<Scalar> = vec![];

    for i in (0..n_joints).rev() {
        data_taus.push(all_f[i].at(5));

        //if(parent>0) data.f[parent] += data.liMi[i].act(data.f[i]);
        if i > 0 {
            let new_data_f_parent_add = act(
                limi_rotations[i].clone(),
                limi_translations[i].clone(),
                all_f[i].clone()
            );
            let new_data_f_parent = all_f[i - 1].clone() + new_data_f_parent_add;
            all_f[i - 1] = new_data_f_parent;
        }
    }

    let data_tau_refs: Vec<&Scalar> = data_taus.iter().rev().collect();
    let data_tau = Vector::from_scalars("final_tau", data_tau_refs);

    (all_f, data_tau)
}

#[allow(dead_code)]
pub fn rnea(qcos: Vector, qsin: Vector, v: Vector, a: Vector, robot_info: &RobotInfo) -> Vector {
    let n_joints = robot_info.n_joints;
    let limi_translations = robot_info.limi_translations.clone();
    let levers = robot_info.levers.clone();
    let masses = robot_info.masses.clone();
    let inertias = robot_info.inertias.clone();

    let mut limi_rotations: Vec<Matrix> = vec![];

    // we also have data.v, which v[0] is set to zero explicitly, and I assume the rest should also be zero
    // I will keep it constant here, too, but keep in mind
    let mut data_v: Vec<Vector> = vec![];

    let mut all_v: Vec<Vector> = vec![];
    let mut all_a_gf: Vec<Vector> = vec![];
    let mut all_h: Vec<Vector> = vec![];
    let mut all_f: Vec<Vector> = vec![];

    let parent_v = Vector!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).define("parent_v".to_string());
    let parent_a_gf = Vector!([0.0, 0.0, 9.81, 0.0, 0.0, 0.0]).define("parent_a_gf".to_string());

    let mut new_v: Vector;
    let mut new_a_gf: Vector;
    let mut new_h: Vector;
    let mut new_f: Vector;

    for i in 0..n_joints {
        data_v.push(Vector!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).define(format!("data_v_{}", i)));
        if i == 0 {
            (limi_rotations, new_v, new_a_gf, new_h, new_f) = first_pass(
                qsin.at(i).clone(),
                qcos.at(i).clone(),
                &data_v[i],
                &v,
                &a,
                &parent_v,
                &parent_a_gf,
                &limi_translations,
                limi_rotations,
                i,
                &levers,
                &masses,
                &inertias,
                robot_info,
            );
        } else {
            (limi_rotations, new_v, new_a_gf, new_h, new_f) = first_pass(
                qsin.at(i).clone(),
                qcos.at(i).clone(),
                &data_v[i],
                &v,
                &a,
                &all_v[i - 1],
                &all_a_gf[i - 1],
                &limi_translations,
                limi_rotations,
                i,
                &levers,
                &masses,
                &inertias,
                robot_info,
            );
        }

        all_v.push(new_v.clone());
        all_a_gf.push(new_a_gf.clone());
        all_h.push(new_h.clone());
        all_f.push(new_f.clone());
    }

    // sec_pass will do its own iteration
    let (_new_f, taus) = sec_pass(all_f, limi_rotations, &limi_translations, n_joints);

    taus
}
