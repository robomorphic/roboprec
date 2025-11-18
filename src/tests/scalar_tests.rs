use crate::{Scalar, analysis::real::Real, tests::test_wrapper::run_default_test};

#[test]
fn test_scalar_new_constant() {
    run_default_test(|| {
        scalar_new_constant();
    });
}

#[test]
fn test_scalar_from_scalar() {
    run_default_test(|| {
        scalar_from_scalar();
    });
}

#[test]
fn test_scalar_macro_constant() {
    run_default_test(|| {
        scalar_macro_constant();
    });
}

#[test]
fn test_scalar_macro_from_scalar() {
    run_default_test(|| {
        scalar_macro_from_scalar();
    });
}

#[test]
fn test_scalar_define() {
    run_default_test(|| {
        scalar_define();
    });
}

#[test]
fn test_scalar_arithmetic_add() {
    run_default_test(|| {
        scalar_arithmetic_add();
    });
}

#[test]
fn test_scalar_arithmetic_subtract() {
    run_default_test(|| {
        scalar_arithmetic_subtract();
    });
}

#[test]
fn test_scalar_arithmetic_multiply() {
    run_default_test(|| {
        scalar_arithmetic_multiply();
    });
}

#[test]
fn test_scalar_arithmetic_divide() {
    run_default_test(|| {
        scalar_arithmetic_divide();
    });
}

#[test]
fn test_scalar_arithmetic_chain() {
    run_default_test(|| {
        scalar_arithmetic_chain();
    });
}

#[test]
fn test_scalar_negation() {
    run_default_test(|| {
        scalar_negation();
    });
}

#[test]
fn test_scalar_rational_creation() {
    run_default_test(|| {
        scalar_rational_creation();
    });
}

#[test]
fn test_scalar_zero_and_negative() {
    run_default_test(|| {
        scalar_zero_and_negative();
    });
}

#[test]
fn test_scalar_arithmetic_complex_chain() {
    run_default_test(|| {
        scalar_arithmetic_complex_chain();
    });
}

fn scalar_new_constant() {
    let scalar = Scalar::new("test_scalar", 3.14);
    assert_eq!(scalar.value_f64(), 3.14);
}

fn scalar_from_scalar() {
    let original = Scalar::new("original", 2.5);
    let copy = Scalar::from_scalar("copy", &original);
    assert_eq!(copy.value_f64(), 2.5);
}

fn scalar_macro_constant() {
    let scalar = crate::Scalar!(5.0);
    assert_eq!(scalar.value_f64(), 5.0);
}

fn scalar_macro_from_scalar() {
    let original = crate::Scalar!(7.5);
    let copy = crate::Scalar!(original);
    assert_eq!(copy.value_f64(), 7.5);
}

fn scalar_define() {
    let mut scalar = Scalar::new("original", 1.0);
    scalar.define("redefined".to_string());
    assert_eq!(scalar.value_f64(), 1.0); // Value should remain the same
}

fn scalar_arithmetic_add() {
    let a = crate::Scalar!(2.0);
    let b = crate::Scalar!(3.0);
    let result = &a + &b;
    assert_eq!(result.value_f64(), 5.0);
}

fn scalar_arithmetic_subtract() {
    let a = crate::Scalar!(5.0);
    let b = crate::Scalar!(2.0);
    let result = &a - &b;
    assert_eq!(result.value_f64(), 3.0);
}

fn scalar_arithmetic_multiply() {
    let a = crate::Scalar!(4.0);
    let b = crate::Scalar!(3.0);
    let result = &a * &b;
    assert_eq!(result.value_f64(), 12.0);
}

fn scalar_arithmetic_divide() {
    let a = crate::Scalar!(10.0);
    let b = crate::Scalar!(2.0);
    let result = &a / &b;
    assert_eq!(result.value_f64(), 5.0);
}

fn scalar_arithmetic_chain() {
    let a = crate::Scalar!(2.0);
    let b = crate::Scalar!(3.0);
    let c = crate::Scalar!(4.0);
    let result = &(&a + &b) * &c;
    assert_eq!(result.value_f64(), 20.0); // (2 + 3) * 4 = 20
}

fn scalar_negation() {
    let a = crate::Scalar!(5.0);
    let result = -&a;
    assert_eq!(result.value_f64(), -5.0);
}

fn scalar_rational_creation() {
    let rational_val = Real::from_f64(3.14159);
    let scalar = Scalar::new_rational("pi", rational_val.clone());
    assert!((scalar.value_f64() - 3.14159).abs() < 1e-10);
}

fn scalar_zero_and_negative() {
    let zero = crate::Scalar!(0.0);
    let negative = crate::Scalar!(-42.5);

    assert_eq!(zero.value_f64(), 0.0);
    assert_eq!(negative.value_f64(), -42.5);
}

fn scalar_arithmetic_complex_chain() {
    let a = crate::Scalar!(10.0);
    let b = crate::Scalar!(5.0);
    let c = crate::Scalar!(2.0);

    // ((a / b) + c) * c = ((10/5) + 2) * 2 = (2 + 2) * 2 = 8
    let result = &(&(&a / &b) + &c) * &c;
    assert_eq!(result.value_f64(), 8.0);
}
