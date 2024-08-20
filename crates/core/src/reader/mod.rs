pub use byte::ByteReader;

use crate::leb128::Leb128Error;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum Error {
    OutOfBounds,
    UnexpectedEndOfFile,
    InvalidLEB128Encoding,
}

impl From<Leb128Error> for Error {
    fn from(value: Leb128Error) -> Self {
        match value {
            Leb128Error::InvalidEncoding => Error::InvalidLEB128Encoding,
            Leb128Error::IncompleteEncoding => Error::UnexpectedEndOfFile
        }
    }
}

mod byte;