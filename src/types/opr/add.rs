use std::ops::Add;

use crate::{
    types::{matrix::Matrix, scalar::Scalar, vector::Vector},
    analysis::real::Real,
    ir::expr::{
        OprBinary, create_binary_matrix_expr, create_binary_scalar_expr, create_binary_vector_expr,
    },
};

// implement Add trait for Scalar
impl Add for Scalar {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

// Add reference versions
impl<'a> Add<&'a Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: &'a Scalar) -> Self::Output {
        &self + other
    }
}

impl Add<Scalar> for &Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Self::Output {
        self + &other
    }
}

impl<'b> Add<&'b Scalar> for &Scalar {
    type Output = Scalar;

    fn add(self, other: &'b Scalar) -> Self::Output {
        let new_id = create_binary_scalar_expr(
            &format!("{}_plus_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Add,
        );
        let new_value = &self.value + &other.value;
        Scalar {
            id: new_id,
            value: new_value,
        }
    }
}

// add a matrix to a matrix, element-wise addition
impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

// Add reference versions
impl<'a> Add<&'a Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, other: &'a Matrix) -> Self::Output {
        &self + other
    }
}

impl Add<Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Self::Output {
        self + &other
    }
}

impl<'b> Add<&'b Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, other: &'b Matrix) -> Self::Output {
        let size0 = self.value.len();
        let size1 = self.value[0].len();
        let new_id = create_binary_matrix_expr(
            &format!("{}_plus_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Add,
            size0,
            size1,
        );
        let new_value: Vec<Vec<Real>> = self
            .value
            .iter()
            .zip(&other.value)
            .map(|(row1, row2)| row1.iter().zip(row2).map(|(v1, v2)| v1 + v2).collect())
            .collect();
        Matrix {
            id: new_id,
            value: new_value,
        }
    }
}

// scalar + matrix, scalar value is added to each element of the matrix
impl<'b> Add<&'b Matrix> for &Scalar {
    type Output = Matrix;

    fn add(self, other: &'b Matrix) -> Self::Output {
        let size0 = other.value.len();
        let size1 = other.value[0].len();
        let new_id = create_binary_matrix_expr(
            &format!("{}_plus_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Add,
            size0,
            size1,
        );
        let new_value: Vec<Vec<Real>> = other
            .value
            .iter()
            .map(|row| row.iter().map(|v| v + &self.value).collect())
            .collect();
        Matrix {
            id: new_id,
            value: new_value,
        }
    }
}

impl Add<Matrix> for &Scalar {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Self::Output {
        self + &other
    }
}

impl<'a> Add<&'a Matrix> for Scalar {
    type Output = Matrix;

    fn add(self, other: &'a Matrix) -> Self::Output {
        &self + other
    }
}

impl Add<Matrix> for Scalar {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Self::Output {
        &self + &other
    }
}

// matrix + scalar should behave the same
impl Add<Scalar> for Matrix {
    type Output = Matrix;

    fn add(self, other: Scalar) -> Self::Output {
        &other + &self
    }
}

impl<'a> Add<&'a Scalar> for Matrix {
    type Output = Matrix;

    fn add(self, other: &'a Scalar) -> Self::Output {
        other + &self
    }
}

impl Add<Scalar> for &Matrix {
    type Output = Matrix;

    fn add(self, other: Scalar) -> Self::Output {
        &other + self
    }
}

impl<'b> Add<&'b Scalar> for &Matrix {
    type Output = Matrix;

    fn add(self, other: &'b Scalar) -> Self::Output {
        other + self
    }
}

// Vector addition implementations
impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl<'a> Add<&'a Vector> for Vector {
    type Output = Vector;

    fn add(self, other: &'a Vector) -> Self::Output {
        &self + other
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        self + &other
    }
}

impl<'b> Add<&'b Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Self::Output {
        let size = self.value.len();
        let new_id = create_binary_vector_expr(
            "vector_add",
            self.id.clone(),
            other.id.clone(),
            OprBinary::Add,
            size,
        );

        // Element-wise addition
        let result_values: Vec<Real> = self
            .value
            .iter()
            .zip(other.value.iter())
            .map(|(a, b)| a + b)
            .collect();

        Vector {
            id: new_id,
            value: result_values,
        }
    }
}

// scalar + vector, scalar value is added to each element of the vector
impl<'b> Add<&'b Vector> for &Scalar {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Self::Output {
        let size = other.value.len();
        let new_id = create_binary_vector_expr(
            &format!("{}_plus_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Add,
            size,
        );
        let new_value: Vec<Real> = other.value.iter().map(|v| v + &self.value).collect();
        Vector {
            id: new_id,
            value: new_value,
        }
    }
}

impl Add<Vector> for &Scalar {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        self + &other
    }
}

impl<'a> Add<&'a Vector> for Scalar {
    type Output = Vector;

    fn add(self, other: &'a Vector) -> Self::Output {
        &self + other
    }
}

impl Add<Vector> for Scalar {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        &self + &other
    }
}

// vector + scalar should behave the same
impl Add<Scalar> for Vector {
    type Output = Vector;

    fn add(self, other: Scalar) -> Self::Output {
        &other + &self
    }
}

impl<'a> Add<&'a Scalar> for Vector {
    type Output = Vector;

    fn add(self, other: &'a Scalar) -> Self::Output {
        other + &self
    }
}

impl Add<Scalar> for &Vector {
    type Output = Vector;

    fn add(self, other: Scalar) -> Self::Output {
        &other + self
    }
}

impl<'b> Add<&'b Scalar> for &Vector {
    type Output = Vector;

    fn add(self, other: &'b Scalar) -> Self::Output {
        other + self
    }
}
