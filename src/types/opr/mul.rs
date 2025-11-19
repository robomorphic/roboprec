use std::ops::Mul;

use crate::{
    types::{matrix::Matrix, scalar::Scalar, vector::Vector},
    analysis::real::Real,
    ir::expr::{
        OprBinary, create_binary_matrix_expr, create_binary_scalar_expr, create_binary_vector_expr,
    },
};

// implement Mul trait for Scalar
impl Mul for Scalar {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

// Add reference versions
impl<'a> Mul<&'a Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: &'a Scalar) -> Self::Output {
        &self * other
    }
}

impl Mul<Scalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Self::Output {
        self * &other
    }
}

impl<'b> Mul<&'b Scalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, other: &'b Scalar) -> Self::Output {
        let new_id = create_binary_scalar_expr(
            &format!("{}_mul_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Mul,
        );
        let new_value = &self.value * &other.value;
        Scalar {
            id: new_id,
            value: new_value,
        }
    }
}

// vector-vector Element-wise multiplication
impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl<'a> Mul<&'a Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: &'a Vector) -> Self::Output {
        &self * other
    }
}

impl Mul<Vector> for &Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        self * &other
    }
}

impl<'b> Mul<&'b Vector> for &Vector {
    type Output = Vector;

    fn mul(self, other: &'b Vector) -> Self::Output {
        let size = self.value.len();
        let new_id = create_binary_vector_expr(
            "vector_mul",
            self.id.clone(),
            other.id.clone(),
            OprBinary::Mul,
            size,
        );

        // Element-wise multiplication
        let result_values: Vec<Real> = self
            .value
            .iter()
            .zip(other.value.iter())
            .map(|(a, b)| a * b)
            .collect();

        Vector {
            id: new_id,
            value: result_values,
        }
    }
}

// Scalar-vector multiplication
impl Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, scalar: Scalar) -> Vector {
        &self * &scalar
    }
}

impl<'a> Mul<&'a Scalar> for Vector {
    type Output = Vector;

    fn mul(self, scalar: &'a Scalar) -> Vector {
        &self * scalar
    }
}

impl<'b> Mul<&'b Scalar> for &Vector {
    type Output = Vector;

    fn mul(self, scalar: &'b Scalar) -> Vector {
        let size = self.value.len();
        let new_id = create_binary_vector_expr(
            &format!("{}_mul_{}", self.id.name, scalar.id.name),
            scalar.id.clone(),
            self.id.clone(),
            OprBinary::Mul,
            size,
        );
        let new_value: Vec<Real> = self.value.iter().map(|v| v * &scalar.value).collect();

        Vector {
            id: new_id,
            value: new_value,
        }
    }
}

// scalar * vector
impl Mul<Vector> for &Scalar {
    type Output = Vector;
    fn mul(self, vector: Vector) -> Vector {
        &vector * self
    }
}

// scalar * vector
impl<'b> Mul<&'b Vector> for &Scalar {
    type Output = Vector;

    fn mul(self, vector: &'b Vector) -> Vector {
        vector * self
    }
}

impl Mul<Vector> for Scalar {
    type Output = Vector;
    fn mul(self, vector: Vector) -> Vector {
        &vector * &self
    }
}

impl Mul<&Vector> for Scalar {
    type Output = Vector;
    fn mul(self, vector: &Vector) -> Vector {
        vector * &self
    }
}

// &scalar times &matrix, element-wise
impl<'b> Mul<&'b Matrix> for &Scalar {
    type Output = Matrix;

    fn mul(self, matrix: &'b Matrix) -> Matrix {
        let size0 = matrix.value.len();
        let size1 = matrix.value[0].len();
        let new_id = create_binary_matrix_expr(
            &format!("{}_mul_{}", self.id.name, matrix.id.name),
            self.id.clone(),
            matrix.id.clone(),
            OprBinary::Mul,
            size0,
            size1,
        );

        let result_values: Vec<Vec<Real>> = matrix
            .value
            .iter()
            .map(|row| row.iter().map(|v| v * &self.value).collect())
            .collect();

        Matrix {
            id: new_id,
            value: result_values,
        }
    }
}

impl Mul<Matrix> for Scalar {
    type Output = Matrix;

    fn mul(self, matrix: Matrix) -> Matrix {
        &self * &matrix
    }
}

impl<'a> Mul<&'a Matrix> for Scalar {
    type Output = Matrix;

    fn mul(self, matrix: &'a Matrix) -> Matrix {
        &self * matrix
    }
}

impl Mul<Matrix> for &Scalar {
    type Output = Matrix;

    fn mul(self, matrix: Matrix) -> Matrix {
        self * &matrix
    }
}
