use crate::tests::test_wrapper::run_default_test;

#[test]
fn test_scalar_macro_patterns() {
    run_default_test(|| {
        scalar_macro_patterns();
    });
}

#[test]
fn test_matrix_macro_patterns() {
    run_default_test(|| {
        matrix_macro_patterns();
    });
}

#[test]
fn test_macro_with_expressions() {
    run_default_test(|| {
        macro_with_expressions();
    });
}

#[test]
fn test_macro_edge_cases() {
    run_default_test(|| {
        macro_edge_cases();
    });
}

fn scalar_macro_patterns() {
    // Test literal pattern
    let s1 = roboprec::Scalar!(3.14);
    assert_eq!(s1.value_f64(), 3.14);

    // Test expression pattern (from another scalar)
    let s2 = roboprec::Scalar!(s1);
    assert_eq!(s2.value_f64(), 3.14);

    // Test with literal value
    let s3 = roboprec::Scalar!(2.71);
    assert_eq!(s3.value_f64(), 2.71);
}

fn matrix_macro_patterns() {
    // Test array literal pattern
    let m1 = roboprec::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    assert_eq!(m1.value_f64(), vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    // Test scalar semicolon pattern
    let s1 = roboprec::Scalar!(5.0);
    let s2 = roboprec::Scalar!(6.0);
    let s3 = roboprec::Scalar!(7.0);
    let s4 = roboprec::Scalar!(8.0);
    let m2 = roboprec::Matrix!([s1, s2; s3, s4]);
    assert_eq!(m2.value_f64(), vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    // Test matrix copy pattern
    let m3 = roboprec::Matrix!(m1);
    assert_eq!(m3.value_f64(), vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    // Test concatenation patterns
    let h_concat = roboprec::Matrix!(hcat: m1, m2);
    let v_concat = roboprec::Matrix!(vcat: m1, m2);

    assert_eq!(
        h_concat.value_f64(),
        vec![vec![1.0, 2.0, 5.0, 6.0], vec![3.0, 4.0, 7.0, 8.0]]
    );
    assert_eq!(
        v_concat.value_f64(),
        vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0],
            vec![7.0, 8.0]
        ]
    );
}

fn macro_with_expressions() {
    // Test macros with literal values
    let s1 = roboprec::Scalar!(6.0); // Direct literal value
    assert_eq!(s1.value_f64(), 6.0);

    // Test matrix with literal values
    let m1 = roboprec::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    assert_eq!(m1.value_f64(), vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
}

fn macro_edge_cases() {
    // Test with zero and negative values
    let zero = roboprec::Scalar!(0.0);
    let negative = roboprec::Scalar!(-1.5);

    assert_eq!(zero.value_f64(), 0.0);
    assert_eq!(negative.value_f64(), -1.5);

    // Test matrix with single element
    let single = roboprec::Matrix!([vec![42.0]]);
    assert_eq!(single.value_f64(), vec![vec![42.0]]);

    // Test matrix concatenation with single elements
    let m1 = roboprec::Matrix!([vec![1.0]]);
    let m2 = roboprec::Matrix!([vec![2.0]]);
    let h_result = roboprec::Matrix!(hcat: m1, m2);
    let v_result = roboprec::Matrix!(vcat: m1, m2);

    assert_eq!(h_result.value_f64(), vec![vec![1.0, 2.0]]);
    assert_eq!(v_result.value_f64(), vec![vec![1.0], vec![2.0]]);
}
