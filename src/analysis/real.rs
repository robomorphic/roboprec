use std::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rug::Integer as RugInteger;
use rug::Rational;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Integer {
    value: RugInteger,
}

impl Integer {
    pub fn to_i64(&self) -> i64 {
        self.value.to_i64().unwrap()
    }

    pub fn from_u32(n: u32) -> Self {
        Self {
            value: RugInteger::from(n),
        }
    }

    pub fn one() -> Self {
        Self {
            value: RugInteger::from(1),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    pub fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    pub fn bits(&self) -> u32 {
        self.value.significant_bits()
    }
}

// Implement Sub for &Real - &Real
impl Sub for &Integer {
    type Output = Integer;

    fn sub(self, other: &Integer) -> Integer {
        Integer {
            value: RugInteger::from(&self.value - &other.value),
        }
    }
}

// Implement BitAnd for &Integer & &Integer
impl BitAnd for &Integer {
    type Output = Integer;

    fn bitand(self, other: &Integer) -> Integer {
        Integer {
            value: RugInteger::from(&self.value & &other.value),
        }
    }
}

// Implement BitAnd for Integer & Integer
impl BitAnd for Integer {
    type Output = Integer;

    fn bitand(self, other: Integer) -> Integer {
        Integer {
            value: self.value & other.value,
        }
    }
}

// Implement BitAnd for Integer & &Integer
impl BitAnd<&Integer> for Integer {
    type Output = Integer;

    fn bitand(self, other: &Integer) -> Integer {
        Integer {
            value: self.value & &other.value,
        }
    }
}

// Implement BitAnd for &Integer & Integer
impl BitAnd<Integer> for &Integer {
    type Output = Integer;

    fn bitand(self, other: Integer) -> Integer {
        Integer {
            value: &self.value & other.value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Real {
    value: Rational,
}

impl Real {
    pub fn zero() -> Self {
        Self {
            value: Rational::new(),
        }
    }

    pub fn one() -> Self {
        Self {
            value: Rational::from(1),
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.value.to_f64()
    }

    pub fn to_integer(&self) -> Integer {
        Integer {
            value: self.value.numer().clone() / self.value.denom().clone(),
        }
    }

    pub fn from_i64(n: i64) -> Self {
        Self {
            value: Rational::from(n),
        }
    }

    pub fn from_u64(n: u64) -> Self {
        Self {
            value: Rational::from(n),
        }
    }

    pub fn from_usize(n: usize) -> Self {
        Self {
            value: Rational::from(n),
        }
    }

    pub fn from_f64(n: f64) -> Self {
        Self {
            value: Rational::from_f64(n).unwrap(),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            value: self.value.clone().abs(),
        }
    }

    pub fn numer(&self) -> Integer {
        Integer {
            value: self.value.numer().clone(),
        }
    }

    pub fn denom(&self) -> Integer {
        Integer {
            value: self.value.denom().clone(),
        }
    }

    pub fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    pub fn pow(&self, exp: i32) -> Self {
        use rug::ops::Pow;
        Self {
            value: self.value.clone().pow(exp),
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            value: self.value.clone().floor(),
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        if self < other {
            self.clone()
        } else {
            other.clone()
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        if self > other {
            self.clone()
        } else {
            other.clone()
        }
    }

    // Repeat daisy implementation of nearest Integer
    //def roundToInt: Int = {
    //    if (n >= zeroBigInt) { // positive number
    //        if (this - Rational(this.IntegerPart) < Rational(1L, 2L)) {
    //        this.IntegerPart
    //        } else {
    //        this.IntegerPart + 1
    //        }
    //    } else {
    //        if (this - Rational(this.IntegerPart) > Rational(1L, 2L)) {
    //        this.IntegerPart
    //        } else {
    //        this.IntegerPart - 1
    //        }
    //    }
    //}
    pub fn nearest_integer(&self) -> i64 {
        let integer_part = self.value.clone().floor();
        let frac_part = Rational::from(&self.value - &integer_part);
        let half = Rational::from((1, 2));
        let neg_half = Rational::from((-1, 2));

        let integer_value =
            integer_part.numer().to_i64().unwrap() / integer_part.denom().to_i64().unwrap();

        if self.value.is_positive() {
            if frac_part < half {
                integer_value
            } else {
                integer_value + 1
            }
        } else if frac_part > neg_half {
            integer_value
        } else {
            integer_value - 1
        }
    }
}

// Implement Add for &Real + &Real
impl Add for &Real {
    type Output = Real;

    fn add(self, other: &Real) -> Real {
        Real {
            value: Rational::from(&self.value + &other.value),
        }
    }
}

// Implement Sub for &Real - &Real
impl Sub for &Real {
    type Output = Real;

    fn sub(self, other: &Real) -> Real {
        Real {
            value: Rational::from(&self.value - &other.value),
        }
    }
}

// Implement Mul for &Real * &Real
impl Mul for &Real {
    type Output = Real;

    fn mul(self, other: &Real) -> Real {
        Real {
            value: Rational::from(&self.value * &other.value),
        }
    }
}

// Implement Div for &Real / &Real
impl Div for &Real {
    type Output = Real;

    fn div(self, other: &Real) -> Real {
        Real {
            value: Rational::from(&self.value / &other.value),
        }
    }
}

// Implement Add for Real + Real
impl Add for Real {
    type Output = Real;

    fn add(self, other: Real) -> Real {
        Real {
            value: self.value + other.value,
        }
    }
}

// Implement Add for Real + &Real
impl Add<&Real> for Real {
    type Output = Real;

    fn add(self, other: &Real) -> Real {
        Real {
            value: self.value + &other.value,
        }
    }
}

// Implement Add for &Real + Real
impl Add<Real> for &Real {
    type Output = Real;

    fn add(self, other: Real) -> Real {
        Real {
            value: &self.value + other.value,
        }
    }
}

// Implement Sub for Real - Real
impl Sub for Real {
    type Output = Real;

    fn sub(self, other: Real) -> Real {
        Real {
            value: self.value - other.value,
        }
    }
}

// Implement Sub for Real - &Real
impl Sub<&Real> for Real {
    type Output = Real;

    fn sub(self, other: &Real) -> Real {
        Real {
            value: self.value - &other.value,
        }
    }
}

// Implement Sub for &Real - Real
impl Sub<Real> for &Real {
    type Output = Real;

    fn sub(self, other: Real) -> Real {
        Real {
            value: &self.value - other.value,
        }
    }
}

// Implement Mul for Real * Real
impl Mul for Real {
    type Output = Real;

    fn mul(self, other: Real) -> Real {
        Real {
            value: self.value * other.value,
        }
    }
}

// Implement Mul for Real * &Real
impl Mul<&Real> for Real {
    type Output = Real;

    fn mul(self, other: &Real) -> Real {
        Real {
            value: self.value * &other.value,
        }
    }
}

// Implement Mul for &Real * Real
impl Mul<Real> for &Real {
    type Output = Real;

    fn mul(self, other: Real) -> Real {
        Real {
            value: &self.value * other.value,
        }
    }
}

// Implement Div for Real / Real
impl Div for Real {
    type Output = Real;

    fn div(self, other: Real) -> Real {
        Real {
            value: self.value / other.value,
        }
    }
}

// Implement Div for Real / &Real
impl Div<&Real> for Real {
    type Output = Real;

    fn div(self, other: &Real) -> Real {
        Real {
            value: self.value / &other.value,
        }
    }
}

// Implement Div for &Real / Real
impl Div<Real> for &Real {
    type Output = Real;

    fn div(self, other: Real) -> Real {
        Real {
            value: &self.value / other.value,
        }
    }
}

// Implement AddAssign for Real += Real
impl AddAssign for Real {
    fn add_assign(&mut self, other: Real) {
        self.value += other.value;
    }
}

// Implement AddAssign for Real += &Real
impl AddAssign<&Real> for Real {
    fn add_assign(&mut self, other: &Real) {
        self.value += &other.value;
    }
}

// Implement SubAssign for Real -= Real
impl SubAssign for Real {
    fn sub_assign(&mut self, other: Real) {
        self.value -= other.value;
    }
}

// Implement SubAssign for Real -= &Real
impl SubAssign<&Real> for Real {
    fn sub_assign(&mut self, other: &Real) {
        self.value -= &other.value;
    }
}

// Implement MulAssign for Real *= Real
impl MulAssign for Real {
    fn mul_assign(&mut self, other: Real) {
        self.value *= other.value;
    }
}

// Implement MulAssign for Real *= &Real
impl MulAssign<&Real> for Real {
    fn mul_assign(&mut self, other: &Real) {
        self.value *= &other.value;
    }
}

// Implement DivAssign for Real /= Real
impl DivAssign for Real {
    fn div_assign(&mut self, other: Real) {
        self.value /= other.value;
    }
}

// Implement DivAssign for Real /= &Real
impl DivAssign<&Real> for Real {
    fn div_assign(&mut self, other: &Real) {
        self.value /= &other.value;
    }
}

// Implement Neg for -Real
impl Neg for Real {
    type Output = Real;

    fn neg(self) -> Real {
        Real { value: -self.value }
    }
}

// Implement Neg for -&Real
impl Neg for &Real {
    type Output = Real;

    fn neg(self) -> Real {
        Real {
            value: Rational::from(-&self.value),
        }
    }
}
