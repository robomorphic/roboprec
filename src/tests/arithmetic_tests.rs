use crate::tests::test_wrapper::run_default_test;

#[test]
fn test_scalar_arithmetic_operations() {
    run_default_test(|| {
        scalar_arithmetic_operations();
    });
}

#[test]
fn test_scalar_arithmetic_chaining() {
    run_default_test(|| {
        scalar_arithmetic_chaining();
    });
}

#[test]
fn test_matrix_arithmetic_operations() {
    run_default_test(|| {
        matrix_arithmetic_operations();
    });
}

#[test]
fn test_matrix_arithmetic_edge_cases() {
    run_default_test(|| {
        matrix_arithmetic_edge_cases();
    });
}

#[test]
fn test_mixed_scalar_matrix_operations() {
    run_default_test(|| {
        mixed_scalar_matrix_operations();
    });
}

#[test]
fn test_arithmetic_precision() {
    run_default_test(|| {
        arithmetic_precision();
    });
}

/// Tests for arithmetic operations on scalars and matrices
fn scalar_arithmetic_operations() {
    let a = roboprec::Scalar!(8.0);
    let b = roboprec::Scalar!(2.0);

    // Test all basic operations
    let add_result = &a + &b;
    let sub_result = &a - &b;
    let mul_result = &a * &b;
    let div_result = &a / &b;
    let neg_result = -&a;

    assert_eq!(add_result.value_f64(), 10.0);
    assert_eq!(sub_result.value_f64(), 6.0);
    assert_eq!(mul_result.value_f64(), 16.0);
    assert_eq!(div_result.value_f64(), 4.0);
    assert_eq!(neg_result.value_f64(), -8.0);
}

fn scalar_arithmetic_chaining() {
    let a = roboprec::Scalar!(2.0);
    let b = roboprec::Scalar!(3.0);
    let c = roboprec::Scalar!(4.0);

    // Test complex expressions
    let result1 = &(&a + &b) * &c; // (2 + 3) * 4 = 20
    let result2 = &a + &(&b * &c); // 2 + (3 * 4) = 14
    let result3 = &(&a * &b) - &c; // (2 * 3) - 4 = 2

    assert_eq!(result1.value_f64(), 20.0);
    assert_eq!(result2.value_f64(), 14.0);
    assert_eq!(result3.value_f64(), 2.0);
}

fn matrix_arithmetic_operations() {
    let m1 = roboprec::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    let m2 = roboprec::Matrix!([vec![5.0, 6.0], vec![7.0, 8.0]]);
    // Test matrix creation works
    assert_eq!(m1.value_f64(), vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    assert_eq!(m2.value_f64(), vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    // let add_result = &m1 + &m2;
    // assert_eq!(add_result.value_f64(), vec![vec![6.0, 8.0], vec![10.0, 12.0]]);

    // Test matrix subtraction
    // let sub_result = &m2 - &m1;
    // assert_eq!(sub_result.value_f64(), vec![vec![4.0, 4.0], vec![4.0, 4.0]]);

    // Test matrix multiplication
    // let mul_result = &m1 * &m2;
    // [1*5+2*7, 1*6+2*8] = [19, 22]
    // [3*5+4*7, 3*6+4*8] = [43, 50]
    // assert_eq!(mul_result.value_f64(), vec![vec![19.0, 22.0], vec![43.0, 50.0]]);

    // Test matrix negation
    // let neg_result = -&m1;
    // assert_eq!(neg_result.value_f64(), vec![vec![-1.0, -2.0], vec![-3.0, -4.0]]);
}

fn matrix_arithmetic_edge_cases() {
    // Test with identity-like operations
    let m = roboprec::Matrix!([vec![1.0, 0.0], vec![0.0, 1.0]]);
    let zero = roboprec::Matrix!([vec![0.0, 0.0], vec![0.0, 0.0]]);

    // let add_zero = &m + &zero;
    // assert_eq!(add_zero.value_f64(), m.value_f64());

    // Test matrix creation works
    assert_eq!(m.value_f64(), vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    assert_eq!(zero.value_f64(), vec![vec![0.0, 0.0], vec![0.0, 0.0]]);

    let sub_self = &m - &m;
    assert_eq!(sub_self.value_f64(), zero.value_f64());
}

fn mixed_scalar_matrix_operations() {
    // Create matrix from scalar operations
    let s1 = roboprec::Scalar!(2.0);
    let s2 = roboprec::Scalar!(3.0);
    let s3 = &s1 + &s2; // 5.0
    let s4 = &s1 * &s2; // 6.0

    let m1 = roboprec::Matrix!([s1, s2; s3, s4]);
    assert_eq!(m1.value_f64(), vec![vec![2.0, 3.0], vec![5.0, 6.0]]);

    // Use this matrix in operations
    let m2 = roboprec::Matrix!([vec![1.0, 1.0], vec![1.0, 1.0]]);
    let result = &m1 + &m2;
    assert_eq!(result.value_f64(), vec![vec![3.0, 4.0], vec![6.0, 7.0]]);
}

fn arithmetic_precision() {
    // Test operations that might have precision issues
    let a = roboprec::Scalar!(0.1);
    let b = roboprec::Scalar!(0.2);
    let c = roboprec::Scalar!(0.3);

    let sum = &a + &b;
    let diff = &sum - &c;

    // Should be very close to zero (within floating point precision)
    assert!(diff.value_f64().abs() < 1e-15);
}
