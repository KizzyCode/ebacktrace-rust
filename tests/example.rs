use ebacktrace::define_error;

/// The error kind
#[derive(Debug, Copy, Clone)]
enum ErrorKind {
    #[allow(unused)]
    MyErrorA,
    Testolope
}

// Define our custom error type
define_error!(Error);

/// A function that will always fail
fn will_fail() -> Result<(), Error<ErrorKind>> {
    Err(ErrorKind::Testolope.into())
}


#[test]
#[should_panic]
fn test() {
    // Will panic with a nice error
    if let Err(e) = will_fail() {
        // Print the debugging representation
        #[cfg(not(feature = "derive_display"))]
        eprintln!("Error: {:?}", e);
        
        // Print the display representation
        #[cfg(feature = "derive_display")]
        eprintln!("Error: {}", e);
        
        panic!("Fatal error")
    }
}
