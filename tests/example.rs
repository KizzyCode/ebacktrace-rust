use ebacktrace::define_error;
use std::fmt::{ self, Display, Formatter };


/// The error kind
#[derive(Debug, Copy, Clone)]
enum ErrorKind {
    #[allow(unused)]
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
    Err(ErrorKind::Testolope.into())
}


#[test]
#[should_panic]
fn test() {
    // Will panic with a nice error
    if let Err(e) = will_fail() {
        // Print the debugging representation
        eprintln!("Error: {:?}", e);
        
        // Print the display representation
        eprintln!("Error: {}", e);
        
        // PAAAANIIIIC
        panic!("Fatal error: {}", e);
    }
}
