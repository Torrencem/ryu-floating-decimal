# Ryu Floating Decimal

This crate directly copies the internals of the [ryu rust crate](https://github.com/dtolnay/ryu), exposing some useful functions and types for more flexible float printing.

This crate exposes the functions `d2d` and `f2d`, which convert from `f64` to `FloatingDecimal64` and `f32` to `FloatingDecimal32` respectively. These floating decimals can be converted to strings in a custom way.

See [the original crate](https://github.com/dtolnay/ryu) for benchmarks and other information.

## License

Licensed under [Apache License, Version 2.0](https://github.com/torrencem/ryu-floating-decimal/blob/master/LICENSE-APACHE), copied from [the original crate](https://github.com/dtolnay/ryu)
