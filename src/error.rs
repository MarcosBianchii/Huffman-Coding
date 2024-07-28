use std::{
    error::Error,
    fmt::{self, Display},
    io,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum HuffErr {
    SpecifiedProtocolIsInvalid,
    NoProtocolWasSpecified,
    InputMustBeUTF8,
    InputIsEmpty,
    InvalidBytes,
    IoError,
}

impl Error for HuffErr {}

impl Display for HuffErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::SpecifiedProtocolIsInvalid => "The specified protocol is invalid",
            Self::NoProtocolWasSpecified => "No protocol was specified",
            Self::InputMustBeUTF8 => "The input must be UTF-8",
            Self::InputIsEmpty => "The input is empty",
            Self::InvalidBytes => "The given bytes are not encoded data",
            Self::IoError => "There was an IO error",
        };

        write!(f, "{msg}")
    }
}

impl From<io::Error> for HuffErr {
    fn from(_: io::Error) -> Self {
        Self::IoError
    }
}

impl From<FromUtf8Error> for HuffErr {
    fn from(_: FromUtf8Error) -> Self {
        Self::InputMustBeUTF8
    }
}
