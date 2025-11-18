use crate::{Scalar, Vector, tests::test_wrapper::run_default_test, vector_from_scalars};

#[test]
fn test_vector_new() {
    run_default_test(|| {
        vector_new();
    });
}

#[test]
fn test_vector_from_scalars() {
    run_default_test(|| {
        vector_from_scalars();
    });
}

#[test]
fn test_vector_from_vector() {
    run_default_test(|| {
        vector_from_vector();
    });
}

#[test]
fn test_vector_get() {
    run_default_test(|| {
        vector_get();
    });
}

#[test]
#[should_panic(expected = "Index 3 out of bounds in Vector get operation, with size 3")]
fn test_vector_get_out_of_bounds() {
    run_default_test(|| {
        vector_get_out_of_bounds();
    });
}

#[test]
fn test_vector_get_f64() {
    run_default_test(|| {
        vector_get_f64();
    });
}

#[test]
fn test_vector_set_valid_index() {
    run_default_test(|| {
        vector_set_valid_index();
    });
}

#[test]
fn test_vector_set_first_element() {
    run_default_test(|| {
        vector_set_first_element();
    });
}

#[test]
fn test_vector_set_last_element() {
    run_default_test(|| {
        vector_set_last_element();
    });
}

#[test]
fn test_vector_set_single_element_vector() {
    run_default_test(|| {
        vector_set_single_element_vector();
    });
}

#[test]
#[should_panic(expected = "Index out of bounds in Vector set operation")]
fn test_vector_set_out_of_bounds() {
    run_default_test(|| {
        vector_set_out_of_bounds();
    });
}

#[test]
#[should_panic(expected = "Index out of bounds in Vector set operation")]
fn test_vector_set_empty_vector() {
    run_default_test(|| {
        vector_set_empty_vector();
    });
}

#[test]
fn test_vector_push_f64_to_empty() {
    run_default_test(|| {
        vector_push_f64_to_empty();
    });
}

#[test]
fn test_vector_push_f64_to_non_empty() {
    run_default_test(|| {
        vector_push_f64_to_non_empty();
    });
}

#[test]
fn test_vector_push_f64_multiple() {
    run_default_test(|| {
        vector_push_f64_multiple();
    });
}

#[test]
fn test_vector_get_name() {
    run_default_test(|| {
        vector_get_name();
    });
}

#[test]
fn test_vector_dot_product() {
    run_default_test(|| {
        vector_dot_product();
    });
}

#[test]
fn test_vector_dot_product_empty() {
    run_default_test(|| {
        vector_dot_product_empty();
    });
}

#[test]
fn test_vector_dot_product_single_element() {
    run_default_test(|| {
        vector_dot_product_single_element();
    });
}

#[test]
fn test_vector_macro() {
    run_default_test(|| {
        vector_macro();
    });
}

#[test]
fn test_vector_macro_single_element() {
    run_default_test(|| {
        vector_macro_single_element();
    });
}

#[test]
fn test_vector_from_scalars_macro() {
    run_default_test(|| {
        vector_from_scalars_macro();
    });
}

#[test]
fn test_vector_define() {
    run_default_test(|| {
        vector_define();
    });
}

#[test]
fn test_vector_clone() {
    run_default_test(|| {
        vector_clone();
    });
}

#[test]
fn test_vector_edge_case_values() {
    run_default_test(|| {
        vector_edge_case_values();
    });
}

fn vector_new() {
    let vec = Vector::new("test_vector", vec![1.0, 2.0, 3.0]);
    assert_eq!(vec.size(), 3);
    assert_eq!(vec.value_f64(), vec![1.0, 2.0, 3.0]);
}

fn vector_from_scalars() {
    let s1 = Scalar::new("s1", 1.0);
    let s2 = Scalar::new("s2", 2.0);
    let s3 = Scalar::new("s3", 3.0);

    let vec = Vector::from_scalars("from_scalars", vec![&s1, &s2, &s3]);
    assert_eq!(vec.size(), 3);
    assert_eq!(vec.value_f64(), vec![1.0, 2.0, 3.0]);
}

fn vector_from_vector() {
    let original = Vector::new("original", vec![1.0, 2.0, 3.0]);
    let copy = Vector::from_vector("copy", &original);
    assert_eq!(copy.size(), 3);
    assert_eq!(copy.value_f64(), vec![1.0, 2.0, 3.0]);
}

fn vector_get() {
    let vec = Vector::new("test_vector", vec![1.0, 2.0, 3.0]);

    let elem0 = vec.get(0);
    let elem1 = vec.get(1);
    let elem2 = vec.get(2);

    assert_eq!(elem0.value_f64(), 1.0);
    assert_eq!(elem1.value_f64(), 2.0);
    assert_eq!(elem2.value_f64(), 3.0);
}

fn vector_get_out_of_bounds() {
    let vec = Vector::new("test_vector", vec![1.0, 2.0, 3.0]);
    vec.get(3); // Should panic
}

fn vector_get_f64() {
    let vec = Vector::new("test_vector", vec![1.0, 2.0, 3.0]);

    assert_eq!(vec.get_f64(0), Some(1.0));
    assert_eq!(vec.get_f64(1), Some(2.0));
    assert_eq!(vec.get_f64(2), Some(3.0));
    assert_eq!(vec.get_f64(3), None); // Out of bounds should return None
}

fn vector_set_valid_index() {
    let mut vec = Vector::new("test_vector", vec![1.0, 2.0, 3.0]);

    vec.set(1, 10.0);
    assert_eq!(vec.value_f64(), vec![1.0, 10.0, 3.0]);
}

fn vector_set_first_element() {
    let mut vec = Vector::new("test_vector", vec![1.0, 2.0, 3.0]);

    vec.set(0, 99.0);
    assert_eq!(vec.value_f64(), vec![99.0, 2.0, 3.0]);
}

fn vector_set_last_element() {
    let mut vec = Vector::new("test_vector", vec![1.0, 2.0, 3.0]);

    vec.set(2, 88.0);
    assert_eq!(vec.value_f64(), vec![1.0, 2.0, 88.0]);
}

fn vector_set_single_element_vector() {
    let mut vec = Vector::new("single", vec![42.0]);

    vec.set(0, 77.0);
    assert_eq!(vec.value_f64(), vec![77.0]);
}

fn vector_set_out_of_bounds() {
    let mut vec = Vector::new("test_vector", vec![1.0, 2.0, 3.0]);

    let _result = vec.set(3, 10.0);
}

fn vector_set_empty_vector() {
    let mut vec = Vector::new("empty", vec![]);

    let _result = vec.set(0, 10.0);
}

fn vector_push_f64_to_empty() {
    let mut vec = Vector::new("empty", vec![]);

    vec.push_f64(5.0);
    assert_eq!(vec.size(), 1);
    assert_eq!(vec.value_f64(), vec![5.0]);
}

fn vector_push_f64_to_non_empty() {
    let mut vec = Vector::new("test_vector", vec![1.0, 2.0]);

    vec.push_f64(3.0);
    assert_eq!(vec.size(), 3);
    assert_eq!(vec.value_f64(), vec![1.0, 2.0, 3.0]);
}

fn vector_push_f64_multiple() {
    let mut vec = Vector::new("test_vector", vec![1.0]);

    vec.push_f64(2.0);
    vec.push_f64(3.0);
    vec.push_f64(4.0);

    assert_eq!(vec.size(), 4);
    assert_eq!(vec.value_f64(), vec![1.0, 2.0, 3.0, 4.0]);
}

fn vector_get_name() {
    let vec = Vector::new("test_name", vec![1.0, 2.0]);
    assert_eq!(vec.get_name(), "test_name");
}

fn vector_dot_product() {
    let v1 = Vector::new("v1", vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new("v2", vec![4.0, 5.0, 6.0]);

    let dot = v1.dot(&v2);
    // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    assert_eq!(dot.value_f64(), 32.0);
}

fn vector_dot_product_empty() {
    let v1 = Vector::new("v1", vec![]);
    let v2 = Vector::new("v2", vec![]);

    let dot = v1.dot(&v2);
    assert_eq!(dot.value_f64(), 0.0);
}

fn vector_dot_product_single_element() {
    let v1 = Vector::new("v1", vec![3.0]);
    let v2 = Vector::new("v2", vec![4.0]);

    let dot = v1.dot(&v2);
    assert_eq!(dot.value_f64(), 12.0);
}

fn vector_macro() {
    let vec = Vector![1.0, 2.0, 3.0];
    assert_eq!(vec.size(), 3);
    assert_eq!(vec.value_f64(), vec![1.0, 2.0, 3.0]);
}

fn vector_macro_single_element() {
    let vec = Vector![42.0];
    assert_eq!(vec.size(), 1);
    assert_eq!(vec.value_f64(), vec![42.0]);
}

fn vector_from_scalars_macro() {
    let s1 = Scalar::new("s1", 1.0);
    let s2 = Scalar::new("s2", 2.0);
    let s3 = Scalar::new("s3", 3.0);

    let vec = vector_from_scalars![s1, s2, s3];
    assert_eq!(vec.size(), 3);
    assert_eq!(vec.value_f64(), vec![1.0, 2.0, 3.0]);
}

fn vector_define() {
    let mut vec = Vector::new("original", vec![1.0, 2.0]);
    let vec = vec.define("redefined".to_string());

    // Value should remain the same after redefining
    assert_eq!(vec.value_f64(), vec![1.0, 2.0]);
    // Name should be updated (though we can't directly test this without accessing id.name)
}

fn vector_clone() {
    let vec1 = Vector::new("original", vec![1.0, 2.0, 3.0]);
    let vec2 = vec1.clone();

    assert_eq!(vec1.size(), vec2.size());
    assert_eq!(vec1.value_f64(), vec2.value_f64());
}

fn vector_edge_case_values() {
    let mut vec = Vector::new("edge_cases", vec![0.0, -1.0, 1.5, -3.14]);

    // Test with zero
    vec.set(0, 0.0);
    assert_eq!(vec.get_f64(0), Some(0.0));

    // Test with negative values
    vec.push_f64(-99.9);
    assert_eq!(vec.get_f64(4), Some(-99.9));

    // Test with fractional values
    vec.set(2, 0.123456);
    assert!((vec.get_f64(2).unwrap() - 0.123456).abs() < 1e-10);
}

/*
//
fn test_cross_product_with_range_analysis() {



    println!("Testing cross product with range analysis...");

    // Create input vectors with specific ranges for range analysis
    let a_range = vec![
        (
            BigRational::from_f64(-2.0).unwrap(),
            BigRational::from_f64(2.0).unwrap(),
        ),
        (
            BigRational::from_f64(-3.0).unwrap(),
            BigRational::from_f64(3.0).unwrap(),
        ),
        (
            BigRational::from_f64(-1.0).unwrap(),
            BigRational::from_f64(1.0).unwrap(),
        ),
    ];

    let b_range = vec![
        (
            BigRational::from_f64(-1.5).unwrap(),
            BigRational::from_f64(1.5).unwrap(),
        ),
        (
            BigRational::from_f64(-2.0).unwrap(),
            BigRational::from_f64(2.0).unwrap(),
        ),
        (
            BigRational::from_f64(-4.0).unwrap(),
            BigRational::from_f64(4.0).unwrap(),
        ),
    ];

    let a = get_program().add_input_vector(
        "a".to_string(),
        a_range,
        vec![Precision::Float64; 3],
        vec![1.0, 2.0, 0.5], // Specific test values
    );

    let b = get_program().add_input_vector(
        "b".to_string(),
        b_range,
        vec![Precision::Float64; 3],
        vec![0.5, 1.0, 2.0], // Specific test values
    );

    // Compute cross product: a × b
    let cross_result = a.cross(&b);

    // Expected result for a = [1.0, 2.0, 0.5] × b = [0.5, 1.0, 2.0]
    // cross = [a.y*b.z - a.z*b.y, a.z*b.x - a.x*b.z, a.x*b.y - a.y*b.x]
    //       = [2.0*2.0 - 0.5*1.0, 0.5*0.5 - 1.0*2.0, 1.0*1.0 - 2.0*0.5]
    //       = [4.0 - 0.5, 0.25 - 2.0, 1.0 - 1.0]
    //       = [3.5, -1.75, 0.0]
    let expected_values = vec![3.5, -1.75, 0.0];

    // Test actual values
    let computed_values = cross_result.value_f64();
    println!("Cross product result: {:?}", computed_values);
    println!("Expected result: {:?}", expected_values);

    for i in 0..3 {
        assert!(
            compare_f64(computed_values[i], expected_values[i]),
            "Mismatch at index {}: expected {}, got {}",
            i,
            expected_values[i],
            computed_values[i]
        );
    }

    // Perform range analysis
    let program = get_program();
    let range_result = range_analysis::<IntervalRange>(&program);
    let cross_range = range_result.get(&cross_result.id).unwrap();
    let range_bounds = cross_range.to_f64();

    println!("Range analysis results:");
    for i in 0..3 {
        println!(
            "Component {}: [{}, {}]",
            i, range_bounds[0][i].0, range_bounds[0][i].1
        );
    }

    // Expected ranges for cross product components:
    // For a ∈ [-2,2] × [-3,3] × [-1,1] and b ∈ [-1.5,1.5] × [-2,2] × [-4,4]
    // Component 0: a.y*b.z - a.z*b.y = [-3,3]*[-4,4] - [-1,1]*[-2,2] = [-12,12] - [-2,2] = [-14,14]
    // Component 1: a.z*b.x - a.x*b.z = [-1,1]*[-1.5,1.5] - [-2,2]*[-4,4] = [-1.5,1.5] - [-8,8] = [-9.5,9.5]
    // Component 2: a.x*b.y - a.y*b.x = [-2,2]*[-2,2] - [-3,3]*[-1.5,1.5] = [-4,4] - [-4.5,4.5] = [-8.5,8.5]
    let expected_ranges = vec![(-14.0, 14.0), (-9.5, 9.5), (-8.5, 8.5)];

    println!("Expected ranges:");
    for i in 0..3 {
        println!(
            "Component {}: [{}, {}]",
            i, expected_ranges[i].0, expected_ranges[i].1
        );

        // Check if computed ranges are close to expected ranges (allowing some tolerance)
        let tolerance = 0.1;
        assert!(
            (range_bounds[0][i].0 - expected_ranges[i].0).abs() < tolerance,
            "Range lower bound mismatch at component {}: expected {}, got {}",
            i,
            expected_ranges[i].0,
            range_bounds[0][i].0
        );
        assert!(
            (range_bounds[0][i].1 - expected_ranges[i].1).abs() < tolerance,
            "Range upper bound mismatch at component {}: expected {}, got {}",
            i,
            expected_ranges[i].1,
            range_bounds[0][i].1
        );
    }

    // print all rangemap
    for (name, range) in range_result.iter() {
        println!("Range for {:?}: {:?}", name, range.range_f64());
    }

    println!("✓ Cross product test passed: both values and ranges are correct!");

    // generate daisy dsl
    //generate_daisy_dsl(get_program(), "cross_product_test.scala");



}
*/
