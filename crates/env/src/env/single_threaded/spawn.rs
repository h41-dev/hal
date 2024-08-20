use hal_wat::WatParser;

use crate::{State, LoadWasm, SingleThreadedEnvironment, wat_source};
use crate::env::error::LoadError;
use crate::env::source::wasm_source;


pub trait SpawnWasm<SOURCE> {
    fn spawn(&mut self, source: SOURCE) -> Result<State, LoadError>;
}

pub trait SpawnWat<SOURCE> {
    fn spawn(&mut self, source: SOURCE) -> Result<State, LoadError>;
}

impl<T: AsRef<[u8]>> SpawnWasm<wasm_source::Bytes<T>> for SingleThreadedEnvironment {
    fn spawn(&mut self, source: wasm_source::Bytes<T>) -> Result<State, LoadError> {
        let state = self.load(source)?;

        Ok(state)
    }
}

impl<T: AsRef<str>> SpawnWat<wat_source::String<T>> for SingleThreadedEnvironment {
    fn spawn(&mut self, source: wat_source::String<T>) -> Result<State, LoadError> {
        let bytes = WatParser::parse_str(source.as_ref())
            .map(|data| wasm_source::bytes(data))?;
        SpawnWasm::spawn(self, bytes)
    }
}
