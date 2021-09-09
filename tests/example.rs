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
    Err(ErrorKind::Testolope)?
}


#[test]
#[should_panic]
fn test() {
    // Will panic with a nice error
    will_fail().map_err(|e| e.to_string()).unwrap();
}
