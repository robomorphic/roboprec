use crate::{Scalar, tests::test_wrapper::run_default_test};

#[test]
fn test_sine() {
    run_default_test(|| {
        sine();
    });
}

fn sine() {
    let x = Scalar!(1.55);

    let x2 = &x * &x;
    let x3 = &x2 * &x;
    let x5 = &x3 * &x2;
    let x7 = &x5 * &x2;

    let res = x - (x3 / Scalar!(6.0)) + (x5 / Scalar!(120.0)) - (x7 / Scalar!(5040.0));

    let expected = Scalar!(0.99964451926);

    assert!(
        (&res - &expected).value_f64().abs() < 1e-10,
        "Expected: {}, got: {}",
        expected.value_f64(),
        res.value_f64()
    );
}
