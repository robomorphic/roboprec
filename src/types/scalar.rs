use crate::{
    analysis::real::Real,
    ir::{
        expr::{
            OprUnary, create_construct_scalar_expr, create_scalar_constant,
            create_unary_scalar_expr,
        },
        identifier::Identifier,
    },
};

#[derive(Clone, Debug)]
pub struct Scalar {
    pub id: Identifier, // Name of the scalar, guaranteed to be unique over the whole program
    pub value: Real,    // Value of the scalar, computed with default values
}

impl Scalar {
    /// This function will be only used when the scalar is constant.
    pub fn new(name: &str, value: f64) -> Self {
        let rational_value = Real::from_f64(value);
        let new_id = create_scalar_constant(name, Real::from_f64(value));
        Self {
            id: new_id,
            value: rational_value,
        }
    }

    pub fn new_rational(name: &str, value: Real) -> Self {
        let new_id = create_scalar_constant(name, value.clone());
        Self { id: new_id, value }
    }

    // This function simply redefines a new value to this scalar.
    pub fn define(&mut self, new_name: String) -> Self {
        let new_id = create_unary_scalar_expr(&new_name, self.id.clone(), OprUnary::AssignNoOpt);
        Self {
            id: new_id,
            value: self.value.clone(),
        }
    }

    pub fn sin(&self) -> Self {
        let new_id = create_unary_scalar_expr(
            format!("{}_sin", self.id.name).as_str(),
            self.id.clone(),
            OprUnary::Sine,
        );
        // Compute the sine value using f64 for simplicity
        let sine_value = self.value.to_f64().sin();
        let rational_value = Real::from_f64(sine_value);
        Self {
            id: new_id,
            value: rational_value,
        }
    }

    pub fn cos(&self) -> Self {
        let new_id = create_unary_scalar_expr(
            format!("{}_cos", self.id.name).as_str(),
            self.id.clone(),
            OprUnary::Cosine,
        );
        // Compute the cosine value using f64 for simplicity
        let cosine_value = self.value.to_f64().cos();
        let rational_value = Real::from_f64(cosine_value);
        Self {
            id: new_id,
            value: rational_value,
        }
    }

    pub fn value_f64(&self) -> f64 {
        self.value.to_f64()
    }

    /// Create a scalar from another scalar using the Construct operation
    pub fn from_scalar(name: &str, scalar: &Scalar) -> Self {
        let id = scalar.id.clone();
        let new_id = create_construct_scalar_expr(name, id);

        // Use the input scalar's value
        let value = scalar.value.clone();

        Self { id: new_id, value }
    }

    /// Generic constructor that accepts either a reference to a `Scalar` or a reference to an
    /// f64; implemented via the `IntoScalarValue` trait below. This lets callers write a single
    /// call that accepts both kinds of inputs.
    pub fn from_any<T: IntoScalarValue>(name: &str, t: T) -> Self {
        t.into_scalar_with_name(name)
    }
}

/// Macro to create a scalar from a constant value or from another scalar
/// Usage:
///   Scalar!(3.14) - creates from constant with auto-generated name
///   Scalar!(other_scalar) - creates from another scalar with auto-generated name
#[macro_export]
macro_rules! Scalar {
    // Pattern for creating from a constant value with auto-generated name
    ($value:literal) => {{ $crate::types::scalar::Scalar::new("scalar", $value) }};

    // Pattern for creating from another scalar with auto-generated name
    ($scalar:expr) => {{ $crate::types::scalar::Scalar::from_any("scalar", &$scalar) }};
}

// TODO: make a similar change to vector and matrix macros
/// Helper trait used by `from_any` to allow passing either `&Scalar` or `&f64`.
pub trait IntoScalarValue {
    fn into_scalar_with_name(self, name: &str) -> Scalar;
}

impl IntoScalarValue for &Scalar {
    fn into_scalar_with_name(self, name: &str) -> Scalar {
        Scalar::from_scalar(name, self)
    }
}

impl IntoScalarValue for &f64 {
    fn into_scalar_with_name(self, name: &str) -> Scalar {
        Scalar::new(name, *self)
    }
}
