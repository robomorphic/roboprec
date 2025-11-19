use std::ops::Sub;

use crate::{
    types::{matrix::Matrix, scalar::Scalar, vector::Vector},
    analysis::real::Real,
    ir::expr::{
        OprBinary, create_binary_matrix_expr, create_binary_scalar_expr, create_binary_vector_expr,
    },
};

// implement Sub trait for Scalar
impl Sub for Scalar {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

// Add reference versions
impl<'a> Sub<&'a Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: &'a Scalar) -> Self::Output {
        &self - other
    }
}

impl Sub<Scalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Self::Output {
        self - &other
    }
}

impl<'b> Sub<&'b Scalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, other: &'b Scalar) -> Self::Output {
        let new_id = create_binary_scalar_expr(
            &format!("{}_minus_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Sub,
        );
        let new_value = &self.value - &other.value;
        Scalar {
            id: new_id,
            value: new_value,
        }
    }
}

// subtract a matrix from a matrix, element-wise subtraction
impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

// Add reference versions
impl<'a> Sub<&'a Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, other: &'a Matrix) -> Self::Output {
        &self - other
    }
}

impl Sub<Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Self::Output {
        self - &other
    }
}

impl<'b> Sub<&'b Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: &'b Matrix) -> Self::Output {
        let size0 = other.value.len();
        let size1 = other.value[0].len();
        let new_id = create_binary_matrix_expr(
            &format!("{}_minus_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Sub,
            size0,
            size1,
        );
        let new_value: Vec<Vec<Real>> = self
            .value
            .iter()
            .zip(other.value.iter())
            .map(|(row_self, row_other)| {
                row_self
                    .iter()
                    .zip(row_other.iter())
                    .map(|(v_self, v_other)| v_self - v_other)
                    .collect()
            })
            .collect();
        Matrix {
            id: new_id,
            value: new_value,
        }
    }
}

// matrix - scalar
impl Sub<Scalar> for Matrix {
    type Output = Matrix;

    fn sub(self, other: Scalar) -> Self::Output {
        &self - &other
    }
}

impl<'a> Sub<&'a Scalar> for Matrix {
    type Output = Matrix;

    fn sub(self, other: &'a Scalar) -> Self::Output {
        &self - other
    }
}

impl Sub<Scalar> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: Scalar) -> Self::Output {
        self - &other
    }
}

impl<'b> Sub<&'b Scalar> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: &'b Scalar) -> Self::Output {
        let size0 = self.value.len();
        let size1 = self.value[0].len();
        let new_id = create_binary_matrix_expr(
            &format!("{}_minus_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Sub,
            size0,
            size1,
        );
        let new_value: Vec<Vec<Real>> = self
            .value
            .iter()
            .map(|row| row.iter().map(|v| v - &other.value).collect())
            .collect();
        Matrix {
            id: new_id,
            value: new_value,
        }
    }
}

// Vector - Scalar subtraction
impl Sub<Scalar> for Vector {
    type Output = Vector;

    fn sub(self, other: Scalar) -> Self::Output {
        &self - &other
    }
}

impl<'a> Sub<&'a Scalar> for Vector {
    type Output = Vector;

    fn sub(self, other: &'a Scalar) -> Self::Output {
        &self - other
    }
}

impl Sub<Scalar> for &Vector {
    type Output = Vector;

    fn sub(self, other: Scalar) -> Self::Output {
        self - &other
    }
}

impl<'b> Sub<&'b Scalar> for &Vector {
    type Output = Vector;

    fn sub(self, other: &'b Scalar) -> Self::Output {
        let size = self.value.len();
        let new_id = create_binary_vector_expr(
            &format!("{}_minus_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Sub,
            size,
        );
        let new_value: Vec<Real> = self
            .value
            .iter()
            .map(|v_self| v_self - &other.value)
            .collect();
        Vector {
            id: new_id,
            value: new_value,
        }
    }
}

// Vector subtraction implementations
impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl<'a> Sub<&'a Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: &'a Vector) -> Self::Output {
        &self - other
    }
}

impl Sub<Vector> for &Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        self - &other
    }
}

impl<'b> Sub<&'b Vector> for &Vector {
    type Output = Vector;

    fn sub(self, other: &'b Vector) -> Self::Output {
        let size = self.value.len();
        let new_id = create_binary_vector_expr(
            "vector_sub",
            self.id.clone(),
            other.id.clone(),
            OprBinary::Sub,
            size,
        );

        // Element-wise subtraction
        let result_values: Vec<Real> = self
            .value
            .iter()
            .zip(other.value.iter())
            .map(|(a, b)| a - b)
            .collect();

        Vector {
            id: new_id,
            value: result_values,
        }
    }
}
