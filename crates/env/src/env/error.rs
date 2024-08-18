use alloc::string::{String, ToString};

use hal_compile::CompilationError;
use hal_wasm::WasmParseError;
use hal_wat::WatParseError;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum LoadError {
    CompilationFailed(String),
    NotFound(String),
    WasmParsingFailed(String),
    WatParsingFailed(String),
}

impl From<WatParseError> for LoadError {
    fn from(value: WatParseError) -> Self {
        LoadError::WatParsingFailed(value.to_string())
    }
}

impl From<CompilationError> for LoadError {
    fn from(value: CompilationError) -> Self {
        LoadError::CompilationFailed(value.to_string())
    }
}

impl From<WasmParseError> for LoadError {
    fn from(value: WasmParseError) -> Self {
        LoadError::WasmParsingFailed(value.to_string())
    }
}

impl LoadError {
    pub fn not_found(reason: impl Into<String>) -> Self {
        Self::NotFound(reason.into())
    }

    pub fn compilation_failed(reason: impl Into<String>) -> Self {
        Self::CompilationFailed(reason.into())
    }

    pub fn wasm_parsing_failed(reason: impl Into<String>) -> Self {
        Self::WasmParsingFailed(reason.into())
    }

    pub fn wat_parsing_failed(reason: impl Into<String>) -> Self {
        Self::WatParsingFailed(reason.into())
    }
}

