//! Implements a backtrace

use std::{ 
    cell::RefCell,
    fmt::{ self, Debug, Display, Formatter }
};


/// The real backtrace implementation
#[derive(Debug)]
struct InnerMut {
    /// The wrapped backtrace; will use `std::backtrace` once it is stable
    pub backtrace: backtrace::Backtrace,
    /// The backtrace as string
    pub string: Option<String>
}


/// The backtrace implementation
pub struct Backtrace {
    /// The wrapped backtrace; will use `std::backtrace` once it is stable
    inner: RefCell<InnerMut>
}
impl Backtrace {
    /// Captures a new backtrace if `RUST_BACKTRACE` is set
    #[inline]
    #[cfg(not(feature = "force_backtrace"))]
    pub fn capture() -> Option<Self> {
        // Determine whether we should capture a backtrace or not
        let capture_backtrace = {
            // HACK: Use full path to avoid "unused_imports"-errors when using "force_backtrace"
            let rust_backtrace = std::env::var("RUST_BACKTRACE").unwrap_or_default();
            matches!(rust_backtrace.as_str(), "1" | "true" | "full")
        };

        // Capture the backtrace if appropriate
        if capture_backtrace {
            let inner_mut = InnerMut { backtrace: backtrace::Backtrace::new_unresolved(), string: None };
            Some(Self { inner: RefCell::new(inner_mut) })
        } else {
            None
        }
    }

    /// Always captures a new backtrace
    #[inline]
    #[cfg(feature = "force_backtrace")]
    pub fn capture() -> Option<Self> {
        let inner_mut = InnerMut { backtrace: backtrace::Backtrace::new_unresolved(), string: None };
        Some(Self { inner: RefCell::new(inner_mut) })
    }
}
impl Debug for Backtrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Check whether we have a resolved backtrace already or not
        let has_backtrace = {
            // Get the cached backtrace
            let backtrace_ref = self.inner.borrow();
            backtrace_ref.string.is_some()
        };

        // Resolve the backtrace if necessary
        if !has_backtrace {
            let mut inner_mut = self.inner.borrow_mut();
            inner_mut.backtrace.resolve();
            inner_mut.string = Some(format!("{:?}", inner_mut.backtrace));
        }

        // Write the struct
        f.debug_struct("Backtrace")
            .field("inner", &self.inner)
            .finish()
    }
}
impl Display for Backtrace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Check whether we have a resolved backtrace already or not
        let has_backtrace = {
            // Get the cached backtrace
            let backtrace_ref = self.inner.borrow();
            backtrace_ref.string.is_some()
        };

        // Resolve the backtrace if necessary
        if !has_backtrace {
            let mut inner_mut = self.inner.borrow_mut();
            inner_mut.backtrace.resolve();
            inner_mut.string = Some(format!("{:?}", inner_mut.backtrace));
        }

        // Write the backtrace
        let backtrace_ref = self.inner.borrow();
        let bactrace_string = backtrace_ref.string.as_ref().expect("Failed to access captured backtrace?!");
        write!(f, "{}", bactrace_string)
    }
}