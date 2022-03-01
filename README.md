[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/ebacktrace-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/ebacktrace-rust)
[![docs.rs](https://docs.rs/ebacktrace/badge.svg)](https://docs.rs/ebacktrace)
[![crates.io](https://img.shields.io/crates/v/ebacktrace.svg)](https://crates.io/crates/ebacktrace)
[![Download numbers](https://img.shields.io/crates/d/ebacktrace.svg)](https://crates.io/crates/ebacktrace)
[![dependency status](https://deps.rs/crate/ebacktrace/0.5.0/status.svg)](https://deps.rs/crate/ebacktrace/0.5.0)


# `ebacktrace`
Welcome to `ebacktrace` 🎉

This crate implements a simple error wrapper which captures a backtrace upon creation and can carry an optional textual
description of the error.


## Example
```rust
use ebacktrace::define_error;
use std::fmt::{ self, Display, Formatter };

/// The error kind
#[derive(Debug, Copy, Clone)]
enum ErrorKind {
    MyErrorA,
    Testolope
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
// Define our custom error type
define_error!(Error);

/// A function that will always fail
fn will_fail() -> Result<(), Error<ErrorKind>> {
    Err(ErrorKind::Testolope)?
}

// Will panic with a nice error
if let Err(e) = will_fail() {
    eprintln!("Error: {:?}", e);
    panic!("Fatal error")
}
```


## Features
This crate currently has one feature gate:
  - `force_backtrace` (disabled by default): If `force_backtrace` is enable, the backtrace is always captured,
    regardless whether `RUST_BACKTRACE` is set or not.
