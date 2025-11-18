use crate::{
    analysis::real::Real,
    ir::{
        expr::{
            OprBinary, OprUnary, create_binary_scalar_expr, create_binary_vector_expr,
            create_construct_vector_expr, create_index_vector_expr, create_unary_vector_expr,
            create_vector_constant,
        },
        identifier::Identifier,
    },
    types::scalar::Scalar,
};

pub struct Vector {
    pub id: Identifier, // Name of the vector, guaranteed to be unique over the whole program
    pub value: Vec<Real>, // Values of the vector, computed with default values
}

impl Vector {
    /// Create a vector from f64 values
    pub fn new(name: &str, values: Vec<f64>) -> Self {
        let rational_values: Vec<Real> = values.into_iter().map(Real::from_f64).collect();
        Self::new_rational(name, rational_values)
    }

    /// Create a vector from Real values
    pub fn new_rational(name: &str, values: Vec<Real>) -> Self {
        let new_id = create_vector_constant(name, values.clone());
        Self {
            id: new_id,
            value: values,
        }
    }

    /// Redefine the vector with a new name
    pub fn define(&mut self, new_name: String) -> Self {
        let new_id = create_unary_vector_expr(&new_name, self.id.clone(), OprUnary::AssignNoOpt);
        Self {
            id: new_id,
            value: self.value.clone(),
        }
    }

    pub fn sin(&self) -> Self {
        let new_id = create_unary_vector_expr(
            format!("{}_sin", self.id.name).as_str(),
            self.id.clone(),
            OprUnary::Sine,
        );
        let sine_values: Vec<Real> = self
            .value
            .iter()
            .map(|v| {
                let sine_value = v.to_f64().sin();
                Real::from_f64(sine_value)
            })
            .collect();
        Self {
            id: new_id,
            value: sine_values,
        }
    }

    pub fn cos(&self) -> Self {
        let new_id = create_unary_vector_expr(
            format!("{}_cos", self.id.name).as_str(),
            self.id.clone(),
            OprUnary::Cosine,
        );
        let cosine_values: Vec<Real> = self
            .value
            .iter()
            .map(|v| {
                let cosine_value = v.to_f64().cos();
                Real::from_f64(cosine_value)
            })
            .collect();
        Self {
            id: new_id,
            value: cosine_values,
        }
    }

    /// Get vector values as f64
    pub fn value_f64(&self) -> Vec<f64> {
        self.value.iter().map(|v| v.to_f64()).collect()
    }

    /// Create a vector from scalars
    pub fn from_scalars(name: &str, scalars: Vec<&Scalar>) -> Self {
        let ids: Vec<Identifier> = scalars.iter().map(|s| s.id.clone()).collect();
        let new_id = create_construct_vector_expr(name, ids);

        let values: Vec<Real> = scalars.iter().map(|s| s.value.clone()).collect();
        Self {
            id: new_id,
            value: values,
        }
    }

    /// Create a vector from another vector
    pub fn from_vector(name: &str, vector: &Vector) -> Self {
        let new_id = create_vector_constant(name, vector.value.clone());
        Self {
            id: new_id,
            value: vector.value.clone(),
        }
    }

    /// Get the length of the vector
    pub fn size(&self) -> usize {
        self.value.len()
    }

    /// TODO: DEPRECATE THIS IN FAVOR OF AT
    /// Get a specific element by index
    pub fn get(&self, index: usize) -> Scalar {
        let size = self.size();
        assert!(
            index < size,
            "Index {} out of bounds in Vector get operation, with size {}",
            index,
            size
        );
        let new_id = create_index_vector_expr(
            &format!("{}_index_{}", self.id.name, index),
            self.id.clone(),
            index,
        );

        let curr_value = self
            .value
            .get(index)
            .expect("Index out of bounds in Vector indexing");

        Scalar {
            id: new_id,
            value: curr_value.clone(),
        }
    }

    pub fn at(&self, index: usize) -> Scalar {
        self.get(index)
    }

    /// Get a specific element as f64 by index
    pub fn get_f64(&self, index: usize) -> Option<f64> {
        self.value.get(index).map(|v| v.to_f64())
    }

    /// Set an element at a specific index, creating a new vector
    pub fn set(&mut self, index: usize, value: f64) {
        assert!(
            index < self.value.len(),
            "Index out of bounds in Vector set operation"
        );

        // Create scalars for all elements
        let mut scalars = Vec::new();
        for i in 0..self.size() {
            if i == index {
                // Create new scalar for the value being set
                let new_scalar = Scalar::new(&format!("{}_set_val_{}", self.id.name, i), value);
                scalars.push(new_scalar);
            } else {
                // Get existing element as scalar
                scalars.push(self.get(i));
            }
        }

        // Create new vector from scalars
        let scalar_refs: Vec<&Scalar> = scalars.iter().collect();
        let new_vector = Vector::from_scalars(&self.id.name, scalar_refs);

        // Update this vector
        self.id = new_vector.id;
        self.value = new_vector.value;
    }

    /// Push a new element to the end of the vector, creating a new vector
    pub fn push_f64(&mut self, value: f64) {
        // Create scalars for all existing elements
        let mut scalars = Vec::new();
        for i in 0..self.size() {
            scalars.push(self.get(i));
        }

        // Add new scalar for the pushed value
        let new_scalar = Scalar::new(&format!("{}_push_val", self.id.name), value);
        scalars.push(new_scalar);

        // Create new vector from scalars
        let scalar_refs: Vec<&Scalar> = scalars.iter().collect();
        let new_vector = Vector::from_scalars(&self.id.name, scalar_refs);

        // Update this vector
        self.id = new_vector.id;
        self.value = new_vector.value;
    }

    /// Pop the last element from the vector, creating a new vector
    pub fn pop(&mut self) -> Real {
        assert!(!self.value.is_empty(), "Cannot pop from an empty vector");

        // Store the value that will be popped
        let popped_value = self.value.last().cloned().unwrap();

        // Create scalars for all elements except the last one
        let mut scalars = Vec::new();
        for i in 0..(self.size() - 1) {
            scalars.push(self.get(i));
        }

        // Create new vector from scalars (empty vector if no elements left)
        if scalars.is_empty() {
            // Create empty vector
            let new_vector = Vector::new(&self.id.name, vec![]);
            self.id = new_vector.id;
            self.value = new_vector.value;
        } else {
            let scalar_refs: Vec<&Scalar> = scalars.iter().collect();
            let new_vector = Vector::from_scalars(&self.id.name, scalar_refs);

            // Update this vector
            self.id = new_vector.id;
            self.value = new_vector.value;
        }

        popped_value
    }

    /// Get the name of the vector
    pub fn get_name(&self) -> &str {
        &self.id.name
    }

    /// Dot product with another vector
    pub fn dot(&self, other: &Vector) -> Scalar {
        assert_eq!(
            self.size(),
            other.size(),
            "Vectors must be of the same size for dot product"
        );

        let new_id = create_binary_scalar_expr(
            &format!("{}_dot_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Dot,
        );

        let dot_product = self
            .value
            .iter()
            .zip(other.value.iter())
            .map(|(a, b)| a * b)
            .fold(Real::zero(), |acc, x| acc + x);

        Scalar {
            id: new_id,
            value: dot_product,
        }
    }

    /// Cross product with another vector (only for 3D vectors)
    pub fn cross(&self, other: &Vector) -> Vector {
        assert_eq!(
            self.size(),
            3,
            "Cross product is only defined for 3D vectors"
        );
        assert_eq!(
            other.size(),
            3,
            "Cross product is only defined for 3D vectors"
        );

        let new_id = create_binary_vector_expr(
            &format!("{}_cross_{}", self.id.name, other.id.name),
            self.id.clone(),
            other.id.clone(),
            OprBinary::Cross,
            3, // cross product always outputs a 3 size vector
        );

        let cross_product = vec![
            self.value[1].clone() * other.value[2].clone()
                - self.value[2].clone() * other.value[1].clone(),
            self.value[2].clone() * other.value[0].clone()
                - self.value[0].clone() * other.value[2].clone(),
            self.value[0].clone() * other.value[1].clone()
                - self.value[1].clone() * other.value[0].clone(),
        ];

        Vector {
            id: new_id,
            value: cross_product,
        }
    }
}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            value: self.value.clone(),
        }
    }
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("id", &self.id)
            .field("value", &self.value_f64())
            .finish()
    }
}

/// Convenience macro for creating vectors
/// Usage:
///   Vector![1.0, 2.0, 3.0] - creates vector from f64 constants
///   Vector!([1.0, 2.0, 3.0]) - creates vector from f64 constants with parentheses
///   Vector!([scalar1, scalar2, scalar3;]) - creates vector from scalars with semicolon
#[macro_export]
macro_rules! Vector {
    // Pattern for parentheses syntax with scalars (semicolon at end to distinguish)
    ([$($scalar:expr),+ $(,)?;]) => {
        {
            $crate::types::vector::Vector::from_scalars("vector", vec![$(&$scalar),+])
        }
    };

    // Pattern for parentheses syntax with f64 values
    ([$($val:expr),+ $(,)?]) => {
        {
            $crate::types::vector::Vector::new("vector", vec![$($val),+])
        }
    };

    // Pattern for bracket syntax
    [$($val:expr),+ $(,)?] => {
        {
            $crate::types::vector::Vector::new("vector", vec![$($val),+])
        }
    };

    // Fallback pattern for other expressions
    ($values:expr) => {
        {
            $crate::types::vector::Vector::new("vector", $values)
        }
    };
}

/// Macro for creating vectors from scalars
#[macro_export]
macro_rules! vector_from_scalars {
    [$($scalar:expr),+ $(,)?] => {
        {
            $crate::types::vector::Vector::from_scalars("vector", vec![$(&$scalar),+])
        }
    };
}
