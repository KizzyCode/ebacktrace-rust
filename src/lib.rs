//! [![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
//! [![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
//! [![Travis CI](https://travis-ci.com/KizzyCode/ebacktrace-rust.svg?branch=master)](https://travis-ci.com/KizzyCode/ebacktrace-rust)
//! [![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/ebacktrace-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/ebacktrace-rust)
//! [![docs.rs](https://docs.rs/ebacktrace/badge.svg)](https://docs.rs/ebacktrace)
//! [![crates.io](https://img.shields.io/crates/v/ebacktrace.svg)](https://crates.io/crates/ebacktrace)
//! [![Download numbers](https://img.shields.io/crates/d/ebacktrace.svg)](https://crates.io/crates/ebacktrace)
//! [![dependency status](https://deps.rs/crate/ebacktrace/0.1.0/status.svg)](https://deps.rs/crate/ebacktrace/0.1.0)
//! 
//! 
//! # `ebacktrace`
//! Welcome to `ebacktrace` 🎉
//! 
//! This crate implements a simple error wrapper which captures a backtrace upon creation and can carry an optional textual
//! description of the error.
//! 
//! ## Example
//! ```should_panic
//! use ebacktrace::Etrace;
//! 
//! /// The error kind
//! #[derive(Debug, Copy, Clone)]
//! enum ErrorKind {
//!     MyErrorA,
//!     Testolope
//! }
//! 
//! /// A function that will always fail
//! fn will_fail() -> Result<(), Etrace<ErrorKind>> {
//!     Err(ErrorKind::Testolope)?
//! }
//! 
//! // Will panic with a nice fully-backtraced error
//! will_fail().unwrap();
//! ```
//! 
//! ## Features
//! This crate currently has two feature gates:
//!   - `force_backtrace` (enabled by default): If this feature is enabled, the crate uses `Backtrace::force_capture`
//!     (instead of `Backtrace::capture`) to always capture a backtrace regardless of whether `RUST_BACKTRACE` is set or
//!     not.
//!   - `derive_display` (enabled by default): Implements the `Display`-trait for `Etrace<MyType>` using the `Debug`
//!     representation of `MyType` (instead of the `Display` representation). This way you can pretty-print the underlying
//!     error types without the necessity to manually implement the `Display`-trait for them.

// FIXME: Remove once https://github.com/rust-lang/rust/pull/72981 has been stabilized
#![feature(backtrace)]

use std::{
    backtrace::Backtrace, borrow::Cow, error::Error, ops::Deref, sync::Arc,
    fmt::{ self, Debug, Display, Formatter }
};


/// A struct that wraps an error together with a backtrace and an optional description
pub struct Etrace<E> {
    err: E,
    desc: Cow<'static, str>,
    backtrace: Arc<Backtrace>
}
impl<E> Etrace<E> {
    /// Wraps an error `err`
    pub fn new(err: E) -> Self {
        let backtrace = (Self::capture_fn())();
        Self { err, desc: Cow::Borrowed(""), backtrace: Arc::new(backtrace) }
    }
    /// Wraps an error `err` together with a description `desc`
    pub fn with_str(err: E, desc: &'static str) -> Self {
        let backtrace = (Self::capture_fn())();
        Self { err, desc: Cow::Borrowed(desc), backtrace: Arc::new(backtrace) }
    }
    /// Wraps an error `err` together with a description `desc`
    pub fn with_string<S>(err: E, desc: S) -> Self where S: ToString {
        let backtrace = (Self::capture_fn())();
        Self { err, desc: Cow::Owned(desc.to_string()), backtrace: Arc::new(backtrace) }
    }

    /// The wrapped error
    pub const fn err(&self) -> &E {
        &self.err
    }
    /// The error description
    pub const fn desc(&self) -> &Cow<'static, str> {
        &self.desc
    }
    /// The backtrace
    pub fn backtrace(&self) -> &Backtrace {
        self.backtrace.as_ref()
    }

    /// Returns a function to create a backtrace
    ///
    /// This function exists to implement the `force_backtrace` feature
    #[inline]
    fn capture_fn() -> impl FnOnce() -> Backtrace {
        match cfg!(feature = "force_backtrace") {
            true => Backtrace::force_capture,
            false => Backtrace::capture
        }
    }
}
impl<E> Deref for Etrace<E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.err
    }
}
impl<E> From<E> for Etrace<E> {
    fn from(err: E) -> Self {
        Self::new(err)
    }
}
// Error
impl<E> Error for Etrace<E> where E: Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.err.source()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        Some(&self.backtrace)
    }
}
// Debug
impl<E> Debug for Etrace<E> where E: Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}
// Display
#[cfg(not(feature = "derive_display"))]
impl<E> Display for Etrace<E> where E: Display {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Write the error and description
        write!(f, "{}", &self.err)?;
        if !self.desc.is_empty() {
            write!(f, " ({})", &self.desc)?;
        }
        writeln!(f)?;

        // Print the backtrace
        write!(f, "{}", &self.backtrace)?;
        writeln!(f)?;
        Ok(())
    }
}
#[cfg(feature = "derive_display")]
impl<E> Display for Etrace<E> where E: Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Write the error and description
        write!(f, "{:?}", &self.err)?;
        if !self.desc.is_empty() {
            write!(f, " ({})", &self.desc)?;
        }
        writeln!(f)?;

        // Print the backtrace
        write!(f, "{}", &self.backtrace)?;
        writeln!(f)?;
        Ok(())
    }
}
// Default
impl<E> Default for Etrace<E> where E: Default {
    fn default() -> Self {
        Self::new(E::default())
    }
}
// Clone
impl<E> Clone for Etrace<E> where E: Clone {
    fn clone(&self) -> Self {
        Self { err: self.err.clone(), desc: self.desc.clone(), backtrace: self.backtrace.clone() }
    }
}
