use alloc::string::String;

use crate::module::function::FunctionAddress;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Export {
    name: String,
    data: ExportData,
}

impl Export {
    pub fn function(name: String, addr: FunctionAddress) -> Self {
        Self {
            name,
            data: ExportData::Function(addr),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn data(&self) -> &ExportData {
        &self.data
    }
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum ExportData {
    Function(FunctionAddress),
}
