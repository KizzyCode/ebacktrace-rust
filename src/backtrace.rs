//! Implements a backtrace

use std::{ 
    cell::RefCell,
    fmt::{ self, Display, Formatter }
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
#[derive(Debug)]
pub struct Backtrace {
    /// The wrapped backtrace; will use `std::backtrace` once it is stable
    inner: Option<RefCell<InnerMut>>
}
impl Backtrace {
    /// Captures a new backtrace
    #[inline]
    pub fn capture() -> Self {
        // Determine whether we should capture a backtrace or not
        #[cfg(not(feature = "force_backtrace"))]
        let capture_backtrace = {
            // HACK: Use full path to avoid "unused_imports"-errors when using "force_backtrace"
            let rust_backtrace = std::env::var("RUST_BACKTRACE").unwrap_or_default();
            matches!(rust_backtrace.as_str(), "1" | "true" | "full")
        };
        #[cfg(feature = "force_backtrace")]
        let capture_backtrace = true;

        // Capture the backtrace if appropriate
        let mut inner = None;
        if capture_backtrace {
            let inner_mut = InnerMut { backtrace: backtrace::Backtrace::new_unresolved(), string: None };
            inner = Some(RefCell::new(inner_mut));
        }

        // Create `self`
        Self { inner }
    }
}
impl Display for Backtrace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Unwrap the backtrace if any
        let inner = match self.inner.as_ref() {
            None => {
                // Write indent and info message
                write!(f, "   ")?;
                write!(f, "Backtraces are hidden; use \"RUST_BACKTRACE=1\" to display backtraces")?;
                return Ok(())
            },
            Some(inner) => inner
        };

        // Check whether we have a formatted backtrace already or not
        let has_backtrace = {
            // Get the cached backtrace
            let backtrace_ref = inner.borrow();
            backtrace_ref.string.is_some()
        };
        
        // Resolve the backtrace if necessary
        if !has_backtrace {
            let mut inner_mut = inner.borrow_mut();
            inner_mut.backtrace.resolve();
            inner_mut.string = Some(format!("{:?}", inner_mut.backtrace));
        }

        // Write the backtrace
        let backtrace_ref = inner.borrow();
        let string = backtrace_ref.string.as_ref().expect("Failed to get cached backtrace?!");
        write!(f, "{}", string)
    }
}