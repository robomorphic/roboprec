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

/// A scalar value with tracked operations for numerical analysis.
///
/// `Scalar` represents a single numerical value that RoboPrec tracks through
/// your computation. All operations on scalars build an expression tree that
/// is later analyzed for range and error bounds.
///
/// # Examples
///
/// ```rust
/// use roboprec::*;
///
/// // Create from constant
/// let x = Scalar!(2.0);
///
/// // Arithmetic operations
/// let y = &x * &x;  // x²
/// let z = &y + &Scalar!(1.0);  // x² + 1
/// ```
#[derive(Clone, Debug)]
pub struct Scalar {
    /// Unique identifier for this scalar in the program
    pub id: Identifier,
    /// Computed value using default inputs (for validation)
    pub value: Real,
}

impl Scalar {
    /// Creates a new scalar from a constant f64 value.
    ///
    /// This is typically used via the [`Scalar!`] macro rather than directly.
    ///
    /// # Arguments
    /// * `name` - Name prefix for the scalar (auto-generated unique name)
    /// * `value` - Constant numerical value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use roboprec::*;
    ///
    /// let pi = Scalar::new("pi", 3.14159);
    /// let two = Scalar!(2.0);  // Preferred: use macro
    /// ```
    pub fn new(name: &str, value: f64) -> Self {
        let rational_value = Real::from_f64(value);
        let new_id = create_scalar_constant(name, Real::from_f64(value));
        Self {
            id: new_id,
            value: rational_value,
        }
    }

    /// Creates a new scalar from an arbitrary-precision rational value.
    ///
    /// Used internally when exact rational arithmetic is needed.
    pub fn new_rational(name: &str, value: Real) -> Self {
        let new_id = create_scalar_constant(name, value.clone());
        Self { id: new_id, value }
    }

    /// Redefines the scalar with a new name (creates an assignment operation).
    ///
    /// This creates a new scalar that is identical to the current one but with
    /// a different name in the expression tree.
    pub fn define(&mut self, new_name: String) -> Self {
        let new_id = create_unary_scalar_expr(&new_name, self.id.clone(), OprUnary::AssignNoOpt);
        Self {
            id: new_id,
            value: self.value.clone(),
        }
    }

    /// Returns the scalar's value as an f64 for testing/validation.
    ///
    /// Note: This is the value computed with default inputs, not the analyzed result.
    pub fn value_f64(&self) -> f64 {
        self.value.to_f64()
    }

    /// Creates a scalar from another scalar (identity operation).
    ///
    /// Used internally for constructing scalars from existing values.
    pub fn from_scalar(name: &str, scalar: &Scalar) -> Self {
        let id = scalar.id.clone();
        let new_id = create_construct_scalar_expr(name, id);

        // Use the input scalar's value
        let value = scalar.value.clone();

        Self { id: new_id, value }
    }

    /// Generic constructor accepting either a `Scalar` reference or an f64 reference.
    ///
    /// This enables flexible construction via the [`Scalar!`] macro.
    pub fn from_any<T: IntoScalarValue>(name: &str, t: T) -> Self {
        t.into_scalar_with_name(name)
    }
}

/// Creates a scalar from a constant value or another scalar.
///
/// This is the preferred way to create scalars in RoboPrec code.
///
/// # Examples
///
/// ```rust
/// use roboprec::*;
///
/// // From constant
/// let x = Scalar!(2.0);
/// let pi = Scalar!(3.14159);
///
/// // From another scalar
/// let y = Scalar!(x);
/// ```
#[macro_export]
macro_rules! Scalar {
    // Pattern for creating from a constant value with auto-generated name
    ($value:literal) => {{ $crate::Scalar::new("scalar", $value) }};

    // Pattern for creating from another scalar with auto-generated name
    ($scalar:expr) => {{ $crate::Scalar::from_any("scalar", &$scalar) }};
}

/// Helper trait for flexible scalar construction.
///
/// Allows [`Scalar::from_any`] to accept either `&Scalar` or `&f64`.
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
