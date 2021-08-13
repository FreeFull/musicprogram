use std::fmt;

#[derive(Debug)]
pub enum Error {
    Jack(jack::Error),
}

impl From<jack::Error> for Error {
    fn from(err: jack::Error) -> Error {
        Error::Jack(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("JACK error.")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Jack(ref err) => Some(err),
        }
    }
}
