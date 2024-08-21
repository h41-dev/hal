use hal_core::module::ModuleId;
use hal_wat::WatParser;

use crate::{Environment, Instance, wat_source};
use crate::env::error::EnvironmentError;
use crate::env::load::LoadWasm;
use crate::env::source::wasm_source;

pub trait Spawn {
    fn spawn(&mut self, id: ModuleId) -> Result<&Instance, EnvironmentError>;
}

pub trait SpawnWasm<SOURCE> {
    fn spawn(&mut self, source: SOURCE) -> Result<&mut Instance, EnvironmentError>;
}

pub trait SpawnWat<SOURCE> {
    fn spawn(&mut self, source: SOURCE) -> Result<&mut Instance, EnvironmentError>;
}

impl<T: AsRef<[u8]>> SpawnWasm<wasm_source::Bytes<T>> for Environment {
    fn spawn(&mut self, source: wasm_source::Bytes<T>) -> Result<&mut Instance, EnvironmentError> {
        let module_id = self.load(source)?;
        return self.instantiate(module_id);
    }
}

impl<T: AsRef<str>> SpawnWat<wat_source::String<T>> for Environment {
    fn spawn(&mut self, source: wat_source::String<T>) -> Result<&mut Instance, EnvironmentError> {
        let bytes = WatParser::parse_str(source.as_ref())
            .map(|data| wasm_source::bytes(data))?;
        SpawnWasm::spawn(self, bytes)
    }
}
