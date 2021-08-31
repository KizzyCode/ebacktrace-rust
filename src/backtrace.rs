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
    inner: RefCell<InnerMut>
}
impl Backtrace {
    /// Captures a new backtrace
    #[inline]
    pub fn capture() -> Self {
        let inner_mut = InnerMut { backtrace: backtrace::Backtrace::new_unresolved(), string: None };
        Self { inner: RefCell::new(inner_mut) }
    }
}
impl Display for Backtrace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Check whether we have a backtrace or not
        let has_backtrace = {
            // Get the cached backtrace
            let backtrace_ref = self.inner.borrow();
            backtrace_ref.string.is_some()
        };
        
        // Resolve the backtrace if necessary
        if !has_backtrace {
            let mut inner_mut = self.inner.borrow_mut();
            inner_mut.string = Some(format!("{:#?}", inner_mut.backtrace));
        }

        // Write the backtrace
        let backtrace_ref = self.inner.borrow();
        let string = backtrace_ref.string.as_ref().expect("Failed to get cached backtrace?!");
        write!(f, "{}", string)
    }
}