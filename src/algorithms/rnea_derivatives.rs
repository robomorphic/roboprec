use super::robots::robot_info::RobotInfo;
use roboprec::{Matrix, Scalar, Vector};

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

// If I am not mistaken, skew(vec3d) gives
//
//    M_(0,0) = Scalar(0);  M_(0,1) = -v[2];      M_(0,2) = v[1];
//    M_(1,0) = v[2];       M_(1,1) = Scalar(0);  M_(1,2) = -v[0];
//    M_(2,0) = -v[1];      M_(2,1) = v[0];       M_(2,2) = Scalar(0);
fn skew_vec3d(vec: &Vector) -> Matrix {
    Matrix!([
        Scalar!(0.0), -&vec.at(2), vec.at(1);
        vec.at(2), Scalar!(0.0), -&vec.at(0);
        -&vec.at(1), vec.at(0), Scalar!(0.0);
    ])
}

//inline void skewSquare(const Eigen::MatrixBase<V1> & u,
//                       const Eigen::MatrixBase<V2> & v,
//                       const Eigen::MatrixBase<Matrix3> & C)
//{
//  C_.noalias() = v*u.transpose();
//  const Scalar udotv(u.dot(v));
//  C_.diagonal().array() -= udotv;
//}
fn skew_square(u: &Vector, v: &Vector) -> Matrix {
    let c1 = Matrix!([
        &v.at(0) * &u.at(0), &v.at(0) * &u.at(1), &v.at(0) * &u.at(2);
        &v.at(1) * &u.at(0), &v.at(1) * &u.at(1), &v.at(1) * &u.at(2);
        &v.at(2) * &u.at(0), &v.at(2) * &u.at(1), &v.at(2) * &u.at(2)
    ]);

    let udotv = u.dot(v);

    let c2 = Matrix!([
        udotv.clone(), Scalar!(0.0), Scalar!(0.0);
        Scalar!(0.0), udotv.clone(), Scalar!(0.0);
        Scalar!(0.0), Scalar!(0.0), udotv.clone()
    ]);

    &c1 - &c2
}

// alphaskewsquare here:
// operator Symmetric3Tpl () const
// {
// const Scalar & x = v[0], & y = v[1], & z = v[2];
// return Symmetric3Tpl(-m*(y*y+z*z),
//                         m* x*y,-m*(x*x+z*z),
//                         m* x*z,m* y*z,-m*(x*x+y*y));
// }
fn alpha_skew_square(m: &Scalar, v: &Vector) -> Matrix {
    let x = &v.at(0);
    let y = &v.at(1);
    let z = &v.at(2);

    Matrix!([
        &-m * &(y * y + z * z), m * &(x * y), m * &(x * z);
        m * &(x * y), &-m * &(x * x + z * z), m * &(y * z);
        m * &(x * z), m * &(y * z), &-m * &(x * x + y * y)
    ])
}

// rotation is inertia(), and translation is lever()
fn inertia_variation(
    rotation: &Matrix,
    translation: &Vector,
    linear: &Vector,
    angular: &Vector,
    mass: Scalar,
) -> Matrix {
    let mv_linear = &mass * linear;
    let mv_angular = &mass * angular;

    // res.template block<3,3>(LINEAR,ANGULAR) = -skew(mv.linear()) - skewSquare(mv.angular(),lever()) + skewSquare(lever(),mv.angular());
    let skew_first_first = skew_vec3d(&mv_linear);
    let skew_first_second = skew_square(&mv_angular, translation);
    let skew_first_third = skew_square(translation, &mv_angular);
    let res_first = &(&skew_first_third - &skew_first_second) - &skew_first_first;

    //      res.template block<3,3>(ANGULAR,LINEAR) = res.template block<3,3>(LINEAR,ANGULAR).transpose();
    // passing this for now

    //      res.template block<3,3>(ANGULAR,ANGULAR) = -skewSquare(mv.linear(),lever()) - skewSquare(lever(),mv.linear());
    let skew_second_first = skew_square(&mv_linear, translation);
    let skew_second_second = skew_square(translation, &mv_linear);
    let res_second = &-skew_second_first - &skew_second_second;

    //      res.template block<3,3>(LINEAR,LINEAR) = (inertia() - AlphaSkewSquare(mass(),lever())).matrix();
    let res_third = rotation - &alpha_skew_square(&mass, translation);

    // res.template block<3,3>(ANGULAR,ANGULAR) -= res.template block<3,3>(LINEAR,LINEAR) * skew(v.angular());
    let res_fourth = &res_third.matmul(&skew_vec3d(angular));

    let res_fifth = res_second - res_fourth;

    // res.template block<3,3>(ANGULAR,ANGULAR) += cross(v.angular(),res.template block<3,3>(LINEAR,LINEAR));
    // cross here applies the cross product onto the columns of M.
    let res_third_first_col = Vector!([
        res_third.at(0, 0),
        res_third.at(1, 0),
        res_third.at(2, 0);
    ]);
    let res_third_second_col = Vector!([
        res_third.at(0, 1),
        res_third.at(1, 1),
        res_third.at(2, 1);
    ]);
    let res_third_third_col = Vector!([
        res_third.at(0, 2),
        res_third.at(1, 2),
        res_third.at(2, 2);
    ]);
    let res_sixth_first_col = angular.cross(&res_third_first_col);
    let res_sixth_second_col = angular.cross(&res_third_second_col);
    let res_sixth_third_col = angular.cross(&res_third_third_col);

    let res_sixth = Matrix!([
        res_fifth.at(0, 0) + res_sixth_first_col.at(0), res_fifth.at(0, 1) + res_sixth_second_col.at(0), res_fifth.at(0, 2) + res_sixth_third_col.at(0);
        res_fifth.at(1, 0) + res_sixth_first_col.at(1), res_fifth.at(1, 1) + res_sixth_second_col.at(1), res_fifth.at(1, 2) + res_sixth_third_col.at(1);
        res_fifth.at(2, 0) + res_sixth_first_col.at(2), res_fifth.at(2, 1) + res_sixth_second_col.at(2), res_fifth.at(2, 2) + res_sixth_third_col.at(2);
    ]);

    // linear parts are 0
    // the third part of the matrix is res_sixth

    // the first and second parts are the same, res_first
    Matrix!([
        Scalar!(0.0), Scalar!(0.0), Scalar!(0.0), res_first.at(0, 0), res_first.at(0, 1), res_first.at(0, 2);
        Scalar!(0.0), Scalar!(0.0), Scalar!(0.0), res_first.at(1, 0), res_first.at(1, 1), res_first.at(1, 2);
        Scalar!(0.0), Scalar!(0.0), Scalar!(0.0), res_first.at(2, 0), res_first.at(2, 1), res_first.at(2, 2);
        res_first.at(0, 0), res_first.at(1, 0), res_first.at(2, 0), res_sixth.at(0, 0), res_sixth.at(0, 1), res_sixth.at(0, 2);
        res_first.at(0, 1), res_first.at(1, 1), res_first.at(2, 1), res_sixth.at(1, 0), res_sixth.at(1, 1), res_sixth.at(1, 2);
        res_first.at(0, 2), res_first.at(1, 2), res_first.at(2, 2), res_sixth.at(2, 0), res_sixth.at(2, 1), res_sixth.at(2, 2);
    ])
}

// motionSet::motion_action
//void motion_action(const MotionDense<M1> & v, MotionDense<M2> & mout) const
//{
//    std::cout << "MotionDense::motion_action 1" << std::endl;
//    mout.linear() = v.linear().cross(angular())+v.angular().cross(linear());
//    mout.angular() = v.angular().cross(angular());
//}
fn motion_action(
    v_linear: &Vector,
    v_angular: &Vector,
    linear: &Vector,
    angular: &Vector,
) -> (Vector, Vector) {
    let mout_linear_cross_angular = v_linear.cross(angular);
    let mout_angular_cross_linear = v_angular.cross(linear);
    let mout_linear = &mout_linear_cross_angular + &mout_angular_cross_linear;
    let mout_angular = v_angular.cross(angular);

    (mout_linear, mout_angular)
}

// the cross here is not the regular cross product since the vectors are 6D
// it is implemented in pinocchio/include/pinocchio/spatial/motion-dense.hpp cross_impl,
// and it calls a motion_action, which is implemented in pinocchio/include/pinocchio/spatial/force-dense.hpp motion_action
// final line is data.f[i] += data.v[i].cross(data.h[i]);
/*
void motion_action(const MotionDense<M1> & v, ForceDense<M2> & fout) const
{
    std::cout << "ForceDense::motion_action" << std::endl;
    fout.linear().noalias() = v.angular().cross(linear());
    fout.angular().noalias() = v.angular().cross(angular())+v.linear().cross(linear());
}
*/
fn cross_6d(v1: &Vector, v2: &Vector) -> Vector {
    let v1_linear = Vector!([v1.at(0), v1.at(1), v1.at(2);]);
    let v1_angular = Vector!([v1.at(3), v1.at(4), v1.at(5);]);

    let v2_linear = Vector!([v2.at(0), v2.at(1), v2.at(2);]);
    let v2_angular = Vector!([v2.at(3), v2.at(4), v2.at(5);]);

    let res_linear = v1_angular.cross(&v2_linear);
    let res_angular_1 = v1_angular.cross(&v2_angular);
    let res_angular_2 = v1_linear.cross(&v2_linear);
    let res_angular = &res_angular_1 + &res_angular_2;

    Vector!([
        res_linear.at(0),
        res_linear.at(1),
        res_linear.at(2),
        res_angular.at(0),
        res_angular.at(1),
        res_angular.at(2);
    ])
}

//  ///
//  /// \brief Add skew matrix represented by a 3d vector to a given matrix,
//  ///        i.e. add the antisymmetric matrix representation of the cross product operator (\f$ [v]_{\times} x = v \times x \f$)
//  ///
//  /// \param[in]  v a vector of dimension 3.
//  /// \param[out] M the 3x3 matrix to which the skew matrix is added.
//  ///
//  template <typename Vector3Like, typename Matrix3Like>
//  inline void addSkew(const Eigen::MatrixBase<Vector3Like> & v,
//                      const Eigen::MatrixBase<Matrix3Like> & M)
//  {
//    EIGEN_STATIC_ASSERT_VECTOR_SPECIFIC_SIZE(Vector3Like,3);
//    PINOCCHIO_ASSERT_MATRIX_SPECIFIC_SIZE(Matrix3Like,M,3,3);
//
//    Matrix3Like & M_ = PINOCCHIO_EIGEN_CONST_CAST(Matrix3Like,M);
//
//                          M_(0,1) -= v[2];      M_(0,2) += v[1];
//    M_(1,0) += v[2];                            M_(1,2) -= v[0];
//    M_(2,0) -= v[1];      M_(2,1) += v[0];                     ;
//  }
fn add_skew(m: &Matrix, v: &Vector) -> Matrix {
    Matrix!([
        m.at(0, 0), &m.at(0, 1) - &v.at(2), m.at(0, 2) + v.at(1);
        m.at(1, 0) + v.at(2), m.at(1, 1), &m.at(1, 2) - &v.at(0);
        &m.at(2, 0) - &v.at(1), m.at(2, 1) + v.at(0), m.at(2, 2)
    ])
}

/// template<typename ForceDerived, typename M6>
/// static void addForceCrossMatrix(const ForceDense<ForceDerived> & f,
///                                 const Eigen::MatrixBase<M6> & mout)
/// {
///   M6 & mout_ = PINOCCHIO_EIGEN_CONST_CAST(M6,mout);
///   addSkew(-f.linear(),mout_.template block<3,3>(ForceDerived::LINEAR,ForceDerived::ANGULAR));
///   addSkew(-f.linear(),mout_.template block<3,3>(ForceDerived::ANGULAR,ForceDerived::LINEAR));
///   addSkew(-f.angular(),mout_.template block<3,3>(ForceDerived::ANGULAR,ForceDerived::ANGULAR));
/// }
fn add_force_cross_matrix(f_linear: Vector, f_angular: Vector, mout: Matrix) -> Matrix {
    let linear_linear = Matrix!([
        mout.at(0, 0), mout.at(0, 1), mout.at(0, 2);
        mout.at(1, 0), mout.at(1, 1), mout.at(1, 2);
        mout.at(2, 0), mout.at(2, 1), mout.at(2, 2)
    ]);
    let linear_angular = Matrix!([
        mout.at(0, 3), mout.at(0, 4), mout.at(0, 5);
        mout.at(1, 3), mout.at(1, 4), mout.at(1, 5);
        mout.at(2, 3), mout.at(2, 4), mout.at(2, 5)
    ]);
    let angular_linear = Matrix!([
        mout.at(3, 0), mout.at(3, 1), mout.at(3, 2);
        mout.at(4, 0), mout.at(4, 1), mout.at(4, 2);
        mout.at(5, 0), mout.at(5, 1), mout.at(5, 2)
    ]);
    let angular_angular = Matrix!([
        mout.at(3, 3), mout.at(3, 4), mout.at(3, 5);
        mout.at(4, 3), mout.at(4, 4), mout.at(4, 5);
        mout.at(5, 3), mout.at(5, 4), mout.at(5, 5)
    ]);

    // addSkew(-f.linear(),mout_.template block<3,3>(ForceDerived::LINEAR,ForceDerived::ANGULAR));
    let res_linear_angular = add_skew(&linear_angular, &-&f_linear);
    // addSkew(-f.linear(),mout_.template block<3,3>(ForceDerived::ANGULAR,ForceDerived::LINEAR));
    let res_angular_linear = add_skew(&angular_linear, &-&f_linear);
    // addSkew(-f.angular(),mout_.template block<3,3>(ForceDerived::ANGULAR,ForceDerived::ANGULAR));
    let res_angular_angular = add_skew(&angular_angular, &-&f_angular);

    Matrix!([
        linear_linear.at(0, 0), linear_linear.at(0, 1), linear_linear.at(0, 2), res_linear_angular.at(0, 0), res_linear_angular.at(0, 1), res_linear_angular.at(0, 2);
        linear_linear.at(1, 0), linear_linear.at(1, 1), linear_linear.at(1, 2), res_linear_angular.at(1, 0), res_linear_angular.at(1, 1), res_linear_angular.at(1, 2);
        linear_linear.at(2, 0), linear_linear.at(2, 1), linear_linear.at(2, 2), res_linear_angular.at(2, 0), res_linear_angular.at(2, 1), res_linear_angular.at(2, 2);
        res_angular_linear.at(0, 0), res_angular_linear.at(0, 1), res_angular_linear.at(0, 2), res_angular_angular.at(0, 0), res_angular_angular.at(0, 1), res_angular_angular.at(0, 2);
        res_angular_linear.at(1, 0), res_angular_linear.at(1, 1), res_angular_linear.at(1, 2), res_angular_angular.at(1, 0), res_angular_angular.at(1, 1), res_angular_angular.at(1, 2);
        res_angular_linear.at(2, 0), res_angular_linear.at(2, 1), res_angular_linear.at(2, 2), res_angular_angular.at(2, 0), res_angular_angular.at(2, 1), res_angular_angular.at(2, 2);
    ])
}

// it uses a variable "axis", and it is always 2 for some reason.
//template<typename S1, int O1>
//typename SE3GroupAction<ConstraintRevoluteTpl>::ReturnType
//se3Action(const SE3Tpl<S1,O1> & m) const
//{
//  typedef typename SE3GroupAction<ConstraintRevoluteTpl>::ReturnType ReturnType;
//  ReturnType res;
//  res.template segment<3>(LINEAR) = m.translation().cross(m.rotation().col(axis));
//  res.template segment<3>(ANGULAR) = m.rotation().col(axis);
//  std::cout << "res: " << res << std::endl;
//  return res;
//}
fn act_constraint(rotation: &Matrix, translation: &Vector, joint_axis: usize) -> (Vector, Vector) {
    let rotation_col = Vector!([
        rotation.at(0, joint_axis),
        rotation.at(1, joint_axis),
        rotation.at(2, joint_axis);
    ]);

    let linear = translation.cross(&rotation_col);

    (linear, rotation_col)
}

// not to be confused with motion_action.
//      v.angular().noalias() = m.rotation()*angular();
//      v.linear().noalias() = m.rotation()*linear() + m.translation().cross(v.angular());
fn act_motion1(
    rotation: &Matrix,
    translation: &Vector,
    linear: &Vector,
    angular: &Vector,
) -> Vector {
    let rotation_cross_linear = rotation.matmul_vec(linear);
    let rotation_cross_angular = rotation.matmul_vec(angular);

    let res_angular = rotation_cross_angular;

    let cross = translation.cross(&res_angular);
    let res_linear = &rotation_cross_linear + &cross;

    Vector!([
        res_linear.at(0),
        res_linear.at(1),
        res_linear.at(2),
        res_angular.at(0),
        res_angular.at(1),
        res_angular.at(2);
    ])
}

// not to be confused with motion_action.
// in C++, pinocchio/include/pinocchio/spatial/motion-dense.hpp:
//      v.angular().noalias() = m.rotation()*angular();
//      v.linear().noalias() = m.rotation()*linear() + m.translation().cross(v.angular());
fn act_motion(rotation: &Matrix, translation: &Vector, t: &Vector) -> Vector {
    let linear = Vector!([t.at(0), t.at(1), t.at(2);]);
    let angular = Vector!([t.at(3), t.at(4), t.at(5);]);

    let rotation_cross_linear = rotation.matmul_vec(&linear);
    let rotation_cross_angular = rotation.matmul_vec(&angular);

    let res_angular = rotation_cross_angular;

    let cross = translation.cross(&res_angular);
    let res_linear = &rotation_cross_linear + &cross;

    Vector!([
        res_linear.at(0),
        res_linear.at(1),
        res_linear.at(2),
        res_angular.at(0),
        res_angular.at(1),
        res_angular.at(2);
    ])
}

// I strongly disagree with this function's name, but it is actInv in pinocchio,
// so I will leave it as is for now
fn act_motion_inv(
    translation: &Vector,
    rotation: &Matrix,
    linear: &Vector,
    angular: &Vector,
    linear_parent: &Vector,
    angular_parent: &Vector,
) -> (Vector, Vector) {
    let act_inv1 = translation.cross(angular_parent);
    let act_inv2 = linear_parent - &act_inv1;
    let act_inv3 = rotation.transpose();
    let act_inv4 = &act_inv3.matmul_vec(&act_inv2);
    let new_linear = linear + act_inv4;
    let act_inv5 = &act_inv3.matmul_vec(angular_parent);
    let new_angular = angular + act_inv5;

    (new_linear, new_angular)
}

// res(0,0) = m_data(0); res(0,1) = m_data(1); res(0,2) = m_data(3);
// res(1,0) = m_data(1); res(1,1) = m_data(2); res(1,2) = m_data(4);
// res(2,0) = m_data(3); res(2,1) = m_data(4); res(2,2) = m_data(5);
/////Matrix32  decomposeltI() const
/////    {
/////      Matrix32 L;
/////      L <<
/////      m_data(0) - m_data(5),    m_data(1),
/////      m_data(1),              m_data(2) - m_data(5),
/////      2*m_data(3),            m_data(4) + m_data(4);
/////      return L;
/////    }
/// S:   0.70337 -0.000139  0.006772
/// -0.000139   0.70661  0.019169
/// 0.006772  0.019169  0.009117
/// L:  0.694253 -0.000139
/// -0.000139  0.697493
/// 0.013544  0.038338
fn decompose_it_i(i: &Matrix) -> Matrix {
    Matrix!([
        &i.at(0, 0) - &i.at(2, 2), i.at(0, 1);
        i.at(0, 1), &i.at(1, 1) - &i.at(2, 2);
        &Scalar!(2.0) * &i.at(0, 2), i.at(1, 2) + i.at(1, 2)
    ])
}

// this is implemented in
// include/pinocchio/spatial/symmetric3.hpp, Symmetric3::rotate
//Symmetric3Tpl rotate(const Eigen::MatrixBase<D> & R) const
//{
//    EIGEN_STATIC_ASSERT_MATRIX_SPECIFIC_SIZE(D,3,3);
//    assert(isUnitary(R.transpose()*R) && "R is not a Unitary matrix");
//
//    Symmetric3Tpl Sres;
//
//    // 4 a
//    const Matrix32 L( decomposeltI() );
//
//    // Y = R' L   ===> (12 m + 8 a)
//    const Matrix2 Y( R.template block<2,3>(1,0) * L );
//
//    // Sres= Y R  ===> (16 m + 8a)
//    Sres.m_data(1) = Y(0,0)*R(0,0) + Y(0,1)*R(0,1);
//    Sres.m_data(2) = Y(0,0)*R(1,0) + Y(0,1)*R(1,1);
//    Sres.m_data(3) = Y(1,0)*R(0,0) + Y(1,1)*R(0,1);
//    Sres.m_data(4) = Y(1,0)*R(1,0) + Y(1,1)*R(1,1);
//    Sres.m_data(5) = Y(1,0)*R(2,0) + Y(1,1)*R(2,1);
//
//    // r=R' v ( 6m + 3a)
//    const Vector3 r(-R(0,0)*m_data(4) + R(0,1)*m_data(3),
//                    -R(1,0)*m_data(4) + R(1,1)*m_data(3),
//                    -R(2,0)*m_data(4) + R(2,1)*m_data(3));
//
//    // Sres_11 (3a)
//    Sres.m_data(0) = L(0,0) + L(1,1) - Sres.m_data(2) - Sres.m_data(5);
//
//    // Sres + D + (Ev)x ( 9a)
//    Sres.m_data(0) += m_data(5);
//    Sres.m_data(1) += r(2); Sres.m_data(2)+= m_data(5);
//    Sres.m_data(3) +=-r(1); Sres.m_data(4)+= r(0); Sres.m_data(5) += m_data(5);
//
//    return Sres;
//}
fn symmetric3_rotate(s: &Matrix, r: &Matrix) -> Matrix {
    let l = decompose_it_i(s);

    // const Matrix2 Y( R.template block<2,3>(1,0) * L );
    // R.template block<2,3>(1,0) takes the bottom two rows of R
    let bottom_r = Matrix!([
        r.at(1, 0), r.at(1, 1), r.at(1, 2);
        r.at(2, 0), r.at(2, 1), r.at(2, 2)
    ]);

    let y = &bottom_r.matmul(&l);

    let sres_first_1 = (&y.at(0, 0) * &r.at(0, 0)) + (&y.at(0, 1) * &r.at(0, 1));
    let sres_first_2 = (&y.at(0, 0) * &r.at(1, 0)) + (&y.at(0, 1) * &r.at(1, 1));
    let sres_first_3 = (&y.at(1, 0) * &r.at(0, 0)) + (&y.at(1, 1) * &r.at(0, 1));
    let sres_first_4 = (&y.at(1, 0) * &r.at(1, 0)) + (&y.at(1, 1) * &r.at(1, 1));
    let sres_first_5 = (&y.at(1, 0) * &r.at(2, 0)) + (&y.at(1, 1) * &r.at(2, 1));

    //    const Vector3 r(-R(0,0)*m_data(4) + R(0,1)*m_data(3),
    //                    -R(1,0)*m_data(4) + R(1,1)*m_data(3),
    //                    -R(2,0)*m_data(4) + R(2,1)*m_data(3));
    let const_r = Vector!([
        -r.at(0,0) * &s.at(1, 2) + &r.at(0,1) * &s.at(0, 2),
        -r.at(1,0) * &s.at(1, 2) + &r.at(1,1) * &s.at(0, 2),
        -r.at(2,0) * &s.at(1, 2) + &r.at(2,1) * &s.at(0, 2);
    ]);

    //    Sres.m_data(0) = L(0,0) + L(1,1) - Sres.m_data(2) - Sres.m_data(5);
    //
    //    // Sres + D + (Ev)x ( 9a)
    //    Sres.m_data(0) += m_data(5);
    //    Sres.m_data(1) += r(2); Sres.m_data(2)+= m_data(5);
    //    Sres.m_data(3) +=-r(1); Sres.m_data(4)+= r(0); Sres.m_data(5) += m_data(5);

    // let sres_update_0 = l.at_mat(0, 0) + l.at_mat(1, 1) - sres_first_2.clone() - sres_first_5.clone();
    let sres_update_0_tmp1 = l.at(0, 0) + l.at(1, 1);
    let sres_update_0_tmp2 = &sres_first_2 + &sres_first_5;
    let sres_update_0 = &sres_update_0_tmp1 - &sres_update_0_tmp2;

    let sres_final_0 = sres_update_0 + s.at(2, 2);
    let sres_final_1 = &sres_first_1 + &const_r.at(2);
    let sres_final_2 = &sres_first_2 + &s.at(2, 2);
    let sres_final_3 = &sres_first_3 - &const_r.at(1);
    let sres_final_4 = &sres_first_4 + &const_r.at(0);
    let sres_final_5 = &sres_first_5 + &s.at(2, 2);

    // res(0,0) = m_data(0); res(0,1) = m_data(1); res(0,2) = m_data(3);
    // res(1,0) = m_data(1); res(1,1) = m_data(2); res(1,2) = m_data(4);
    // res(2,0) = m_data(3); res(2,1) = m_data(4); res(2,2) = m_data(5);
    Matrix!([
        sres_final_0, sres_final_1.clone(), sres_final_3.clone();
        sres_final_1, sres_final_2, sres_final_4.clone();
        sres_final_3, sres_final_4, sres_final_5
    ])
}

// in oMi.act(model.inertias[i]),
// model.inertias[i].se3Action_impl(oMi) is called
//InertiaTpl se3Action_impl(const SE3 & M) const
//    {
//      /* The multiplication RIR' has a particular form that could be used, however it
//       * does not seems to be more efficient, see http://stackoverflow.m_comom/questions/
//       * 13215467/eigen-best-way-to-evaluate-asa-transpose-and-store-the-result-in-a-symmetric .*/
//       //std::cout << "InertiaTpl::se3Action_impl" << std::endl;
//       return InertiaTpl(mass(),
//                         M.translation()+M.rotation()*lever(),
//                         inertia().rotate(M.rotation()));
//     }
fn act_inertia(
    rotation: &Matrix,
    translation: &Vector,
    lever: &Vector,
    inertia: Matrix,
) -> (Vector, Matrix) {
    let new_translation_temp = rotation.matmul_vec(lever);
    let new_translation = translation + &new_translation_temp;
    let new_rotation = symmetric3_rotate(&inertia, rotation);

    (new_translation, new_rotation)
}

// inertia * vector
//data.oh[i] = data.oYcrb[i] * ov;
// mult implemented as this
//f.linear().noalias() = mass()*(v.linear() - lever().cross(v.angular()));
//Symmetric3::rhsMult(inertia(),v.angular(),f.angular());
//f.angular() += lever().cross(f.linear());
fn inertia_vec_mult(
    inertia_mass: Scalar,
    inertia_lever: &Vector,
    inertia_inertia: &Matrix,
    v: &Vector,
) -> Vector {
    let v_linear = Vector!([v.at(0), v.at(1), v.at(2);]);
    let v_angular = Vector!([v.at(3), v.at(4), v.at(5);]);

    let data_oh_linear_temp1 = inertia_lever.cross(&v_angular);
    let data_oh_linear_temp2 = &v_linear - &data_oh_linear_temp1;
    let data_oh_linear = &inertia_mass * &data_oh_linear_temp2;

    // last value of data_oh_angular_temp is wrong
    let data_oh_angular_temp = rhs_mult(inertia_inertia, &v_angular);
    let data_oh_angular_temp2 = inertia_lever.cross(&data_oh_linear);
    let data_oh_angular = &data_oh_angular_temp + &data_oh_angular_temp2;

    Vector!([
        data_oh_linear.at(0),
        data_oh_linear.at(1),
        data_oh_linear.at(2),
        data_oh_angular.at(0),
        data_oh_angular.at(1),
        data_oh_angular.at(2);
    ])
}

fn first_pass(
    qsin: Scalar,
    qcos: Scalar,
    data_v: &Vector,
    v: &Vector,
    a: &Vector,
    all_v: &mut Vec<Vector>,
    omis: &mut Vec<(Matrix, Vector)>,
    limi_translations: &[Vector],
    limi_rotations: &mut Vec<Matrix>,
    joint_index: usize,
    levers: &[Vector],
    masses: &[Scalar],
    inertias: &[Matrix],
    all_of: &mut Vec<Vector>,
    all_oh: &mut Vec<Vector>,
    all_doycrb: &mut Vec<Matrix>,
    all_ov: &mut Vec<Vector>,
    all_oa: &mut Vec<Vector>,
    all_oa_gf: &mut Vec<Vector>,
    all_a: &mut Vec<Vector>,
    all_oycrb: &mut Vec<(Matrix, Vector)>,
    j_cols: &mut Vec<Vector>,
    dj_cols: &mut Vec<Vector>,
    dadq_cols: &mut Vec<Vector>,
    dvdq_cols: &mut Vec<Vector>,
    dadv_cols: &mut Vec<Vector>,
    robot_info: &RobotInfo,
) {
    let rotation_matrix = Matrix!([
        qcos.clone(), -&qsin, Scalar!(0.0);
        qsin, qcos, Scalar!(0.0);
        Scalar!(0.0), Scalar!(0.0), Scalar!(1.0)
    ]);

    let limi_rotation = (robot_info.calc_limi)(rotation_matrix.clone(), joint_index);
    limi_rotations.push(limi_rotation.clone());
    let limi_translation = limi_translations[joint_index].clone();

    let mut new_v_linear = Vector!([
        data_v.at(0), data_v.at(1), data_v.at(2);
    ]);

    let mut new_v_angular = Vector!([
        data_v.at(3), data_v.at(4), v.at(joint_index);
    ]);

    let parent_v = match joint_index {
        0 => Vector!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        _ => all_v[joint_index - 1].clone(),
    };

    let parent_v_linear = Vector!([
        parent_v.at(0), parent_v.at(1), parent_v.at(2);
    ]);
    let parent_v_angular = Vector!([
        parent_v.at(3), parent_v.at(4), parent_v.at(5);
    ]);

    let parent_a = match joint_index {
        0 => Vector!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        _ => all_a[joint_index - 1].clone(),
    };

    let parent_a_linear = Vector!([
        parent_a.at(0), parent_a.at(1), parent_a.at(2);
    ]);
    let parent_a_angular = Vector!([
        parent_a.at(3), parent_a.at(4), parent_a.at(5);
    ]);

    //if(parent > 0)
    //  {
    //    data.oMi[i] = data.oMi[parent] * data.liMi[i];
    //    data.v[i] += data.liMi[i].actInv(data.v[parent]);
    //  }
    //  else
    //    data.oMi[i] = data.liMi[i];
    match joint_index {
        0 => omis.push((limi_rotation.clone(), limi_translation.clone())),
        _ => {
            // the multiplication between oMi and liMi is defined as:
            //{ return SE3Tpl(rot*m2.rotation()
            //    ,translation()+rotation()*m2.translation());}
            let omi_rotation_i = &omis[joint_index - 1].0.matmul(&limi_rotation);
            let omi_translation_to_add = &omis[joint_index - 1].0.matmul_vec(&limi_translation);
            let omi_translation_i = omis[joint_index - 1].1.clone() + omi_translation_to_add;
            omis.push((omi_rotation_i.clone(), omi_translation_i));
            (new_v_linear, new_v_angular) = act_motion_inv(
                &limi_translation,
                &limi_rotation,
                &new_v_linear,
                &new_v_angular,
                &parent_v_linear,
                &parent_v_angular,
            )
        }
    }

    // as the calculation of data.v[i] is done here, we can push it to all_v
    all_v.push(Vector!([
        new_v_linear.at(0),
        new_v_linear.at(1),
        new_v_linear.at(2),
        new_v_angular.at(0),
        new_v_angular.at(1),
        new_v_angular.at(2);
    ]));

    //omis.push(omis[joint_id - 1].clone() * limi_rotation.clone());

    // The below portion is the same with RNEA, because in RNEA derivative it is like this:
    //data.a[i] = jdata.S() * jmodel.jointVelocitySelector(a) + jdata.c() + (data.v[i] ^ jdata.v());
    //data.a_gf[i] = jdata.c() + (data.v[i] ^ jdata.v());
    // ^ operator is actually implemented in pinocchio/include/pinocchio/spatial/cartesian-axis.hpp inline void CartesianAxis<2>::alphaCross
    // vout_[0] = -s*vin[1]; vout_[1] = s*vin[0]; vout_[2] = 0.;
    let minus_m_w = -&(v.at(joint_index));
    let alpha_cross_linear = alpha_cross_linear(&minus_m_w, &new_v_linear);
    let alpha_cross_angular = alpha_cross_angular(&minus_m_w, &new_v_angular);

    // data.a_gf[i] += jdata.S() * jmodel.jointVelocitySelector(a);
    // jointVelocitySelector(a) is only a[joint_id]
    // I couldn't print out info about jdata.S() easily but it is ConstraintRevoluteTpl, and I believe the only thing this line does is
    // data.a_gf[i][5] = jmodel.jointVelocitySelector(a)

    let mut new_data_a = Vector!([
        alpha_cross_linear.at(0),
        alpha_cross_linear.at(1),
        alpha_cross_linear.at(2),
        alpha_cross_angular.at(0),
        alpha_cross_angular.at(1),
        a.at(joint_index);
    ]);

    let temp_a_linear = Vector!([
        new_data_a.at(0),
        new_data_a.at(1),
        new_data_a.at(2);
    ]);

    let temp_a_angular = Vector!([
        new_data_a.at(3),
        new_data_a.at(4),
        new_data_a.at(5);
    ]);

    //if(parent > 0)
    //  {
    //    data.a[i] += data.liMi[i].actInv(data.a[parent]);
    //  }
    match joint_index {
        0 => (),
        _ => {
            let limi_actinv_a_parent = act_motion_inv(
                &limi_translation,
                &limi_rotation,
                &temp_a_linear,
                &temp_a_angular,
                &parent_a_linear,
                &parent_a_angular,
            );
            new_data_a = Vector!([
                limi_actinv_a_parent.0.at(0),
                limi_actinv_a_parent.0.at(1),
                limi_actinv_a_parent.0.at(2),
                limi_actinv_a_parent.1.at(0),
                limi_actinv_a_parent.1.at(1),
                limi_actinv_a_parent.1.at(2);
            ]);
        }
    }

    all_a.push(new_data_a.clone());

    // all of these act functions are different than rnea.rs
    // data.oYcrb[i] = data.oinertias[i] = data.oMi[i].act(model.inertias[i]);
    let (data_oycrb_trans_i, data_oycrb_rot_i) = act_inertia(
        &omis[joint_index].0,
        &omis[joint_index].1,
        &levers[joint_index],
        inertias[joint_index].clone(),
    );
    let _data_oinertias_trans_i = data_oycrb_trans_i.clone();
    let _data_oinertias_rot_i = data_oycrb_rot_i.clone();

    all_oycrb.push((data_oycrb_rot_i.clone(), data_oycrb_trans_i.clone()));

    // ov = data.oMi[i].act(data.v[i]);
    let ov = act_motion1(
        &omis[joint_index].0,
        &omis[joint_index].1,
        &new_v_linear,
        &new_v_angular,
    );
    // oa = data.oMi[i].act(data.a[i]);
    let oa = act_motion(&omis[joint_index].0, &omis[joint_index].1, &new_data_a);

    all_ov.push(ov.clone());
    all_oa.push(oa.clone());

    //oa_gf = oa - model.gravity; // add gravity contribution
    let model_gravity = Vector!([0.0, 0.0, -9.81, 0.0, 0.0, 0.0]);
    let oa_gf = &oa - &model_gravity;

    all_oa_gf.push(oa_gf.clone());

    // inertia * vector
    //data.oh[i] = data.oYcrb[i] * ov;
    // mult implemented as this
    //f.linear().noalias() = mass()*(v.linear() - lever().cross(v.angular()));
    //Symmetric3::rhsMult(inertia(),v.angular(),f.angular());
    //f.angular() += lever().cross(f.linear());
    let ov_linear = Vector!([ov.at(0), ov.at(1), ov.at(2);]);
    let ov_angular = Vector!([ov.at(3), ov.at(4), ov.at(5);]);

    let data_oh_linear_temp1 = data_oycrb_trans_i.cross(&ov_angular);
    let data_oh_linear_temp2 = &ov_linear - &data_oh_linear_temp1;
    let data_oh_linear = &masses[joint_index] * &data_oh_linear_temp2;

    let data_oh_angular_temp = rhs_mult(&data_oycrb_rot_i, &ov_angular);
    let data_oh_angular_temp2 = data_oycrb_trans_i.cross(&data_oh_linear);
    let data_oh_angular = &data_oh_angular_temp + &data_oh_angular_temp2;

    // push all to all_oh
    all_oh.push(Vector!([
        data_oh_linear.at(0),
        data_oh_linear.at(1),
        data_oh_linear.at(2),
        data_oh_angular.at(0),
        data_oh_angular.at(1),
        data_oh_angular.at(2);
    ]));

    //data.of[i] = data.oYcrb[i] * oa_gf + ov.cross(data.oh[i]);
    // cross is ov.cross(data.oh[i]), so v is ov, linear() is oh_linear and angular() is oh_angular
    // the cross here is not the regular cross product since the vectors are 6D
    // it is implemented in pinocchio/include/pinocchio/spatial/motion-dense.hpp cross_impl,
    // and it calls a motion_action, which is implemented in pinocchio/include/pinocchio/spatial/force-dense.hpp motion_action
    // final line is data.f[i] += data.v[i].cross(data.h[i]);
    /*
    void motion_action(const MotionDense<M1> & v, ForceDense<M2> & fout) const
    {
      std::cout << "ForceDense::motion_action" << std::endl;
      fout.linear().noalias() = v.angular().cross(linear());
      fout.angular().noalias() = v.angular().cross(angular())+v.linear().cross(linear());
    }
    */
    let oa_gf_linear = Vector!([oa_gf.at(0), oa_gf.at(1), oa_gf.at(2);]);
    let oa_gf_angular = Vector!([oa_gf.at(3), oa_gf.at(4), oa_gf.at(5);]);

    let data_of_linear_temp1 = data_oycrb_trans_i.cross(&oa_gf_angular);
    let data_of_linear_temp2 = &oa_gf_linear - &data_of_linear_temp1;
    let data_of_linear_temp3 = &masses[joint_index] * &data_of_linear_temp2;

    let data_of_angular_temp = rhs_mult(&data_oycrb_rot_i, &oa_gf_angular);
    let data_of_angular_temp2 = data_oycrb_trans_i.cross(&data_of_linear_temp3);
    let data_of_angular_temp3 = data_of_angular_temp + data_of_angular_temp2;

    // now ov.cross(data_oh[i])
    let data_of_linear_temp4 = ov_angular.cross(&data_oh_linear);

    let data_of_angular_temp4 = ov_angular.cross(&data_oh_angular);
    let data_of_angular_temp5 = ov_linear.cross(&data_oh_linear);
    let data_of_angular_temp6 = data_of_angular_temp4 + data_of_angular_temp5;

    let data_of_linear = data_of_linear_temp3 + data_of_linear_temp4;
    let data_of_angular = data_of_angular_temp3 + data_of_angular_temp6;
    all_of.push(Vector!([
        data_of_linear.at(0),
        data_of_linear.at(1),
        data_of_linear.at(2),
        data_of_angular.at(0),
        data_of_angular.at(1),
        data_of_angular.at(2);
    ]));
    // Correct until here

    // J_cols = data.oMi[i].act(jdata.S());
    // S is ConstraintRevoluteTpl, and this function can be found in:
    // include/pinocchio/multibody/joint/joint-revolute.hpp, ConstraintRevoluteTpl::se3Action
    let (j_cols_linear, j_cols_angular) = act_constraint(
        &omis[joint_index].0,
        &omis[joint_index].1,
        robot_info.joint_axes[joint_index],
    );

    j_cols.push(Vector!([
        j_cols_linear.at(0),
        j_cols_linear.at(1),
        j_cols_linear.at(2),
        j_cols_angular.at(0),
        j_cols_angular.at(1),
        j_cols_angular.at(2);
    ]));

    //motionSet::motion_action(ov,J_cols,dJ_cols);
    let (dj_cols_linear, dj_cols_angular) =
        motion_action(&ov_linear, &ov_angular, &j_cols_linear, &j_cols_angular);

    dj_cols.push(Vector!([
        dj_cols_linear.at(0),
        dj_cols_linear.at(1),
        dj_cols_linear.at(2),
        dj_cols_angular.at(0),
        dj_cols_angular.at(1),
        dj_cols_angular.at(2);
    ]));

    let oa_gf_parent_linear = match joint_index {
        0 => Vector!([0.0, 0.0, 0.0]),
        _ => Vector!([
            all_oa_gf[joint_index - 1].at(0),
            all_oa_gf[joint_index - 1].at(1),
            all_oa_gf[joint_index - 1].at(2);
        ]),
    };
    let oa_gf_parent_angular = match joint_index {
        0 => Vector!([0.0, 0.0, 9.81]),
        _ => Vector!([
            all_oa_gf[joint_index - 1].at(3),
            all_oa_gf[joint_index - 1].at(4),
            all_oa_gf[joint_index - 1].at(5);
        ]),
    };

    //motionSet::motion_action(data.oa_gf[parent],J_cols,dAdq_cols);
    let (mut dadq_cols_linear, mut dadq_cols_angular) = motion_action(
        &oa_gf_parent_linear,
        &oa_gf_parent_angular,
        &j_cols_linear,
        &j_cols_angular,
    );

    // dAdv_cols = dJ_cols;
    let mut dadv_cols_linear = dj_cols_linear.clone();
    let mut dadv_cols_angular = dj_cols_angular.clone();

    //if(parent > 0)
    //  {
    //    motionSet::motion_action(data.ov[parent],J_cols,dVdq_cols);
    //    motionSet::motion_action<ADDTO>(data.ov[parent],dVdq_cols,dAdq_cols);
    //    dAdv_cols.noalias() += dVdq_cols;
    //  }
    //  else
    //  {
    //    dVdq_cols.setZero();
    //  }

    let (dvdq_cols_linear, dvdq_cols_angular) = match joint_index {
        0 => (Vector!([0.0, 0.0, 0.0]), Vector!([0.0, 0.0, 0.0])),
        _ => {
            // joint_index is one more than what it actually should be, so in parent for oa_gfs I should check joint_index for parent
            let data_ov_parent_linear = Vector!([
                all_ov[joint_index - 1].at(0),
                all_ov[joint_index - 1].at(1),
                all_ov[joint_index - 1].at(2);
            ]);
            let data_ov_parent_angular = Vector!([
                all_ov[joint_index - 1].at(3),
                all_ov[joint_index - 1].at(4),
                all_ov[joint_index - 1].at(5);
            ]);
            motion_action(
                &data_ov_parent_linear,
                &data_ov_parent_angular,
                &j_cols_linear,
                &j_cols_angular,
            )
        }
    };

    //    motionSet::motion_action<ADDTO>(data.ov[parent],dVdq_cols,dAdq_cols);
    //    dAdv_cols.noalias() += dVdq_cols;
    match joint_index {
        0 => (),
        _ => {
            // joint_index is one more than what it actually should be, so in parent for oa_gfs I should check joint_index for parent
            let data_ov_parent_linear = Vector!([
                all_ov[joint_index - 1].at(0),
                all_ov[joint_index - 1].at(1),
                all_ov[joint_index - 1].at(2);
            ]);
            let data_ov_parent_angular = Vector!([
                all_ov[joint_index - 1].at(3),
                all_ov[joint_index - 1].at(4),
                all_ov[joint_index - 1].at(5);
            ]);
            let (dadq_add_linear, dadq_add_angular) = motion_action(
                &data_ov_parent_linear,
                &data_ov_parent_angular,
                &dvdq_cols_linear,
                &dvdq_cols_angular,
            );
            dadq_cols_linear = dadq_cols_linear + dadq_add_linear;
            dadq_cols_angular = dadq_cols_angular + dadq_add_angular;
            dadv_cols_linear = &dadv_cols_linear + &dvdq_cols_linear;
            dadv_cols_angular = &dadv_cols_angular + &dvdq_cols_angular;
        }
    };

    dadq_cols.push(Vector!([
        dadq_cols_linear.at(0),
        dadq_cols_linear.at(1),
        dadq_cols_linear.at(2),
        dadq_cols_angular.at(0),
        dadq_cols_angular.at(1),
        dadq_cols_angular.at(2);
    ]));

    dadv_cols.push(Vector!([
        dadv_cols_linear.at(0),
        dadv_cols_linear.at(1),
        dadv_cols_linear.at(2),
        dadv_cols_angular.at(0),
        dadv_cols_angular.at(1),
        dadv_cols_angular.at(2);
    ]));

    dvdq_cols.push(Vector!([
        dvdq_cols_linear.at(0),
        dvdq_cols_linear.at(1),
        dvdq_cols_linear.at(2),
        dvdq_cols_angular.at(0),
        dvdq_cols_angular.at(1),
        dvdq_cols_angular.at(2);
    ]));

    //// computes variation of inertias
    //data.doYcrb[i] = data.oYcrb[i].variation(ov);
    //
    //addForceCrossMatrix(data.oh[i],data.doYcrb[i]);

    let data_doycrb_i = inertia_variation(
        &data_oycrb_rot_i.clone(),
        &data_oycrb_trans_i.clone(),
        &ov_linear,
        &ov_angular,
        masses[joint_index].clone(),
    );

    let data_doycrb_i = add_force_cross_matrix(
        data_oh_linear.clone(),
        data_oh_angular.clone(),
        data_doycrb_i.clone(),
    );

    all_doycrb.push(data_doycrb_i.clone());
}

// InertiaTpl __plus__(const InertiaTpl & Yb) const
// {
// /* Y_{a+b} = ( m_a+m_b,
// *             (m_a*c_a + m_b*c_b ) / (m_a + m_b),
// *             I_a + I_b - (m_a*m_b)/(m_a+m_b) * AB_x * AB_x )
// */
//
// const Scalar eps = ::Eigen::NumTraits<Scalar>::epsilon();
//
// const Scalar & mab = mass()+Yb.mass();
// const Scalar mab_inv = Scalar(1)/math::max((Scalar)(mass()+Yb.mass()),eps);
// const Vector3 & AB = (lever()-Yb.lever()).eval();
// return InertiaTpl(mab,
//                 (mass()*lever()+Yb.mass()*Yb.lever())*mab_inv,
//                 inertia()+Yb.inertia() - (mass()*Yb.mass()*mab_inv)* typename Symmetric3::SkewSquare(AB));
// }
fn add_inertia(
    lhs_mass: &Scalar,
    lhs_lever: &Vector,
    lhs_inertia: &Matrix,
    rhs_mass: &Scalar,
    rhs_lever: &Vector,
    rhs_inertia: &Matrix,
) -> (Scalar, Vector, Matrix) {
    let new_mass = lhs_mass + rhs_mass;

    let mab_inv = &Scalar!(1.0) / &new_mass;

    let mass_times_lever_lhs = lhs_mass * lhs_lever;
    let mass_times_lever_rhs = rhs_mass * rhs_lever;
    let mass_times_lever_sum = &mass_times_lever_lhs + &mass_times_lever_rhs;
    let new_lever = &mab_inv * &mass_times_lever_sum;

    let inertia_sum = lhs_inertia + rhs_inertia;
    let mass_times_mass = lhs_mass * rhs_mass;
    let mass_times_mass_mab_inv = mass_times_mass * &mab_inv;

    let ab = lhs_lever - rhs_lever;
    // this skew_square is implemented in pinocchio/include/pinocchio/spatial/symmetric3.hpp
    //    const Scalar & x = v[0], & y = v[1], & z = v[2];
    //    return Symmetric3Tpl( -y*y-z*z,
    //                         x*y    ,  -x*x-z*z,
    //                         x*z    ,   y*z    ,  -x*x-y*y );
    // I don't want to put this as a function, as there are other skewSquare functions that have different implementations
    let x = ab.at(0);
    let y = ab.at(1);
    let z = ab.at(2);
    let skew_square_ab = Matrix!([
        &(&-&y*&y)-&(&z*&z), &x*&y, &x*&z;
        &x*&y, &(&-&x*&x)-&(&z*&z), &y*&z;
        &x*&z, &y*&z, &(&-&x*&x)-&(&y*&y)
    ]);

    let new_inertia_temp = mass_times_mass_mab_inv * &skew_square_ab;

    let new_inertia = &inertia_sum - &new_inertia_temp;

    (new_mass, new_lever.clone(), new_inertia)
}

fn second_pass(
    j_cols_vec: &mut [Vector],
    dvdq_cols: &mut [Vector],
    dadq_cols: &mut [Vector],
    dadv_cols: &mut [Vector],
    dfdq_cols: &mut [Vector],
    dfdv_cols: &mut [Vector],
    dfda_cols: &mut [Vector],
    dytj_cols: &mut [Vector],
    rnea_partial_dq_rows1: &mut [Vector],
    rnea_partial_dq_rows2: &mut [Vector],
    rnea_partial_dv_rows1: &mut [Vector],
    rnea_partial_dv_rows2: &mut [Vector],
    rnea_partial_da: &mut [Vector],
    of: &mut [Vector],
    taus: &mut [Scalar],
    all_oycrb: &mut [(Matrix, Vector)],
    all_doycrb: &mut [Matrix],
    masses: &mut [Scalar],
    joint_index: usize,
) {
    let n = masses.len();
    let j_cols = j_cols_vec[joint_index].clone();
    //jmodel.jointVelocitySelector(data.tau).noalias() = J_cols.transpose()*data.of[i].toVector();
    let of_i = of[joint_index].clone();
    let of_parent = match joint_index {
        0 => Vector!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        _ => of[joint_index - 1].clone(),
    };
    let tau = j_cols.dot(&of_i).define(format!("tau_{}", joint_index));
    taus[joint_index] = tau.clone();

    let oycrb_i = all_oycrb[joint_index].clone(); // This is not updated yet, and it shouldn't be. Be careful!
    let doycrb_i = all_doycrb[joint_index].clone(); // This is not updated yet, and it shouldn't be. Be careful!

    // motionSet::inertiaAction(data.oYcrb[i],J_cols,dFda_cols);
    let dfda_cols_ = inertia_vec_mult(masses[joint_index].clone(), &oycrb_i.1, &oycrb_i.0, &j_cols);
    //dFda_cols.push(dFda_cols_.define(format!("dFda_cols_{}", joint_id)));
    dfda_cols[joint_index] = dfda_cols_.clone(); // dfda_cols is correct

    // rnea_partial_da_.block(jmodel.idx_v(),jmodel.idx_v(),jmodel.nv(),data.nvSubtree[i]).noalias()
    // = J_cols.transpose()*data.dFda.middleCols(jmodel.idx_v(),data.nvSubtree[i]);

    // dfda size is 6xN, so I need to use this:
    //let dfda_cols_matrix_data: [[Scalar; N]; 6] =
    //    core::array::from_fn(|row_idx| { // Iterates 6 times for rows (0 to 5)
    //        core::array::from_fn(|col_idx| { // Iterates N times for columns (0 to N-1)
    //            // Matrix element at (row_idx, col_idx) is dfda_cols[col_idx].at(row_idx)
    //            // This assumes dfda_cols is a Vec or slice of Vector with length N.
    //            // And Scalar implements Clone, and .at() returns something cloneable.
    //            dfda_cols[col_idx].at(row_idx).clone()
    //        })
    //    });
    let mut dfda_cols_matrix = Matrix::zero(6, n);
    for (i, dfda_col) in dfda_cols.iter().enumerate() {
        for j in 0..dfda_col.size() {
            dfda_cols_matrix.set((j, i), &dfda_col.at(j).clone()); // dfda last row first and last values are wrong
        }
    }
    //let dfda_cols_matrix = Matrixrr!(from_vectors:
    //    dfda_cols[0],
    //    dfda_cols[1],
    //    dfda_cols[2],
    //    dfda_cols[3],
    //    dfda_cols[4],
    //    dfda_cols[5],
    //);

    //let j_cols_jointcol = j_cols_matrix.col(joint_index);
    let j_cols_jointcol = j_cols_vec[joint_index].clone();

    // convert j_cols_jointcol to a Matrix
    let j_cols_jointcol = Matrix!(from_vector:
        j_cols_jointcol
    );

    let rnea_partial_da_temp = &j_cols_jointcol
        .transpose()
        .clone()
        .matmul(&dfda_cols_matrix);

    //let data_for_vector_n: [Scalar; N] = core::array::from_fn(|col_idx| {
    //    // Extract element from the first (and only) row of rnea_partial_da_temp
    //    rnea_partial_da_temp.at(col_idx).clone()
    //});
    //
    //let rnea_partial_da_temp2 = Vector::<N>::new(data_for_vector_n);
    let rnea_partial_da_temp2 = rnea_partial_da_temp.transpose().to_vector();

    rnea_partial_da[joint_index] = rnea_partial_da_temp2;

    // for some reason dadv_cols is half of what it should be
    dadv_cols[joint_index] = Vector!([
        &dadv_cols[joint_index].at(0) * &Scalar!(2.0),
        &dadv_cols[joint_index].at(1) * &Scalar!(2.0),
        &dadv_cols[joint_index].at(2) * &Scalar!(2.0),
        &dadv_cols[joint_index].at(3) * &Scalar!(2.0),
        &dadv_cols[joint_index].at(4) * &Scalar!(2.0),
        &dadv_cols[joint_index].at(5) * &Scalar!(2.0);
    ]);

    // dFdv_cols.noalias() = data.doYcrb[i] * J_cols;
    let dfdv_cols_temp = &doycrb_i.matmul_vec(&j_cols);
    // motionSet::inertiaAction<ADDTO>(data.oYcrb[i],dAdv_cols,dFdv_cols);
    let dfdv_cols_temp2 = inertia_vec_mult(
        masses[joint_index].clone(),
        &oycrb_i.1,
        &oycrb_i.0,
        &dadv_cols[joint_index],
    );
    let dfdv_cols_final = dfdv_cols_temp + &dfdv_cols_temp2;
    dfdv_cols[joint_index] = dfdv_cols_final.clone();

    //let dfdv_data: [[Scalar; N]; 6] =
    //    core::array::from_fn(|r_idx| {
    //        core::array::from_fn(|c_idx| {
    //            dfdv_cols[c_idx].at(r_idx).clone()
    //        })
    //    });
    //let dfdv_cols_matrix = Matrix::<6, N>::new(dfdv_data)
    //    .define(format!("dfdv_cols_{}", joint_index)); // Adjusted define name for clarity
    // TODO: HERE
    //let dfdv_cols_matrix = Matrix!(from_vectors:
    //    dfdv_cols[0].clone().define(format!("dfdv_cols_0_{}", joint_index)),
    //    dfdv_cols[1].clone().define(format!("dfdv_cols_1_{}", joint_index)),
    //    dfdv_cols[2].clone().define(format!("dfdv_cols_2_{}", joint_index)),
    //    dfdv_cols[3].clone().define(format!("dfdv_cols_3_{}", joint_index)),
    //    dfdv_cols[4].clone().define(format!("dfdv_cols_4_{}", joint_index)),
    //    dfdv_cols[5].clone().define(format!("dfdv_cols_5_{}", joint_index)),
    //).define(format!("dfdv_cols_final_{}", joint_index));

    let mut dfdv_cols_matrix = Matrix::zero(6, n);
    for (i, dfdv_col) in dfdv_cols.iter().enumerate() {
        for j in 0..dfdv_col.size() {
            dfdv_cols_matrix.set((j, i), &dfdv_col.at(j).clone());
        }
    }

    // I don't know why, but for some reason the dvdq_cols values are double of what they should be
    dvdq_cols[joint_index] = Vector!([
        &dvdq_cols[joint_index].at(0) / &Scalar!(2.0),
        &dvdq_cols[joint_index].at(1) / &Scalar!(2.0),
        &dvdq_cols[joint_index].at(2) / &Scalar!(2.0),
        &dvdq_cols[joint_index].at(3) / &Scalar!(2.0),
        &dvdq_cols[joint_index].at(4) / &Scalar!(2.0),
        &dvdq_cols[joint_index].at(5) / &Scalar!(2.0);
    ]);

    // // dtau/dq
    // if(parent>0)
    // {
    //   dFdq_cols.noalias() = data.doYcrb[i] * dVdq_cols;
    //   motionSet::inertiaAction<ADDTO>(data.oYcrb[i],dAdq_cols,dFdq_cols);
    //   std::cout << "dFdq_cols temp 1: \n" << dFdq_cols << std::endl;
    // }
    // else{
    //   motionSet::inertiaAction(data.oYcrb[i],dAdq_cols,dFdq_cols);
    //   std::cout << "dFdq_cols temp 2: \n" << dFdq_cols << std::endl;
    // }
    dfdq_cols[joint_index] = match joint_index {
        // motionSet::inertiaAction(data.oYcrb[i],dAdq_cols,dFdq_cols);
        0 => inertia_vec_mult(
            masses[joint_index].clone(),
            &all_oycrb[joint_index].1,
            &all_oycrb[joint_index].0,
            &dadq_cols[joint_index],
        ),
        //   dFdq_cols.noalias() = data.doYcrb[i] * dVdq_cols;
        //   motionSet::inertiaAction<ADDTO>(data.oYcrb[i],dAdq_cols,dFdq_cols);
        _ => {
            let dfdq_temp = &doycrb_i.clone().matmul_vec(&dvdq_cols[joint_index].clone());
            let dfdq_temp2 = inertia_vec_mult(
                masses[joint_index].clone(),
                &oycrb_i.1,
                &oycrb_i.0,
                &dadq_cols[joint_index],
            );
            dfdq_temp + dfdq_temp2
        }
    };

    // dfdq is correct
    //let dfdq_data: [[Scalar; N]; 6] =
    //    core::array::from_fn(|r_idx| {
    //        core::array::from_fn(|c_idx| {
    //            dfdq_cols[c_idx].at(r_idx).clone()
    //        })
    //    });
    //let dfdq_temp = Matrix::<6, N>::new(dfdq_data)
    //    .define(format!("dfdq_temp_generic_{}", joint_index));
    let mut dfdq_matrix = Matrix::zero(6, n);
    for (i, dfdq_col) in dfdq_cols.iter().enumerate() {
        for j in 0..dfdq_col.size() {
            dfdq_matrix.set((j, i), &dfdq_col.at(j).clone());
        }
    }

    //dYtJ_cols.transpose().noalias() = J_cols.transpose() * data.doYcrb[i];
    let dytj_col = &j_cols_jointcol.transpose().clone().matmul(&doycrb_i);
    // push this to dytj_cols
    dytj_cols[joint_index] = dytj_col.clone().transpose().to_vector();

    //let dfda_cols_data: [[Scalar; N]; 6] =
    //    core::array::from_fn(|r_idx| {
    //        core::array::from_fn(|c_idx| {
    //            dfda_cols[c_idx].at(r_idx).clone()
    //        })
    //    });
    //let dfda_cols_matrix = Matrix::<6, N>::new(dfda_cols_data);

    //let dytj_data: [[Scalar; N]; 6] =
    //    core::array::from_fn(|r_idx| {
    //        core::array::from_fn(|c_idx| {
    //            dytj_cols[c_idx].at(r_idx).clone() // Assuming dytj_cols is Vec<Vector>
    //        })
    //    });
    //let dytj = Matrix::<6, N>::new(dytj_data);
    //let dytj = Matrix!(from_vectors:
    //    dytj_cols[0].clone().define(format!("dytj_cols_0_{}", joint_index)),
    //    dytj_cols[1].clone().define(format!("dytj_cols_1_{}", joint_index)),
    //    dytj_cols[2].clone().define(format!("dytj_cols_2_{}", joint_index)),
    //    dytj_cols[3].clone().define(format!("dytj_cols_3_{}", joint_index)),
    //    dytj_cols[4].clone().define(format!("dytj_cols_4_{}", joint_index)),
    //    dytj_cols[5].clone().define(format!("dytj_cols_5_{}", joint_index)),
    //).define(format!("dytj_final_{}", joint_index));
    let mut dytj = Matrix::zero(6, n);
    for (i, dytj_col) in dytj_cols.iter().enumerate() {
        for j in 0..dytj_col.size() {
            //println!("i: {}, j: {}, dytj_cols[i].at(j): {:?}", i, j, dytj_cols[i].at(j));
            dytj.set((j, i), &dytj_col.at(j).clone());
        }
    }

    //rnea_partial_dq_.block(idx_v_plus, idx_v, nv_subtree_plus, nv).noalias() =
    //data.dFda.middleCols(idx_v_plus, nv_subtree_plus).transpose() * dAdq_cols
    //+ dYtJ.middleCols(idx_v_plus, nv_subtree_plus).transpose() * dVdq_cols;
    let rnea_partial_dq_temp = &dfda_cols_matrix
        .transpose()
        .matmul_vec(&dadq_cols[joint_index]);
    let rnea_partial_dq_temp2 = &dytj.transpose().clone().matmul_vec(&dvdq_cols[joint_index]);
    let rnea_partial_dq_temp3 = rnea_partial_dq_temp + rnea_partial_dq_temp2;

    //let rnea_partial_dq_temp_vec: std::vec::Vec<Scalar> = (0..N) // N is the const generic
    //    .map(|idx| rnea_partial_dq_temp3.at(idx).clone()) // .at(idx) for Vector
    //    .collect();

    //rnea_partial_dq_.block(idx_v, idx_v, nv, nv_subtree).noalias() =
    //J_cols.transpose() * data.dFdq.middleCols(idx_v, nv_subtree);
    let rnea_partial_dq_temp4 = &j_cols_jointcol.transpose().clone().matmul(&dfdq_matrix);

    //let rnea_partial_dq_temp_vec2: std::vec::Vec<Scalar> = (0..N) // N is the const generic from second_pass
    //    .map(|col_idx| rnea_partial_dq_temp4.at(col_idx).clone())
    //    .collect();

    //let data_for_rows1_array: [Scalar;] = rnea_partial_dq_temp_vec // Consumes rnea_partial_dq_temp_vec. If it's needed later, clone it first.
    //    .try_into()
    //    .unwrap_or_else(|v_err: std::vec::Vec<Scalar>| { // Explicit type for v_err for clarity
    //        panic!("Failed to convert rnea_partial_dq_temp_vec to [Scalar; N] for rnea_partial_dq_rows1. Length was {}", v_err.len())
    //    });
    rnea_partial_dq_rows1[joint_index] = rnea_partial_dq_temp3;

    //let data_for_rows2_array: [Scalar; N] = rnea_partial_dq_temp_vec2 // Consumes rnea_partial_dq_temp_vec2.
    //    .try_into()
    //    .unwrap_or_else(|v_err: std::vec::Vec<Scalar>| { // Explicit type for v_err
    //        panic!("Failed to convert rnea_partial_dq_temp_vec2 to [Scalar; N] for rnea_partial_dq_rows2. Length was {}", v_err.len())
    //    });
    rnea_partial_dq_rows2[joint_index] = rnea_partial_dq_temp4.transpose().to_vector();

    // motionSet::act<ADDTO>(J_cols,data.of[i],dFdq_cols);
    let j_cols_cross_of_i = cross_6d(&j_cols, &of_i);
    dfdq_cols[joint_index] = &dfdq_cols[joint_index] + &j_cols_cross_of_i;

    //rnea_partial_dv_.block(idx_v_plus, idx_v, nv_subtree_plus, nv).noalias() =
    //data.dFda.middleCols(idx_v_plus, nv_subtree_plus).transpose() * dAdv_cols
    //+ dYtJ.middleCols(idx_v_plus, nv_subtree_plus).transpose() * J_cols;
    let rnea_partial_dv_temp = &dfda_cols_matrix
        .transpose()
        .matmul_vec(&dadv_cols[joint_index]);
    let rnea_partial_dv_temp2 = &dytj
        .transpose()
        .clone()
        .matmul_vec(&j_cols_jointcol.to_vector());
    let rnea_partial_dv_temp3 = rnea_partial_dv_temp + rnea_partial_dv_temp2;

    //let rnea_partial_dv_temp_vec: std::vec::Vec<Scalar> = (0..N) // N is the const generic
    //    .map(|idx| rnea_partial_dv_temp3.at(idx).clone()) // .at(idx) for Vector
    //    .collect();

    //rnea_partial_dv_.block(idx_v, idx_v, nv, nv_subtree).noalias() =
    //J_cols.transpose() * data.dFdv.middleCols(idx_v, nv_subtree);
    let rnea_partial_dv_temp4 = &j_cols_jointcol
        .transpose()
        .clone()
        .matmul(&dfdv_cols_matrix);
    //let rnea_partial_dv_temp_vec2: std::vec::Vec<Scalar> = (0..N) // N is the const generic
    //    .map(|col_idx| rnea_partial_dv_temp4.at(col_idx).clone()) // .at((0, col_idx)) for Matrix<1,N>
    //    .collect();

    //let data_for_dv_rows1_array: [Scalar; N] = rnea_partial_dv_temp_vec // This consumes rnea_partial_dv_temp_vec.
    //                                                                  // If rnea_partial_dv_temp_vec is needed later, .clone() it first.
    //    .try_into()
    //    .unwrap_or_else(|v_err: std::vec::Vec<Scalar>| { // Explicit type for v_err for clarity
    //        panic!("Failed to convert rnea_partial_dv_temp_vec to [Scalar; N] for rnea_partial_dv_rows1. Length was {}", v_err.len())
    //    });
    rnea_partial_dv_rows1[joint_index] = rnea_partial_dv_temp3;

    //let data_for_dv_rows2_array: [Scalar; N] = rnea_partial_dv_temp_vec2 // This consumes rnea_partial_dv_temp_vec2.
    //                                                                     // If rnea_partial_dv_temp_vec2 is needed later, .clone() it first.
    //    .try_into()
    //    .unwrap_or_else(|v_err: std::vec::Vec<Scalar>| { // Explicit type for v_err
    //        panic!("Failed to convert rnea_partial_dv_temp_vec2 to [Scalar; N] for rnea_partial_dv_rows2. Length was {}", v_err.len())
    //    });
    rnea_partial_dv_rows2[joint_index] = rnea_partial_dv_temp4.transpose().to_vector();

    /*
        //for(Eigen::DenseIndex k =0; k < jmodel.nv(); ++k)
        //  {
        //    MotionRef<typename ColsBlock::ColXpr> m_in(J_cols.col(k));
        //    MotionRef<typename ColsBlock::ColXpr> m_out(dAdq_cols.col(k));
        //    m_out.linear() += model.gravity.linear().cross(m_in.angular());
        //  }
        // we need to update dAdq_cols
        //dAdq_cols(k) += model.gravity.linear().cross(J_cols.col(k).angular());
        // jmodel.nv() is 1 always for revolute joints
        let gravity_linear = Vector!([0.0, 0.0, -9.81]).define("gravity_linear");
        let j_cols_angular = Vector!([
            j_cols.at(3),
            j_cols.at(4),
            j_cols.at(5)
        ]).define("j_cols_angular");
        let cross_product = gravity_linear.cross(&j_cols_angular).define(format!("cross_product_{}", joint_index));
        let dadq_cols_joint_linear = Vector!([
            dadq_cols[joint_index].at(0),
            dadq_cols[joint_index].at(1),
            dadq_cols[joint_index].at(2)
        ]).define("dAdq_cols_joint_linear");
        let dadq_cols_update = (dadq_cols_joint_linear + cross_product).define(format!("dAdq_cols_temp_updated_{}", joint_index));
        let dadq_cols_updated = Vector!([
            dadq_cols_update.at(0),
            dadq_cols_update.at(1),
            dadq_cols_update.at(2),
            dadq_cols[joint_index].at(3),
            dadq_cols[joint_index].at(4),
            dadq_cols[joint_index].at(5)
        ]).define(format!("dAdq_cols_updated_{}", joint_index));

        dadq_cols[joint_index] = dadq_cols_updated;
    */

    //if(parent>0)
    //  {
    //    data.oYcrb[parent] += data.oYcrb[i];
    //    data.doYcrb[parent] += data.doYcrb[i];
    //    data.of[parent] += data.of[i];
    //  }
    match joint_index {
        0 => (),
        _ => {
            let oy = add_inertia(
                &masses[joint_index - 1],
                &all_oycrb[joint_index - 1].1,
                &all_oycrb[joint_index - 1].0,
                &masses[joint_index],
                &all_oycrb[joint_index].1,
                &all_oycrb[joint_index].0,
            );
            masses[joint_index - 1] = oy.0; // correct
            all_oycrb[joint_index - 1] = (oy.2, oy.1); // correct

            let new_doycrb = all_doycrb[joint_index - 1].clone() + all_doycrb[joint_index].clone();
            all_doycrb[joint_index - 1] = new_doycrb;

            of[joint_index - 1] = of_parent + of_i;
        }
    }
}

/// Both model.nq and model.nv are 6 for panda, the hand joints are excluded
/// \param[in] q The joint configuration vector (dim model.nq).
/// \param[in] v The joint velocity vector (dim model.nv).
/// \param[in] a The joint acceleration vector (dim model.nv).
/// jointPlacements are model.jointPlacements in pinocchio, it is a vector of SE3 objects
/// SE3 has a rotation and a translation element
#[allow(dead_code)]
pub fn rneaderivatives(
    qcos: Vector,
    qsin: Vector,
    v: Vector,
    a: Vector,
    robot_info: &RobotInfo,
) -> (Matrix, Matrix, Matrix, Vec<Scalar>) {
    let n_joints = robot_info.n_joints;
    let limi_translations = robot_info.limi_translations.clone();
    let levers = robot_info.levers.clone();
    let masses = robot_info.masses.clone();
    let mut vec_masses: Vec<Scalar> = vec![];
    for i in 0..n_joints {
        vec_masses.push(masses.at(i).clone()); // I shouldn't do this, but currently I don't support assigning something to only one variable
    }
    let inertias = robot_info.inertias.clone();

    let mut limi_rotations: Vec<Matrix> = vec![];

    // we also have data.v, which v[0] is set to zero explicitly, and I assume the rest should also be zero
    // I will keep it constant here, too, but keep in mind
    let mut data_v = vec![];

    let mut all_v: Vec<Vector> = vec![];
    let mut all_of: Vec<Vector> = vec![];
    let mut all_oh: Vec<Vector> = vec![];
    let mut all_doycrb: Vec<Matrix> = vec![];
    let mut all_ov: Vec<Vector> = vec![];
    let mut all_oa: Vec<Vector> = vec![];
    let mut all_oa_gf: Vec<Vector> = vec![];
    let mut all_a: Vec<Vector> = vec![];
    let mut all_oycrb: Vec<(Matrix, Vector)> = vec![];
    let mut j_cols: Vec<Vector> = vec![];
    let mut dj_cols: Vec<Vector> = vec![];
    let mut dadq_cols: Vec<Vector> = vec![];
    let mut dadv_cols: Vec<Vector> = vec![];
    let mut dvdq_cols: Vec<Vector> = vec![];
    let mut omis: Vec<(Matrix, Vector)> = Vec::new();

    // first pass, it takes model.joints[i], data.joints[i], model, data, q, v, a
    for i in 0..n_joints {
        data_v.push(Vector!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
        first_pass(
            qsin.at(i), // qsin and qcos will not change, therefore no reference needed
            qcos.at(i),
            &data_v[i],
            &v,
            &a,
            &mut all_v,
            &mut omis,
            &limi_translations,
            &mut limi_rotations,
            i,
            &levers,
            &vec_masses,
            &inertias,
            &mut all_of,
            &mut all_oh,
            &mut all_doycrb,
            &mut all_ov,
            &mut all_oa,
            &mut all_oa_gf,
            &mut all_a,
            &mut all_oycrb,
            &mut j_cols,
            &mut dj_cols,
            &mut dadq_cols,
            &mut dadv_cols,
            &mut dvdq_cols,
            robot_info,
        );
    }

    let dof = robot_info.n_joints;

    let mut taus: Vec<Scalar> = vec![Scalar!(0.0); dof];

    // for all vec's here, allocate <dof> elements
    let mut dfda_cols: Vec<Vector> = vec![Vector::new("dfda", vec![]); dof];
    let mut dfdv_cols: Vec<Vector> = vec![Vector::new("dfdv", vec![]); dof];
    let mut dfdq_cols: Vec<Vector> = vec![Vector::new("dfdq", vec![]); dof];
    let mut rnea_partial_da_cols: Vec<Vector> = vec![Vector::new("rnea_partial_da1", vec![]); dof];
    let mut rnea_partial_dv_cols1: Vec<Vector> = vec![Vector::new("rnea_partial_dv1", vec![]); dof];
    let mut rnea_partial_dv_cols2: Vec<Vector> = vec![Vector::new("rnea_partial_dv2", vec![]); dof];
    let mut rnea_partial_dq_cols1: Vec<Vector> = vec![Vector::new("rnea_partial_dq1", vec![]); dof];
    let mut rnea_partial_dq_cols2: Vec<Vector> = vec![Vector::new("rnea_partial_dq2", vec![]); dof];

    let mut dytj_cols: Vec<Vector> = vec![Vector::new("dytj", vec![]); dof];

    for i in (0..n_joints).rev() {
        second_pass(
            &mut j_cols,
            &mut dvdq_cols,
            &mut dadq_cols,
            &mut dadv_cols,
            &mut dfdq_cols,
            &mut dfdv_cols,
            &mut dfda_cols,
            &mut dytj_cols,
            &mut rnea_partial_dq_cols1,
            &mut rnea_partial_dq_cols2,
            &mut rnea_partial_dv_cols1,
            &mut rnea_partial_dv_cols2,
            &mut rnea_partial_da_cols,
            &mut all_of,
            &mut taus,
            &mut all_oycrb,
            &mut all_doycrb,
            &mut vec_masses,
            i,
        );
    }

    let scalar_zero = Scalar!(0.0); // For zero initialization

    // 1. Construct rnea_partial_da (Upper triangular)
    // M_da_lower[r][c] = if c <= r then rnea_partial_da_cols[c].at(r) else 0
    // rnea_partial_da = M_da_lower.transpose()
    let m_da_lower_data: Vec<Vec<Scalar>> = (0..dof)
        .map(|r_idx| {
            (0..dof)
                .map(|c_idx| {
                    if c_idx <= r_idx {
                        rnea_partial_da_cols[c_idx].at(r_idx).clone()
                    } else {
                        scalar_zero.clone()
                    }
                })
                .collect()
        })
        .collect();
    let _rnea_partial_da = Matrix::from_scalars(
        "rnea_partial_da_final",
        m_da_lower_data
            .iter()
            .map(|row| row.iter().collect())
            .collect(),
    );

    // 2. Construct rnea_partial_dv
    // M_dv_raw[r][c] = if c <= r then rnea_partial_dv_cols2[c].at(r) else rnea_partial_dv_cols1[r].at(c)
    // rnea_partial_dv = M_dv_raw.transpose()
    let m_dv_raw_data: Vec<Vec<Scalar>> = (0..dof)
        .map(|r_idx| {
            (0..dof)
                .map(|c_idx| {
                    if c_idx <= r_idx {
                        // Lower triangular part (including diagonal) from dv_cols2
                        rnea_partial_dv_cols2[c_idx].at(r_idx).clone()
                    } else {
                        // Strict upper triangular part from dv_cols1
                        rnea_partial_dv_cols1[r_idx].at(c_idx).clone()
                    }
                })
                .collect()
        })
        .collect();
    let _rnea_partial_dv = Matrix::from_scalars(
        "rnea_partial_dv_final",
        m_dv_raw_data
            .iter()
            .map(|row| row.iter().collect())
            .collect(),
    );

    // 3. Construct rnea_partial_dq
    // M_dq_raw[r][c] = if c <= r then rnea_partial_dq_cols2[c].at(r) else rnea_partial_dq_cols1[r].at(c)
    // rnea_partial_dq = M_dq_raw.transpose()
    let m_dq_raw_data: Vec<Vec<Scalar>> = (0..dof)
        .map(|r_idx| {
            (0..dof)
                .map(|c_idx| {
                    if c_idx <= r_idx {
                        // Lower triangular part (including diagonal) from dq_cols2
                        rnea_partial_dq_cols2[c_idx].at(r_idx).clone()
                    } else {
                        // Strict upper triangular part from dq_cols1
                        rnea_partial_dq_cols1[r_idx].at(c_idx).clone()
                    }
                })
                .collect()
        })
        .collect();
    let _rnea_partial_dq = Matrix::from_scalars(
        "rnea_partial_dq_final",
        m_dq_raw_data
            .iter()
            .map(|row| row.iter().collect())
            .collect(),
    );

    // print return manually for now
    //println!("{}", format!("tau_{:?}_0_0", robot_info.n_joints - 1));
    // Final outputs are now properly computed:
    // - _rnea_partial_da: derivative with respect to acceleration
    // - _rnea_partial_dv: derivative with respect to velocity
    // - _rnea_partial_dq: derivative with respect to position

    //println!("rnea_partial_da: {:?}", _rnea_partial_da.to_f64());
    //println!("rnea_partial_dv: {:?}", _rnea_partial_dv.to_f64());
    //println!("rnea_partial_dq: {:?}", _rnea_partial_dq.to_f64());

    (_rnea_partial_da, _rnea_partial_dv, _rnea_partial_dq, taus)
}
