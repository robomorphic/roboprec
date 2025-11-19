use log::info;

use crate::{
    analysis::real::Real,
    ir::expr::{
        OprUnary, create_construct_matrix_expr, create_index_matrix_element_expr,
        create_index_matrix_row_expr, create_matrix_constant, create_unary_matrix_expr,
    },
    ir::identifier::Identifier,
    types::{scalar::Scalar, vector::Vector},
};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub id: Identifier, // Name of the matrix, guaranteed to be unique over the whole program
    pub value: Vec<Vec<Real>>, // Values of the matrix, computed with default values
}

impl Matrix {
    /// This function will be only used when the matrix is constant.
    pub fn new(name: &str, values: Vec<Vec<f64>>) -> Self {
        let rational_values: Vec<Vec<Real>> = values
            .into_iter()
            .map(|row| row.into_iter().map(Real::from_f64).collect())
            .collect();
        Self::new_rational(name, rational_values)
    }

    pub fn new_rational(name: &str, values: Vec<Vec<Real>>) -> Self {
        let new_id = create_matrix_constant(name, values.clone());
        Self {
            id: new_id,
            value: values,
        }
    }

    // This function simply redefines a new value to this vector.
    pub fn define(&mut self, new_name: String) -> Self {
        let new_id = create_unary_matrix_expr(&new_name, self.id.clone(), OprUnary::AssignNoOpt);
        Self {
            id: new_id,
            value: self.value.clone(),
        }
    }

    pub fn sin(&self) -> Self {
        let new_id = create_unary_matrix_expr(
            format!("{}_sin", self.id.name).as_str(),
            self.id.clone(),
            OprUnary::Sine,
        );
        // Compute the sine value using f64 for simplicity
        let sine_values: Vec<Vec<Real>> = self
            .value
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| {
                        let sine_value = v.to_f64().sin();
                        Real::from_f64(sine_value)
                    })
                    .collect()
            })
            .collect();
        Self {
            id: new_id,
            value: sine_values,
        }
    }

    pub fn cos(&self) -> Self {
        let new_id = create_unary_matrix_expr(
            format!("{}_cos", self.id.name).as_str(),
            self.id.clone(),
            OprUnary::Cosine,
        );
        // Compute the cosine value using f64 for simplicity
        let cosine_values: Vec<Vec<Real>> = self
            .value
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| {
                        let cosine_value = v.to_f64().cos();
                        Real::from_f64(cosine_value)
                    })
                    .collect()
            })
            .collect();
        Self {
            id: new_id,
            value: cosine_values,
        }
    }

    pub fn value_f64(&self) -> Vec<Vec<f64>> {
        self.value
            .iter()
            .map(|v| v.iter().map(|e| e.to_f64()).collect())
            .collect()
    }

    /// Create a matrix from scalars using the Construct operation
    /// The scalars should be arranged in row-major order to form the matrix
    pub fn from_scalars(name: &str, scalars: Vec<Vec<&Scalar>>) -> Self {
        let ids: Vec<Vec<Identifier>> = scalars
            .iter()
            .map(|row| row.iter().map(|s| s.id.clone()).collect())
            .collect();
        let new_id = create_construct_matrix_expr(name, ids);

        // Construct the matrix values from scalar values
        let value: Vec<Vec<Real>> = scalars
            .iter()
            .map(|row| row.iter().map(|s| s.value.clone()).collect())
            .collect();

        Self { id: new_id, value }
    }

    /// Create a matrix from another matrix using the Construct operation
    pub fn from_matrix(name: &str, matrix: &Matrix) -> Self {
        let ids: Vec<Vec<Identifier>> = vec![vec![matrix.id.clone()]];
        let new_id = create_construct_matrix_expr(name, ids);

        // Use the input matrix's value
        let value = matrix.value.clone();

        Self { id: new_id, value }
    }

    pub fn from_vector(name: &str, vector: &Vector) -> Self {
        // For this one, get all scalars independently, then construct the matrix
        let mut ids = vec![vec![]; vector.size()];
        //let values = vector.value.iter().map(|v| v.clone()).collect::<Vec<Real>>();
        // rotate values, so that each row is a vector
        let mut values: Vec<Vec<Real>> = vec![vec![]; vector.size()];
        let vector_size = vector.size();
        for i in 0..vector_size {
            ids[i].push(vector.get(i).id.clone());
            values[i].push(vector.get(i).value.clone());
        }

        let new_id = create_construct_matrix_expr(name, ids);

        Self {
            id: new_id,
            value: values,
        }
    }

    /// Create a matrix from multiple vectors
    /// Each vector becomes a row in the resulting matrix
    pub fn from_vectors(name: &str, vectors: Vec<&Vector>) -> Self {
        // iterate over vectors, if their size is not equal, panic
        if vectors.is_empty() {
            panic!("Cannot create a matrix from an empty vector list");
        }
        let vector_size = vectors[0].size();
        for v in &vectors {
            if v.size() != vector_size {
                panic!("All vectors must have the same size to create a matrix");
            }
        }

        let ids: Vec<Vec<Identifier>> = vectors
            .iter()
            .map(|v| (0..v.size()).map(|i| v.get(i).id.clone()).collect())
            .collect();
        let new_id = create_construct_matrix_expr(name, ids);

        // Construct the matrix values from vector values
        let value: Vec<Vec<Real>> = vectors.iter().map(|v| v.value.clone()).collect();

        info!(
            "values: {:?}",
            value
                .iter()
                .map(|row| row.iter().map(|v| v.to_f64()).collect::<Vec<f64>>())
                .collect::<Vec<Vec<f64>>>()
        );
        Self { id: new_id, value }
    }

    /// Create a matrix by concatenating matrices horizontally
    pub fn from_matrices_horizontal(name: &str, matrices: Vec<&Matrix>) -> Self {
        let ids: Vec<Vec<Identifier>> = vec![matrices.iter().map(|m| m.id.clone()).collect()];
        let new_id = create_construct_matrix_expr(name, ids);

        // Concatenate matrices horizontally (assuming same number of rows)
        let value = if let Some(first) = matrices.first() {
            let rows = first.value.len();
            (0..rows)
                .map(|row_idx| {
                    matrices
                        .iter()
                        .flat_map(|m| {
                            if let Some(row) = m.value.get(row_idx) {
                                row.to_vec()
                            } else {
                                vec![]
                            }
                        })
                        .collect()
                })
                .collect()
        } else {
            vec![]
        };

        Self { id: new_id, value }
    }

    /// Create a matrix by concatenating matrices vertically
    pub fn from_matrices_vertical(name: &str, matrices: Vec<&Matrix>) -> Self {
        let ids: Vec<Vec<Identifier>> = matrices.iter().map(|m| vec![m.id.clone()]).collect();
        let new_id = create_construct_matrix_expr(name, ids);

        // Concatenate matrices vertically
        let value: Vec<Vec<Real>> = matrices
            .iter()
            .flat_map(|m| m.value.iter().cloned())
            .collect();

        Self { id: new_id, value }
    }

    /// Get the dimensions of the matrix (rows, cols)
    pub fn size(&self) -> (usize, usize) {
        let rows = self.value.len();
        if rows == 0 {
            return (0, 0);
        }
        let cols = self.value[0].len();
        (rows, cols)
    }

    /// Get a specific row as a Vector by index
    pub fn get_row(&self, row_index: usize) -> Vector {
        let new_id = create_index_matrix_row_expr(
            &format!("{}_row_{}", self.id.name, row_index),
            self.id.clone(),
            row_index,
        );

        let row_value = self
            .value
            .get(row_index)
            .expect("Row index out of bounds in Matrix row indexing");

        Vector {
            id: new_id,
            value: row_value.clone(),
        }
    }

    pub fn to_vector(&self) -> Vector {
        // if the column size is not 1, we cannot convert to a vector
        assert_eq!(
            self.size().1,
            1,
            "Cannot convert matrix to vector if it has more than one column"
        );

        // use get to convert all elements into a scalar
        let all_get = (0..self.size().0)
            .map(|i| self.get(i, 0))
            .collect::<Vec<Scalar>>();

        //println!("converting to vector with scalars: {:?}", all_get);

        Vector::from_scalars(&self.id.name, all_get.iter().collect())
    }

    /// TODO: DEPRECATE THIS IN FAVOR OF AT
    /// Get a specific element as a Scalar by row and column index
    pub fn get(&self, row_index: usize, col_index: usize) -> Scalar {
        let new_id = create_index_matrix_element_expr(
            &format!("{}_elem_{}_{}", self.id.name, row_index, col_index),
            self.id.clone(),
            row_index,
            col_index,
        );

        let row = self
            .value
            .get(row_index)
            .expect("Row index out of bounds in Matrix element indexing");
        let element_value = row
            .get(col_index)
            .expect("Column index out of bounds in Matrix element indexing");

        Scalar {
            id: new_id,
            value: element_value.clone(),
        }
    }

    pub fn at(&self, row_index: usize, col_index: usize) -> Scalar {
        self.get(row_index, col_index)
    }

    /// Get a specific element as f64 by row and column index
    pub fn get_f64(&self, row_index: usize, col_index: usize) -> Option<f64> {
        self.value
            .get(row_index)
            .and_then(|row| row.get(col_index))
            .map(|v| v.to_f64())
    }

    /// Get the name of the matrix
    pub fn get_name(&self) -> &str {
        &self.id.name
    }

    pub fn matmul(&self, other: &Matrix) -> Matrix {
        let (rows_a, cols_a) = self.size();
        let (rows_b, cols_b) = other.size();

        assert_eq!(
            cols_a, rows_b,
            "Matrix dimensions do not match for multiplication. Size A: {}x{}, Size B: {}x{}",
            rows_a, cols_a, rows_b, cols_b
        );

        // add that to intermediate representation
        //let new_id = create_binary_matrix_expr(
        //    &format!("{}_matmul_{}", self.id.name, other.id.name),
        //    self.id.clone(),
        //    other.id.clone(),
        //    OprBinary::MatMul,
        //);

        // create vectors from the rows of the first matrix and columns of the second matrix
        // then do dot product
        // compiler optimizations are simpler this way
        let mut first_matrix_rows: Vec<Vec<Scalar>> = vec![];
        for i in 0..rows_a {
            let row = (0..cols_a).map(|j| self.get(i, j)).collect::<Vec<Scalar>>();
            first_matrix_rows.push(row);
        }
        let mut second_matrix_cols: Vec<Vec<Scalar>> = vec![];
        for j in 0..cols_b {
            let col = (0..rows_b)
                .map(|i| other.get(i, j))
                .collect::<Vec<Scalar>>();
            second_matrix_cols.push(col);
        }

        // now create vectors
        let first_matrix_row_vectors: Vec<Vector> = first_matrix_rows
            .iter()
            .map(|row| Vector::from_scalars("row_vector", row.iter().collect()))
            .collect();
        let second_matrix_col_vectors: Vec<Vector> = second_matrix_cols
            .iter()
            .map(|col| Vector::from_scalars("col_vector", col.iter().collect()))
            .collect();

        // now do dot product for each pair
        let mut result_ids = vec![vec![Identifier::new_scalar("bos"); cols_b]; rows_a];
        for i in 0..rows_a {
            for j in 0..cols_b {
                let dot_product = first_matrix_row_vectors[i].dot(&second_matrix_col_vectors[j]);
                result_ids[i][j] = dot_product.id;
            }
        }

        let new_id = create_construct_matrix_expr(
            &format!("{}_matmul_{}", self.id.name, other.id.name),
            result_ids,
        );

        let mut result_values = vec![vec![Real::zero(); cols_b]; rows_a];

        //for i in 0..rows_a {
        //    for j in 0..cols_b {
        //        for k in 0..cols_a {
        //            result_values[i][j] += &self.value[i][k] * &other.value[k][j];
        //        }
        //    }
        //}
        // Clippy friendly version
        for (result_row, value_row) in result_values.iter_mut().zip(self.value.iter()) {
            for j in 0..cols_b {
                for (value, other_val_row) in value_row.iter().zip(other.value.iter()) {
                    result_row[j] += value * &other_val_row[j];
                }
            }
        }

        Matrix {
            id: new_id,
            value: result_values,
        }
    }

    pub fn matmul_vec(&self, other: &Vector) -> Vector {
        let (_, cols_a) = self.size();
        let rows_b = other.size();

        assert_eq!(
            cols_a, rows_b,
            "Matrix and Vector dimensions do not match for multiplication"
        );

        // create a matrix from the vector, then multiply
        let vector_as_matrix = Matrix::from_vector("vector_as_matrix", other);
        self.matmul(&vector_as_matrix).to_vector()
    }

    pub fn transpose(&self) -> Matrix {
        let (rows, cols) = self.size();
        let new_id = create_unary_matrix_expr(
            &format!("{}_transpose", self.id.name),
            self.id.clone(),
            OprUnary::Transpose,
        );

        let result_values: Vec<Vec<Real>> = (0..cols)
            .map(|col| (0..rows).map(|row| self.value[row][col].clone()).collect())
            .collect();

        Matrix {
            id: new_id,
            value: result_values,
        }
    }

    /// Set function will create a new matrix, but will set one element to the new scalar
    pub fn set(&mut self, index: (usize, usize), scalar: &Scalar) {
        let (row_index, col_index) = index;
        assert!(
            row_index < self.size().0 && col_index < self.size().1,
            "Index out of bounds in Matrix set operation, row: {}, col: {}, size: {:?}",
            row_index,
            col_index,
            self.size()
        );

        // collect all ids of each element
        let mut ids = vec![vec![]; self.size().0];
        for (i, id) in ids.iter_mut().enumerate() {
            for j in 0..self.size().1 {
                id.push(self.get(i, j).id.clone());
            }
        }

        // replace the specific element with the new scalar id
        ids[row_index][col_index] = scalar.id.clone();

        let mut values: Vec<Vec<Real>> = self.value.clone();

        // replace the specific element with the new scalar value
        values[row_index][col_index] = scalar.value.clone();

        let new_id = create_construct_matrix_expr(
            &format!("{}_set_{}_{}", self.id.name, row_index, col_index),
            ids,
        );

        self.id = new_id;
        self.value = values;
    }

    pub fn zero(row: usize, col: usize) -> Matrix {
        let new_id = create_matrix_constant(
            &format!("zero_matrix_{}_{}", row, col),
            vec![vec![Real::zero(); col]; row],
        );
        Matrix {
            id: new_id,
            value: vec![vec![Real::zero(); col]; row],
        }
    }

    pub fn to_f64(&self) -> Vec<Vec<f64>> {
        self.value
            .iter()
            .map(|row| row.iter().map(|v| v.to_f64()).collect())
            .collect()
    }
}

/// Macro to create a matrix from constant values, scalars, or other matrices
/// Usage:
///   Matrix!([[1.0, 2.0], [3.0, 4.0]]) - creates from constant 2D array
///   Matrix!([scalar1, scalar2; scalar3, scalar4]) - creates from scalars (semicolon separates rows)
///   Matrix!(matrix) - creates from another matrix
///   Matrix!(hcat: matrix1, matrix2, ...) - horizontal concatenation of matrices
///   Matrix!(vcat: matrix1, matrix2, ...) - vertical concatenation of matrices
///   Matrix!(from_vectors: vector1, vector2, ...) - creates from multiple vectors (each vector becomes a row)
#[macro_export]
macro_rules! Matrix {
    // Pattern for creating from multiple vectors
    (from_vectors: $($vector:expr),+ $(,)?) => {
        {
            $crate::types::matrix::Matrix::from_vectors("matrix", vec![$(&$vector),+])
        }
    };

    // Pattern for creating from one vector
    (from_vector: $vector:expr) => {
        {
            $crate::types::matrix::Matrix::from_vector("matrix", &$vector)
        }
    };

    // Pattern for creating from another matrix
    ($matrix:ident) => {
        {
            $crate::types::matrix::Matrix::from_matrix("matrix", &$matrix)
        }
    };

    // Pattern for horizontal concatenation
    (hcat: $($matrix:expr),+ $(,)?) => {
        {
            $crate::types::matrix::Matrix::from_matrices_horizontal("matrix", vec![$(&$matrix),+])
        }
    };

    // Pattern for vertical concatenation
    (vcat: $($matrix:expr),+ $(,)?) => {
        {
            $crate::types::matrix::Matrix::from_matrices_vertical("matrix", vec![$(&$matrix),+])
        }
    };

    // Pattern for creating from constant values (array literals)
    ([$($row:expr),+ $(,)?]) => {
        {
            $crate::Matrix::new("matrix", vec![$($row),+])
        }
    };

    // Pattern for creating from scalars using semicolon syntax
    ([$($($scalar:expr),+ $(,)?);+ $(;)?]) => {
        {
            let scalar_rows = vec![$(vec![$($scalar),+]),+];
            let scalars = scalar_rows.iter().map(|row| row.iter().collect()).collect();
            $crate::Matrix::from_scalars("matrix", scalars)
        }
    };

    // Fallback pattern for other expressions (assume constants)
    ($values:expr) => {
        {
            $crate::Matrix::new("matrix", $values)
        }
    };
}
