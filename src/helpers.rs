use std::{f64::consts::PI, iter::Iterator};

pub fn sin_extremes(a: f64, b: f64) -> (f64, f64) {
    let mut a = a;
    let mut b = b;
    // Ensure a is the lower bound.
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    // Candidate values from endpoints.
    let mut sin_candidates = vec![a.sin(), b.sin()];

    // Helper function: find k integer such that candidate = offset + period * k lies in [a, b]
    // period is 2π.
    let period = 2.0 * PI;

    // For sine:
    // Sine maximum is 1 at π/2 + 2πk,
    // Sine minimum is -1 at 3π/2 + 2πk.
    let mut add_sin_candidates = |offset: f64| {
        // k_min is the smallest integer k such that candidate >= a
        let k_min = ((a - offset) / period).ceil() as i64;
        let mut k = k_min;
        loop {
            let candidate = offset + period * (k as f64);
            if candidate > b {
                break;
            }
            // The candidate sine value is known: either 1 or -1.
            // But we could compute candidate.sin() for robustness.
            sin_candidates.push(candidate.sin());
            k += 1;
        }
    };
    // Check at π/2 for max and 3π/2 for min.
    add_sin_candidates(PI / 2.0);
    add_sin_candidates(3.0 * PI / 2.0);

    // Determine the minimum and maximum values for sine and cosine.
    let min_sin = sin_candidates.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_sin = sin_candidates
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    (min_sin, max_sin)
}

pub fn cos_extremes(a: f64, b: f64) -> (f64, f64) {
    let mut a = a;
    let mut b = b;
    // Ensure a is the lower bound.
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    // Candidate values from endpoints.
    let mut cos_candidates = vec![a.cos(), b.cos()];

    // Helper function: find k integer such that candidate = offset + period * k lies in [a, b]
    // period is 2π.
    let period = 2.0 * PI;

    // For cosine:
    // Cosine maximum is 1 at 0 + 2πk,
    // Cosine minimum is -1 at π + 2πk.
    let mut add_cos_candidates = |offset: f64| {
        let k_min = ((a - offset) / period).ceil() as i64;
        let mut k = k_min;
        loop {
            let candidate = offset + period * (k as f64);
            if candidate > b {
                break;
            }
            cos_candidates.push(candidate.cos());
            k += 1;
        }
    };
    add_cos_candidates(0.0);
    add_cos_candidates(PI);

    let min_cos = cos_candidates.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_cos = cos_candidates
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    (min_cos, max_cos)
}
