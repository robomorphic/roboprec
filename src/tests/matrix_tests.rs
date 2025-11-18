use crate::{Matrix, analysis::real::Real, tests::test_wrapper::run_default_test};

#[test]
fn test_matrix_new_constant() {
    run_default_test(|| {
        matrix_new_constant();
    });
}

#[test]
fn test_matrix_from_scalars() {
    run_default_test(|| {
        matrix_from_scalars();
    });
}

#[test]
fn test_matrix_from_matrix() {
    run_default_test(|| {
        matrix_from_matrix();
    });
}

#[test]
fn test_matrix_from_matrices_horizontal() {
    run_default_test(|| {
        matrix_from_matrices_horizontal();
    });
}

#[test]
fn test_matrix_from_matrices_vertical() {
    run_default_test(|| {
        matrix_from_matrices_vertical();
    });
}

#[test]
fn test_matrix_macro_constant_array() {
    run_default_test(|| {
        matrix_macro_constant_array();
    });
}

#[test]
fn test_matrix_macro_from_scalars() {
    run_default_test(|| {
        matrix_macro_from_scalars();
    });
}

#[test]
fn test_matrix_macro_from_matrix() {
    run_default_test(|| {
        matrix_macro_from_matrix();
    });
}

#[test]
fn test_matrix_macro_horizontal_concatenation() {
    run_default_test(|| {
        matrix_macro_horizontal_concatenation();
    });
}

#[test]
fn test_matrix_macro_vertical_concatenation() {
    run_default_test(|| {
        matrix_macro_vertical_concatenation();
    });
}

#[test]
fn test_matrix_macro_multiple_horizontal_concatenation() {
    run_default_test(|| {
        matrix_macro_multiple_horizontal_concatenation();
    });
}

#[test]
fn test_matrix_macro_multiple_vertical_concatenation() {
    run_default_test(|| {
        matrix_macro_multiple_vertical_concatenation();
    });
}

#[test]
fn test_matrix_define() {
    run_default_test(|| {
        matrix_define();
    });
}

#[test]
fn test_matrix_arithmetic_add() {
    run_default_test(|| {
        matrix_arithmetic_add();
    });
}

#[test]
fn test_matrix_arithmetic_subtract() {
    run_default_test(|| {
        matrix_arithmetic_subtract();
    });
}

#[test]
fn test_matrix_scalar_combinations() {
    run_default_test(|| {
        matrix_scalar_combinations();
    });
}

#[test]
fn test_matrix_single_element() {
    run_default_test(|| {
        matrix_single_element();
    });
}

#[test]
fn test_matrix_rectangular() {
    run_default_test(|| {
        matrix_rectangular();
    });
}

#[test]
fn test_matrix_rational_creation() {
    run_default_test(|| {
        matrix_rational_creation();
    });
}

#[test]
fn test_matrix_size() {
    run_default_test(|| {
        matrix_size();
    });
}

#[test]
fn test_matrix_size_empty() {
    run_default_test(|| {
        matrix_size_empty();
    });
}

#[test]
fn test_matrix_size_single_row() {
    run_default_test(|| {
        matrix_size_single_row();
    });
}

#[test]
fn test_matrix_size_single_column() {
    run_default_test(|| {
        matrix_size_single_column();
    });
}

#[test]
fn test_matrix_get_element() {
    run_default_test(|| {
        matrix_get_element();
    });
}

#[test]
fn test_matrix_get_element_single() {
    run_default_test(|| {
        matrix_get_element_single();
    });
}

#[test]
#[should_panic(expected = "Row index out of bounds in Matrix element indexing")]
fn test_matrix_get_element_row_out_of_bounds() {
    run_default_test(|| {
        matrix_get_element_row_out_of_bounds();
    });
}

#[test]
#[should_panic(expected = "Column index out of bounds in Matrix element indexing")]
fn test_matrix_get_element_col_out_of_bounds() {
    run_default_test(|| {
        matrix_get_element_col_out_of_bounds();
    });
}

#[test]
fn test_matrix_get_f64() {
    run_default_test(|| {
        matrix_get_f64();
    });
}

#[test]
fn test_matrix_get_f64_empty() {
    run_default_test(|| {
        matrix_get_f64_empty();
    });
}

#[test]
fn test_matrix_get_name() {
    run_default_test(|| {
        matrix_get_name();
    });
}

#[test]
fn test_matrix_indexing_with_fractional_values() {
    run_default_test(|| {
        matrix_indexing_with_fractional_values();
    });
}

#[test]
fn test_matrix_large_indexing() {
    run_default_test(|| {
        matrix_large_indexing();
    });
}

#[test]
fn test_matrix_multiplication() {
    run_default_test(|| {
        matrix_multiplication();
    });
}

#[test]
fn test_matrix_multiplication2() {
    run_default_test(|| {
        matrix_multiplication2();
    });
}

fn matrix_new_constant() {
    let matrix = Matrix::new("test_matrix", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    assert_eq!(matrix.value_f64(), expected);
}

fn matrix_from_scalars() {
    let s1 = crate::Scalar!(1.0);
    let s2 = crate::Scalar!(2.0);
    let s3 = crate::Scalar!(3.0);
    let s4 = crate::Scalar!(4.0);

    let matrix = Matrix::from_scalars("test", vec![vec![&s1, &s2], vec![&s3, &s4]]);
    let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    assert_eq!(matrix.value_f64(), expected);
}

fn matrix_from_matrix() {
    let original = Matrix::new("original", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let copy = Matrix::from_matrix("copy", &original);
    assert_eq!(copy.value_f64(), original.value_f64());
}

fn matrix_from_matrices_horizontal() {
    let m1 = Matrix::new("m1", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let m2 = Matrix::new("m2", vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = Matrix::from_matrices_horizontal("hcat", vec![&m1, &m2]);
    let expected = vec![vec![1.0, 2.0, 5.0, 6.0], vec![3.0, 4.0, 7.0, 8.0]];
    assert_eq!(result.value_f64(), expected);
}

fn matrix_from_matrices_vertical() {
    let m1 = Matrix::new("m1", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let m2 = Matrix::new("m2", vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = Matrix::from_matrices_vertical("vcat", vec![&m1, &m2]);
    let expected = vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
        vec![5.0, 6.0],
        vec![7.0, 8.0],
    ];
    assert_eq!(result.value_f64(), expected);
}

fn matrix_macro_constant_array() {
    let matrix = crate::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    assert_eq!(matrix.value_f64(), expected);
}

fn matrix_macro_from_scalars() {
    let s1 = crate::Scalar!(1.0);
    let s2 = crate::Scalar!(2.0);
    let s3 = crate::Scalar!(3.0);
    let s4 = crate::Scalar!(4.0);

    let matrix = crate::Matrix!([s1, s2; s3, s4]);
    let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    assert_eq!(matrix.value_f64(), expected);
}

fn matrix_macro_from_matrix() {
    let original = crate::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    let copy = crate::Matrix!(original);
    let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    assert_eq!(copy.value_f64(), expected);
}

fn matrix_macro_horizontal_concatenation() {
    let m1 = crate::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    let m2 = crate::Matrix!([vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = crate::Matrix!(hcat: m1, m2);
    let expected = vec![vec![1.0, 2.0, 5.0, 6.0], vec![3.0, 4.0, 7.0, 8.0]];
    assert_eq!(result.value_f64(), expected);
}

fn matrix_macro_vertical_concatenation() {
    let m1 = crate::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    let m2 = crate::Matrix!([vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = crate::Matrix!(vcat: m1, m2);
    let expected = vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
        vec![5.0, 6.0],
        vec![7.0, 8.0],
    ];
    assert_eq!(result.value_f64(), expected);
}

fn matrix_macro_multiple_horizontal_concatenation() {
    let m1 = crate::Matrix!([vec![1.0], vec![2.0]]);
    let m2 = crate::Matrix!([vec![3.0], vec![4.0]]);
    let m3 = crate::Matrix!([vec![5.0], vec![6.0]]);

    let result = crate::Matrix!(hcat: m1, m2, m3);
    let expected = vec![vec![1.0, 3.0, 5.0], vec![2.0, 4.0, 6.0]];
    assert_eq!(result.value_f64(), expected);
}

fn matrix_macro_multiple_vertical_concatenation() {
    let m1 = crate::Matrix!([vec![1.0, 2.0]]);
    let m2 = crate::Matrix!([vec![3.0, 4.0]]);
    let m3 = crate::Matrix!([vec![5.0, 6.0]]);

    let result = crate::Matrix!(vcat: m1, m2, m3);
    let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    assert_eq!(result.value_f64(), expected);
}

fn matrix_define() {
    let mut matrix = Matrix::new("original", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix = matrix.define("redefined".to_string());
    let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    assert_eq!(matrix.value_f64(), expected); // Value should remain the same
}

fn matrix_arithmetic_add() {
    let m1 = crate::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    let m2 = crate::Matrix!([vec![5.0, 6.0], vec![7.0, 8.0]]);
    let result = &m1 + &m2;
    let expected = vec![vec![6.0, 8.0], vec![10.0, 12.0]];
    assert_eq!(result.value_f64(), expected);
}

fn matrix_arithmetic_subtract() {
    let m1 = crate::Matrix!([vec![5.0, 6.0], vec![7.0, 8.0]]);
    let m2 = crate::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
    let result = &m1 - &m2;
    let expected = vec![vec![4.0, 4.0], vec![4.0, 4.0]];
    assert_eq!(result.value_f64(), expected);
}

//
// fn test_matrix_arithmetic_multiply() {
//
//     let m1 = crate::Matrix!([vec![1.0, 2.0], vec![3.0, 4.0]]);
//     let m2 = crate::Matrix!([vec![5.0, 6.0], vec![7.0, 8.0]]);
//     let result = &m1 * &m2;
//     // Matrix multiplication: [1*5+2*7, 1*6+2*8; 3*5+4*7, 3*6+4*8] = [19, 22; 43, 50]
//     let expected = vec![vec![19.0, 22.0], vec![43.0, 50.0]];
//     assert_eq!(result.value_f64(), expected);
// }

//
// fn test_matrix_negation() {
//
//     let m = crate::Matrix!([vec![1.0, -2.0], vec![3.0, -4.0]]);
//     let result = -&m;
//     let expected = vec![vec![-1.0, 2.0], vec![-3.0, 4.0]];
//     assert_eq!(result.value_f64(), expected);
// }

fn matrix_scalar_combinations() {
    let s1 = crate::Scalar!(2.0);
    let s2 = crate::Scalar!(3.0);
    let s3 = &s1 + &s2; // s3 = 5.0
    let s4 = crate::Scalar!(1.0);

    let matrix = crate::Matrix!([s1, s2; s3, s4]);
    let expected = vec![vec![2.0, 3.0], vec![5.0, 1.0]];
    assert_eq!(matrix.value_f64(), expected);
}

fn matrix_single_element() {
    let matrix = crate::Matrix!([vec![42.0]]);
    let expected = vec![vec![42.0]];
    assert_eq!(matrix.value_f64(), expected);
}

fn matrix_rectangular() {
    let matrix = crate::Matrix!([vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let expected = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    assert_eq!(matrix.value_f64(), expected);
}

fn matrix_rational_creation() {
    let val1 = Real::from_f64(1.5);
    let val2 = Real::from_f64(2.5);
    let val3 = Real::from_f64(3.5);
    let val4 = Real::from_f64(4.5);

    let matrix = Matrix::new_rational("test", vec![vec![val1, val2], vec![val3, val4]]);
    let expected = vec![vec![1.5, 2.5], vec![3.5, 4.5]];
    assert_eq!(matrix.value_f64(), expected);
}

// === New Matrix Indexing Tests ===

fn matrix_size() {
    let matrix = Matrix::new("test", vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    assert_eq!(matrix.size(), (2, 3)); // 2 rows, 3 columns
}

fn matrix_size_empty() {
    let matrix = Matrix::new("empty", vec![]);
    assert_eq!(matrix.size(), (0, 0)); // 0 rows, 0 columns
}

fn matrix_size_single_row() {
    let matrix = Matrix::new("single_row", vec![vec![1.0, 2.0, 3.0]]);
    assert_eq!(matrix.size(), (1, 3)); // 1 row, 3 columns
}

fn matrix_size_single_column() {
    let matrix = Matrix::new("single_col", vec![vec![1.0], vec![2.0], vec![3.0]]);
    assert_eq!(matrix.size(), (3, 1)); // 3 rows, 1 column
}

fn matrix_get_element() {
    let matrix = Matrix::new(
        "test",
        vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ],
    );

    // Test various elements
    assert_eq!(matrix.get(0, 0).value_f64(), 1.0);
    assert_eq!(matrix.get(0, 1).value_f64(), 2.0);
    assert_eq!(matrix.get(0, 2).value_f64(), 3.0);
    assert_eq!(matrix.get(1, 0).value_f64(), 4.0);
    assert_eq!(matrix.get(1, 1).value_f64(), 5.0);
    assert_eq!(matrix.get(1, 2).value_f64(), 6.0);
    assert_eq!(matrix.get(2, 0).value_f64(), 7.0);
    assert_eq!(matrix.get(2, 1).value_f64(), 8.0);
    assert_eq!(matrix.get(2, 2).value_f64(), 9.0);
}

fn matrix_get_element_single() {
    let matrix = Matrix::new("single", vec![vec![42.0]]);

    let element = matrix.get(0, 0);
    assert_eq!(element.value_f64(), 42.0);
}

fn matrix_get_element_row_out_of_bounds() {
    let matrix = Matrix::new("test", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    matrix.get(2, 0); // Should panic
}

fn matrix_get_element_col_out_of_bounds() {
    let matrix = Matrix::new("test", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    matrix.get(0, 2); // Should panic
}

fn matrix_get_f64() {
    let matrix = Matrix::new("test", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    assert_eq!(matrix.get_f64(0, 0), Some(1.0));
    assert_eq!(matrix.get_f64(0, 1), Some(2.0));
    assert_eq!(matrix.get_f64(1, 0), Some(3.0));
    assert_eq!(matrix.get_f64(1, 1), Some(4.0));

    // Out of bounds should return None
    assert_eq!(matrix.get_f64(2, 0), None);
    assert_eq!(matrix.get_f64(0, 2), None);
    assert_eq!(matrix.get_f64(2, 2), None);
}

fn matrix_get_f64_empty() {
    let matrix = Matrix::new("empty", vec![]);

    assert_eq!(matrix.get_f64(0, 0), None);
}

fn matrix_get_name() {
    let matrix = Matrix::new("test_name", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    assert_eq!(matrix.get_name(), "test_name");
}

fn matrix_indexing_with_fractional_values() {
    let matrix = Matrix::new("fractional", vec![vec![1.5, 2.7], vec![3.14, 0.5]]);

    assert!((matrix.get(0, 0).value_f64() - 1.5).abs() < 1e-10);
    assert!((matrix.get(0, 1).value_f64() - 2.7).abs() < 1e-10);
    assert!((matrix.get(1, 0).value_f64() - 3.14).abs() < 1e-10);
    assert!((matrix.get(1, 1).value_f64() - 0.5).abs() < 1e-10);
}

fn matrix_large_indexing() {
    // Create a larger matrix to test indexing
    let matrix = Matrix::new(
        "large",
        vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0, 10.0],
            vec![11.0, 12.0, 13.0, 14.0, 15.0],
            vec![16.0, 17.0, 18.0, 19.0, 20.0],
        ],
    );

    assert_eq!(matrix.size(), (4, 5));

    // Test corners
    assert_eq!(matrix.get(0, 0).value_f64(), 1.0); // top-left
    assert_eq!(matrix.get(0, 4).value_f64(), 5.0); // top-right
    assert_eq!(matrix.get(3, 0).value_f64(), 16.0); // bottom-left
    assert_eq!(matrix.get(3, 4).value_f64(), 20.0); // bottom-right

    // Test middle
    assert_eq!(matrix.get(1, 2).value_f64(), 8.0);
    assert_eq!(matrix.get(2, 3).value_f64(), 14.0);
}

fn matrix_multiplication() {
    let m1 = Matrix::new("m1", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let m2 = Matrix::new("m2", vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = m1.matmul(&m2);
    let expected = vec![vec![19.0, 22.0], vec![43.0, 50.0]];
    assert_eq!(result.value_f64(), expected);
}

/*
/
fn test_matrix_multiplication_error_analysis2() {



    let m1_0_0_range = (BigRational::from_f64(0.0).unwrap(), BigRational::from_f64(1.0).unwrap());
    let m1_0_0 = get_program().add_input_scalar(
        "m1_0_0".to_string(),
        m1_0_0_range.clone(),
        Precision::create_fixed(32, &IntervalRange::new(m1_0_0_range.0.clone(), m1_0_0_range.1.clone())),
        0.0
    );
    let m1_0_1 = get_program().add_input_scalar(
        "m1_0_1".to_string(),
        m1_0_0_range.clone(),
        Precision::create_fixed(32, &IntervalRange::new(m1_0_0_range.0.clone(), m1_0_0_range.1.clone())),
        0.0
    );
    let m1_1_0 = get_program().add_input_scalar(
        "m1_1_0".to_string(),
        m1_0_0_range.clone(),
        Precision::create_fixed(32, &IntervalRange::new(m1_0_0_range.0.clone(), m1_0_0_range.1.clone())),
        0.0
    );
    let m1_1_1 = get_program().add_input_scalar(
        "m1_1_1".to_string(),
        m1_0_0_range.clone(),
        Precision::create_fixed(32, &IntervalRange::new(m1_0_0_range.0.clone(), m1_0_0_range.1.clone())),
        0.0
    );

    let m1 = Matrix!(
        [m1_0_0, m1_0_1;
        m1_1_0.clone(), m1_1_1.clone();
        m1_1_0, m1_1_1]
    );
    let m2 = Matrix::new("m2", vec![vec![5.0, 6.0, 6.0], vec![7.0, 9.0, 9.0]]);

    let result = m1.matmul(&m2);

    get_program().register_output(result.id);

    let new_program = unroll_ir(&get_program());
    set_program(new_program);

    //generate_daisy_dsl(get_program(), "matrix_multiplication_error_analysis.daisy");

    let program = get_program();
    let (range_results, error_analysis, precision_analysis) = error_analysis::<IntervalRange>(&program);

    error_analysis.iter().for_each(|(id, analysis)| {
        println!("Error analysis for {:?}: {:?}", id, analysis.range_f64());
    });

    precision_analysis.iter().for_each(|(id, analysis)| {
        println!("Precision analysis for {:?}: {:?}", id, analysis);
    });

    //assert!(false)
}
*/

/*

fn test_matrix_matrix_mul() {
    let a = Matrix::<2, 3>::new([
        [Scalar::new(1.0), Scalar::new(2.0), Scalar::new(3.0)],
        [Scalar::new(4.0), Scalar::new(5.0), Scalar::new(6.0)],
    ]);
    let b = Matrix::<3, 2>::new([
        [Scalar::new(7.0), Scalar::new(8.0)],
        [Scalar::new(9.0), Scalar::new(10.0)],
        [Scalar::new(11.0), Scalar::new(12.0)],
    ]);
    let result = &a * &b;

    let result_expected = Matrix::<2, 2>::new([
        [Scalar::new(58.0), Scalar::new(64.0)],
        [Scalar::new(139.0), Scalar::new(154.0)],
    ]);
    assert_eq!(result.get_values(), result_expected.get_values());
}
*/

fn matrix_multiplication2() {
    let m1 = Matrix::new("m1", vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let m2 = Matrix::new(
        "m2",
        vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]],
    );

    // Test multiplication with another matrix
    let result_matrix = m1.matmul(&m2);
    let expected_matrix = vec![vec![58.0, 64.0], vec![139.0, 154.0]];
    assert_eq!(result_matrix.value_f64(), expected_matrix);
}
