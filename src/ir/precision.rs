#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precision {
    Fixed {
        total_bits: i32,
        fractional_bits: i32,
    },
    Float32,
    Float64,
}
