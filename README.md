[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/ebacktrace-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/ebacktrace-rust)
[![docs.rs](https://docs.rs/ebacktrace/badge.svg)](https://docs.rs/ebacktrace)
[![crates.io](https://img.shields.io/crates/v/ebacktrace.svg)](https://crates.io/crates/ebacktrace)
[![Download numbers](https://img.shields.io/crates/d/ebacktrace.svg)](https://crates.io/crates/ebacktrace)
[![dependency status](https://deps.rs/crate/ebacktrace/0.3.0/status.svg)](https://deps.rs/crate/ebacktrace/0.3.0)


# `ebacktrace`
Welcome to `ebacktrace` 🎉

This crate implements a simple error wrapper which captures a backtrace upon creation and can carry an optional textual
description of the error.


## Example
```rust
use ebacktrace::define_error;

/// The error kind
#[derive(Debug, Copy, Clone)]
enum ErrorKind {
    MyErrorA,
    Testolope
}

// Define our custom error type
define_error!(Error);

/// A function that will always fail
fn will_fail() -> Result<(), Error<ErrorKind>> {
    Err(ErrorKind::Testolope)?
}

// Will panic with a nice fully-backtraced error
will_fail().unwrap();
```


## Features
This crate currently has one feature gate:
  - `derive_display` (enabled by default): Use the `Display`-trait for `Etrace<MyType>` using the `Debug` representation 
    of `MyType` (instead of the `Display` representation). This way you can pretty-print the underlying error types
    without the necessity to manually implement the `Display`-trait for them.
