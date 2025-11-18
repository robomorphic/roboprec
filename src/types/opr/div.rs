use std::ops::Div;

use crate::{
    Scalar, Vector,
    analysis::real::Real,
    ir::expr::{OprBinary, create_binary_scalar_expr, create_binary_vector_expr},
};

// implement Div trait for Scalar
impl Div for Scalar {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        &self / &other
    }
}

// Add reference versions
impl<'a> Div<&'a Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: &'a Scalar) -> Self::Output {
        &self / other
    }
}

impl Div<Scalar> for &Scalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Self::Output {
        self / &other
    }
}

impl<'b> Div<&'b Scalar> for &Scalar {
    type Output = Scalar;

    fn div(self, other: &'b Scalar) -> Self::Output {
        let new_id = create_binary_scalar_expr(
            &format!("{}_div_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Div,
        );
        let new_value = &self.value / &other.value;
        Scalar {
            id: new_id,
            value: new_value,
        }
    }
}

// Element-wise division
impl Div for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        &self / &other
    }
}

impl<'a> Div<&'a Vector> for Vector {
    type Output = Vector;

    fn div(self, other: &'a Vector) -> Self::Output {
        &self / other
    }
}

impl Div<Vector> for &Vector {
    type Output = Vector;

    fn div(self, other: Vector) -> Self::Output {
        self / &other
    }
}

impl<'b> Div<&'b Vector> for &Vector {
    type Output = Vector;

    fn div(self, other: &'b Vector) -> Self::Output {
        let size = self.value.len();
        let new_id = create_binary_vector_expr(
            "vector_div",
            self.id.clone(),
            other.id.clone(),
            OprBinary::Div,
            size,
        );

        // Element-wise division
        let result_values: Vec<Real> = self
            .value
            .iter()
            .zip(other.value.iter())
            .map(|(a, b)| a / b)
            .collect();

        Vector {
            id: new_id,
            value: result_values,
        }
    }
}

// Scalar division
impl Div<Scalar> for Vector {
    type Output = Vector;

    fn div(self, scalar: Scalar) -> Vector {
        &self / &scalar
    }
}

impl Div<Scalar> for &Vector {
    type Output = Vector;

    fn div(self, scalar: Scalar) -> Vector {
        self / &scalar
    }
}

impl<'a> Div<&'a Scalar> for Vector {
    type Output = Vector;

    fn div(self, scalar: &'a Scalar) -> Vector {
        &self / scalar
    }
}

impl<'b> Div<&'b Scalar> for &Vector {
    type Output = Vector;

    fn div(self, scalar: &'b Scalar) -> Vector {
        let size0 = self.value.len();
        let new_id = create_binary_vector_expr(
            &format!("{}_div_scalar", self.id.name),
            self.id.clone(),
            scalar.id.clone(),
            OprBinary::Div,
            size0,
        );

        let result_values: Vec<Real> = self.value.iter().map(|v| v / &scalar.value).collect();

        Vector {
            id: new_id,
            value: result_values,
        }
    }
}
