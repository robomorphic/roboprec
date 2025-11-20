# RoboPrec

A verification-aware compiler framework for generating numerically-guaranteed, high-performance code for robotics algorithms. RoboPrec enables safe deployment of compute-intensive kernels to embedded platforms by providing formal worst-case error bounds across mixed-precision datatypes.

## Overview

RoboPrec takes robotics algorithms written in Rust and generates optimized C code with **provable numerical accuracy guarantees**. It combines a Rust frontend with formal numerical analysis (via [Daisy](https://github.com/malyzajko/daisy)) to produce verified fixed-point or floating-point implementations.

**Key Results** (from our IEEE RA-L 2025 paper):
- Up to **122× faster** than `double` on embedded platforms (RP2040)
- **2.5×-30.4× better accuracy** than `float` using 32-bit fixed-point
- Competitive or faster than `double` on desktop CPUs for some kernels

## Features

- **Formal Guarantees**: Static analysis provides worst-case error bounds
- **High Performance**: Optimized fixed-point code for embedded systems
- **Mixed Precision**: Automatic or manual precision selection per variable
- **Robotics-Ready**: Built-in support for kinematics, dynamics (RNEA), and derivatives
- **Easy Integration**: Generates portable C code for any platform

## Installation

### Prerequisites

- Rust ([install via rustup](https://rustup.rs/))
- Scala (for Daisy numerical analyzer)
- GCC/G++ for C compilation

### Quick Start

```bash
git clone https://github.com/robomorphic/roboprec
cd roboprec

# Build RoboPrec
cargo build --release
```

## Usage

### Basic Example

```rust
use roboprec::*;

fn main() -> anyhow::Result<()> {
    // Define input with range
    let x = add_input_scalar("x", 
        (Real::from_f64(0.0), Real::from_f64(1.0)), 
        0.5);
    
    // Compute x²
    let mut result = &x * &x;
    
    // Register output
    register_scalar_output(&mut result, "x_squared");
    
    // Configure and analyze
    let config = Config {
        precision: Precision::Fixed { 
            total_bits: 32, 
            fractional_bits: -1  // Auto-optimize
        },
        output_dir: PathBuf::from("output/"),
    };
    
    analysis(config)?;
    Ok(())
}
```

This generates verified C code in `output/codegen/C/` with formal error bounds in `output/analysis_data/`.

### Precision Options

```rust
// Floating-point
Precision::Float64          // IEEE 754 double
Precision::Float32          // IEEE 754 float

// Fixed-point (auto-optimize)
Precision::Fixed { total_bits: 32, fractional_bits: -1 }

// Fixed-point (manual)
Precision::Fixed { total_bits: 32, fractional_bits: 16 }
```

### Command Line

```bash
# Run with specific precision
cargo run --release -- --precision Float64
cargo run --release -- --precision Fixed32
cargo run --release -- --precision Fixed16-8  # 16 total bits, 8 fractional
```

## Documentation

Build and view the API documentation:

```bash
cargo doc --open
```

For more information:
- **Website**: [robomorphic.github.io/roboprec](https://robomorphic.github.io/roboprec_website)
- **Examples**: See `src/algorithms/` for robotics algorithms, and `examples.rs` for how to use them.

## Supported Algorithms

- **Kinematics**: Forward kinematics for serial manipulators
- **Dynamics**: RNEA (Recursive Newton-Euler Algorithm)
- **Derivatives**: First-order RNEA derivatives for optimization
- **Robot Models**: RoArm-M2/M3, Indy7, Franka Panda (4-7 DOF)

## Citation

Bibtex is coming soon.

## License

See [LICENSE](LICENSE) file for details.

## Contact

- **Issues**: [GitHub Issues](https://github.com/robomorphic/roboprec/issues)


