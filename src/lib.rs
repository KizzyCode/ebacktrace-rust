//! [![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
//! [![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
//! [![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/ebacktrace-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/ebacktrace-rust)
//! [![docs.rs](https://docs.rs/ebacktrace/badge.svg)](https://docs.rs/ebacktrace)
//! [![crates.io](https://img.shields.io/crates/v/ebacktrace.svg)](https://crates.io/crates/ebacktrace)
//! [![Download numbers](https://img.shields.io/crates/d/ebacktrace.svg)](https://crates.io/crates/ebacktrace)
//! [![dependency status](https://deps.rs/crate/ebacktrace/0.3.0/status.svg)](https://deps.rs/crate/ebacktrace/0.3.0)
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
//! use ebacktrace::define_error;
//! use std::fmt::{ self, Display, Formatter };
//! 
//! /// The error kind
//! #[derive(Debug, Copy, Clone)]
//! enum ErrorKind {
//!     MyErrorA,
//!     Testolope
//! }
//! impl Display for ErrorKind {
//!     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//!         write!(f, "{:#?}", self)
//!     }
//! }
//! // Define our custom error type
//! define_error!(Error);
//! 
//! /// A function that will always fail
//! fn will_fail() -> Result<(), Error<ErrorKind>> {
//!     Err(ErrorKind::Testolope)?
//! }
//! 
//! // Will panic with a nice error
//! if let Err(e) = will_fail() {
//!     eprintln!("Error: {:?}", e);
//!     panic!("Fatal error")
//! }
//! ```
//! 
//! ## Features
//! This crate currently has two feature gates:
//!   - `derive_display` (enabled by default): Use the `Display`-trait for `Etrace<MyType>` using the `Debug`
//!     representation of `MyType` (instead of the `Display` representation). This way you can pretty-print the underlying
//!     error types without the necessity to manually implement the `Display`-trait for them.
//!   - `force_backtrace` (disabled by default): If `force_backtrace` is enable, the backtrace is always captured,
//!     regardless whether `RUST_BACKTRACE` is set or not.


/// Implements a backtrace drop-in replacement until `$crate::backtrace::Backtrace` becomes stable
#[doc(hidden)]
pub mod backtrace;


/// Defines a custom error generic `$name<E>` where `E` is an arbitrary payload type
///
/// _Note:_ We use a macro to define a new type so that crates can easily implement stuff like `From<T>` for their errors
/// which would not be possible if we define the error type here in this crate.
#[macro_export]
macro_rules! define_error {
    ($name:ident) => {
        /// A struct that wraps an error together with a backtrace and an optional description
        pub struct $name<E> {
            err: E,
            desc: std::borrow::Cow<'static, str>,
            backtrace: std::option::Option<$crate::backtrace::Backtrace>
        }
        impl<E> $name<E> {
            /// Captures a backtrace and creates a new error
            pub fn new(err: E, desc: String) -> Self {
                let backtrace = $crate::backtrace::Backtrace::capture();
                let desc = std::borrow::Cow::Owned(desc);
                Self::with_backtrace(err, desc, backtrace)
            }
            /// Captures a backtrace and creates a new error with a static description
            pub fn new_static(err: E, desc: &'static str) -> Self {
                let backtrace = $crate::backtrace::Backtrace::capture();
                let desc = std::borrow::Cow::Borrowed(desc);
                Self::with_backtrace(err, desc, backtrace)
            }
            /// Creates a new error with the given backtrace
            pub const fn with_backtrace(err: E, desc: std::borrow::Cow<'static, str>,
                backtrace: Option<$crate::backtrace::Backtrace>) -> Self
            {
                Self { err, desc, backtrace }
            }

            /// The wrapped error
            pub const fn err(&self) -> &E {
                &self.err
            }
            /// The error description
            pub const fn desc(&self) -> &std::borrow::Cow<'static, str> {
                &self.desc
            }
            // TODO: Replace with `std::error::Error::backtrace` when `std::backtrace::Backtrace` becomes stable
            /// The underlying backtrace
            pub fn backtrace(&self) -> Option<&$crate::backtrace::Backtrace> {
                self.backtrace.as_ref()
            }
        }
        impl<E> std::ops::Deref for $name<E> {
            type Target = E;
            fn deref(&self) -> &Self::Target {
                &self.err
            }
        }
        impl<E> std::convert::From<E> for $name<E> where E: std::fmt::Display {
            fn from(error: E) -> Self {
                let desc = error.to_string();
                Self::new(error, desc)
            }
        }
        // Error
        impl<E> std::error::Error for $name<E> where E: std::error::Error {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.err.source()
            }
            // TODO: Reimplement when `std::backtrace::Backtrace` becomes stable
            /*
            fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
                Some(&self.backtrace)
            }
            */
        }
        // Debug
        impl<E> std::fmt::Debug for $name<E> where E: std::fmt::Debug {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(std::any::type_name::<Self>())
                    .field("err", &self.err)
                    .field("desc", &self.desc)
                    .field("backtrace", &self.backtrace)
                    .finish()
            }
        }
        // Display
        impl<E> std::fmt::Display for $name<E> where E: std::fmt::Display {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Write the error and description
                write!(f, "{}", &self.err)?;
                if !self.desc.is_empty() {
                    write!(f, " ({})", &self.desc)?;
                }

                // Print the backtrace if we have any
                if let Some(backtrace) = self.backtrace.as_ref() {
                    writeln!(f)?;
                    writeln!(f)?;
                    writeln!(f, "Backtrace:")?;
                    write!(f, "{}", backtrace)?;
                }
                Ok(())
            }
        }
        // Default
        impl<E> std::default::Default for $name<E> where E: std::default::Default + std::fmt::Display {
            fn default() -> Self {
                Self::from(E::default())
            }
        }
        // Clone
        impl<E> std::clone::Clone for $name<E> where E: std::clone::Clone {
            fn clone(&self) -> Self {
                Self {
                    err: self.err.clone(),
                    desc: self.desc.clone(),
                    backtrace: self.backtrace.clone()
                }
            }
        }
    };
}
