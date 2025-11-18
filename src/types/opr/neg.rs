use std::ops::Neg;

use crate::{
    Matrix, Scalar, Vector,
    analysis::real::Real,
    ir::expr::{
        OprUnary, create_unary_matrix_expr, create_unary_scalar_expr, create_unary_vector_expr,
    },
};

// implement Neg trait for Scalar
impl Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

// Add reference versions
impl Neg for &Scalar {
    type Output = Scalar;

    fn neg(self) -> Self::Output {
        let new_id = create_unary_scalar_expr(
            &format!("neg_{}", self.id.name),
            self.id.clone(),
            OprUnary::Neg,
        );
        let new_value = -&self.value;
        Scalar {
            id: new_id,
            value: new_value,
        }
    }
}

// Vector negation implementations
impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        let new_id = create_unary_vector_expr("vector_neg", self.id.clone(), OprUnary::Neg);

        let result_values: Vec<Real> = self.value.iter().map(|v| -v).collect();

        Vector {
            id: new_id,
            value: result_values,
        }
    }
}

// negate a &matrix
impl Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Neg for &Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        let new_id = create_unary_matrix_expr("matrix_neg", self.id.clone(), OprUnary::Neg);

        let result_values: Vec<Vec<Real>> = self
            .value
            .iter()
            .map(|row| row.iter().map(|v| -v).collect())
            .collect();

        Matrix {
            id: new_id,
            value: result_values,
        }
    }
}
