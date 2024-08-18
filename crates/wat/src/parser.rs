use alloc::boxed::Box;
use alloc::string::{String, ToString};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct WatParseError(String);

impl core::fmt::Display for WatParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct WatParser {}

impl WatParser {
    pub fn parse_str<T: AsRef<str>>(wat: T) -> Result<Box<[u8]>> {
        Ok(wat_delegate::parse_str(wat.as_ref()).map_err(|e| WatParseError(e.to_string()))?.into())
    }

    pub fn parse_file<T: AsRef<str>>(path: T) -> Result<Box<[u8]>> {
        Ok(wat_delegate::parse_file(path.as_ref()).map_err(|e| WatParseError(e.to_string()))?.into())
    }
}

pub(crate) type Result<T, E = WatParseError> = core::result::Result<T, E>;
