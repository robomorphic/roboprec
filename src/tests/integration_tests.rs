use crate::Scalar;
use crate::tests::test_wrapper::run_default_test;

#[test]
fn test_scalar_matrix_integration() {
    run_default_test(|| {
        scalar_matrix_integration();
    });
}

#[test]
fn test_matrix_operations_with_scalars() {
    run_default_test(|| {
        matrix_operations_with_scalars();
    });
}

#[test]
fn test_complex_matrix_concatenation() {
    run_default_test(|| {
        complex_matrix_concatenation();
    });
}

#[test]
fn test_mixed_construction_patterns() {
    run_default_test(|| {
        mixed_construction_patterns();
    });
}

#[test]
fn test_arithmetic_with_constructed_elements() {
    run_default_test(|| {
        arithmetic_with_constructed_elements();
    });
}

fn scalar_matrix_integration() {
    // Create scalars using different methods
    let s1 = crate::Scalar!(1.0);
    let s2 = Scalar::new("s2", 2.0);
    let s3 = crate::Scalar!(s1); // Copy from s1
    let s4 = &s2 * &crate::Scalar!(2.0); // s4 = 4.0

    // Create matrix from these scalars
    let matrix = crate::Matrix!([s1, s2; s3, s4]);
    let expected = vec![vec![1.0, 2.0], vec![1.0, 4.0]];
    assert_eq!(matrix.value_f64(), expected);
}

fn matrix_operations_with_scalars() {
    // Create scalars for matrix construction
    let a = crate::Scalar!(1.0);
    let b = crate::Scalar!(2.0);
    let c = crate::Scalar!(3.0);
    let d = crate::Scalar!(4.0);

    // Create matrices from scalars
    let m1 = crate::Matrix!([a, b; c, d]);
    let m2 = crate::Matrix!(m1); // Copy matrix

    // Perform matrix operations
    let sum = &m1 + &m2;
    let expected_sum = vec![vec![2.0, 4.0], vec![6.0, 8.0]];
    assert_eq!(sum.value_f64(), expected_sum);
}

fn complex_matrix_concatenation() {
    // Create base matrices
    let m1 = crate::Matrix!([vec![1.0, 2.0]]);
    let m2 = crate::Matrix!([vec![3.0, 4.0]]);
    // Test that we can create many matrices in sequence
    let m3 = crate::Matrix!([vec![5.0, 6.0]]);
    assert_eq!(m3.value_f64(), vec![vec![5.0, 6.0]]);

    // Create more complex structures
    let vertical = crate::Matrix!(vcat: m1, m2);
    let horizontal = crate::Matrix!(hcat: vertical, crate::Matrix!([vec![7.0], vec![8.0]]));

    let expected = vec![vec![1.0, 2.0, 7.0], vec![3.0, 4.0, 8.0]];
    assert_eq!(horizontal.value_f64(), expected);
}

fn mixed_construction_patterns() {
    // Mix constants and scalars
    let s1 = crate::Scalar!(10.0);
    let s2 = crate::Scalar!(20.0);

    // Matrix from constants
    let m_const = crate::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);

    // Matrix from scalars
    let s3 = crate::Scalar!(30.0);
    let s4 = crate::Scalar!(40.0);
    let m_scalar = crate::Matrix!([s1, s2; s3, s4]);

    // Concatenate them
    let result = crate::Matrix!(hcat: m_const, m_scalar);
    let expected = vec![vec![1.0, 2.0, 10.0, 20.0], vec![3.0, 4.0, 30.0, 40.0]];
    assert_eq!(result.value_f64(), expected);
}

fn arithmetic_with_constructed_elements() {
    // Create scalars through arithmetic
    let a = crate::Scalar!(5.0);
    let b = crate::Scalar!(3.0);
    let sum = &a + &b; // 8.0
    let diff = &a - &b; // 2.0
    let prod = &a * &b; // 15.0
    let quot = &a / &b; // 1.666...

    // Use in matrix
    let matrix = crate::Matrix!([sum, diff; prod, quot]);
    let result = matrix.value_f64();

    assert_eq!(result[0][0], 8.0);
    assert_eq!(result[0][1], 2.0);
    assert_eq!(result[1][0], 15.0);
    assert!((result[1][1] - 5.0 / 3.0).abs() < 1e-10);
}
