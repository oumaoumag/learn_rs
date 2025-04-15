# rust-piscine / learn_rs

A collection of Rust exercises and implementations focusing on fundamental Rust concepts.

## Projects

### scalar
Basic arithmetic operations implementation with different numeric types:
- Sum (u8)
- Difference (i16)
- Product (i8)
- Quotient (f32)
- Remainder (f32)

### temperature_conv
Temperature conversion utilities:
- Fahrenheit to Celsius conversion
- Celsius to Fahrenheit conversion

### box_it
Rust boxing utilities and examples.

## Running Tests

Each project can be tested individually using Cargo:

```bash
cd scalar
cargo test

cd ../temperature_conv
cargo test

cd ../box_it
cargo test
```

## Project Structure
```
.
├── scalar/
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── temperature_conv/
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
└── box_it/
    ├── src/
    │   └── lib.rs
    └── Cargo.toml
```
