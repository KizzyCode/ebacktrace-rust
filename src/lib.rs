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
//! 
//! /// The error kind
//! #[derive(Debug, Copy, Clone)]
//! enum ErrorKind {
//!     MyErrorA,
//!     Testolope
//! }
//! 
//! // Define our custom error type
//! define_error!(Error);
//! 
//! /// A function that will always fail
//! fn will_fail() -> Result<(), Error<ErrorKind>> {
//!     Err(ErrorKind::Testolope)?
//! }
//! 
//! // Will panic with a nice fully-backtraced error
//! will_fail().unwrap();
//! ```
//! 
//! ## Features
//! This crate currently has one feature gate:
//!   - `derive_display` (enabled by default): Implements the `Display`-trait for `Etrace<MyType>` using the `Debug`
//!     representation of `MyType` (instead of the `Display` representation). This way you can pretty-print the underlying
//!     error types without the necessity to manually implement the `Display`-trait for them.

/// Implements a backtrace
#[doc(hidden)]
pub mod backtrace;


/// Defines a custom error with descripion
#[macro_export]
macro_rules! define_error {
    ($name:ident) => {
        /// A struct that wraps an error together with a backtrace and an optional description
        pub struct $name<E> {
            err: E,
            desc: std::borrow::Cow<'static, str>,
            backtrace: std::sync::Arc<$crate::backtrace::Backtrace>
        }
        impl<E> $name<E> {
            /// Wraps an error `err`
            pub fn new(err: E) -> Self {
                let backtrace = $crate::backtrace::Backtrace::capture();
                Self {
                    err,
                    desc: std::borrow::Cow::Borrowed(""),
                    backtrace: std::sync::Arc::new(backtrace)
                }
            }
            /// Wraps an error `err` together with a description `desc`
            pub fn with_str(err: E, desc: &'static str) -> Self {
                let backtrace = $crate::backtrace::Backtrace::capture();
                Self {
                    err,
                    desc: std::borrow::Cow::Borrowed(desc),
                    backtrace: std::sync::Arc::new(backtrace)
                }
            }
            /// Wraps an error `err` together with a description `desc`
            pub fn with_string<S>(err: E, desc: S) -> Self where S: std::string::ToString {
                let backtrace = $crate::backtrace::Backtrace::capture();
                Self {
                    err,
                    desc: std::borrow::Cow::Owned(desc.to_string()),
                    backtrace: std::sync::Arc::new(backtrace)
                }
            }

            /// The wrapped error
            pub const fn err(&self) -> &E {
                &self.err
            }
            /// The error description
            pub const fn desc(&self) -> &std::borrow::Cow<'static, str> {
                &self.desc
            }
        }
        impl<E> std::ops::Deref for $name<E> {
            type Target = E;
            fn deref(&self) -> &Self::Target {
                &self.err
            }
        }
        impl<E> std::convert::From<E> for $name<E> {
            fn from(error: E) -> Self {
                Self::new(error)
            }
        }
        // Error
        impl<E> std::error::Error for $name<E> where E: std::error::Error {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.err.source()
            }
            // TODO: Reimplement when `std::backtrace::Backtrace` becomes stable
            /*
            fn backtrace(&self) -> Option<&$crate::backtrace::Backtrace> {
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
        #[cfg(not(feature = "derive_display"))]
        impl<E> std::fmt::Display for $name<E> where E: std::fmt::Display {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
        impl<E> std::fmt::Display for $name<E> where E: std::fmt::Debug {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
        impl<E> std::default::Default for $name<E> where E: std::default::Default {
            fn default() -> Self {
                Self::new(E::default())
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
