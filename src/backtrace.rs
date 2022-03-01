//! Implements a backtrace

use std::{ 
    fmt::{ self, Debug, Display, Formatter },
    sync::{ Arc, Mutex }
};


/// The "raw" underlying backtrace
#[derive(Debug)]
struct BacktraceRaw {
    /// The wrapped backtrace; will use `std::backtrace` once it is stable
    backtrace: backtrace::Backtrace,
    /// The backtrace as human readable string
    readable: String
}
impl BacktraceRaw {
    /// Creates a new unresolved (=thin) backtrace
    pub fn new_thin() -> Self {
        Self { backtrace: backtrace::Backtrace::new_unresolved(), readable: String::new() }
    }

    /// Ensures that the backtrace has been resolved
    pub fn ensure_resolved(&mut self) {
        // Resolve the backtrace
        if self.readable.is_empty() {
            self.backtrace.resolve();
            self.readable = format!("{:?}", self.backtrace);
        }
    }
}


/// The backtrace implementation
#[derive(Clone)]
pub struct Backtrace {
    /// The wrapped backtrace; will use `std::backtrace` once it is stable
    inner: Arc<Mutex<BacktraceRaw>>
}
impl Backtrace {
    /// Captures a new backtrace if `RUST_BACKTRACE` is set
    #[inline]
    #[cfg(not(feature = "force_backtrace"))]
    pub fn capture() -> Option<Self> {
        // NOTE: Use full path to avoid "unused_imports"-errors when using "force_backtrace"
        let rust_backtrace = std::env::var("RUST_BACKTRACE").unwrap_or_default();
        if !matches!(rust_backtrace.as_str(), "1" | "true" | "full") {
            return None
        }

        // Capture the backtrace
        let backtrace = BacktraceRaw::new_thin();
        let this = Self { inner: Arc::new(Mutex::new(backtrace)) };
        Some(this)
    }

    /// Always captures a new backtrace
    #[inline]
    #[cfg(feature = "force_backtrace")]
    pub fn capture() -> Option<Self> {
        let backtrace = BacktraceRaw::new_thin();
        let this = Self { inner: Arc::new(Mutex::new(backtrace)) };
        Some(this)
    }
}
impl Debug for Backtrace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Get exclusive access to the underlying backtrace
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(inner) => inner.into_inner()
        };

        // Resolve backtrace if necessary and write the struct
        inner.ensure_resolved();
        f.debug_struct("Backtrace")
            .field("inner", &inner)
            .finish()
    }
}
impl Display for Backtrace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Get exclusive access to the underlying backtrace
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(inner) => inner.into_inner()
        };

        // Resolve backtrace if necessary and write the backtrace
        inner.ensure_resolved();
        write!(f, "{}", &inner.readable)
    }
}
