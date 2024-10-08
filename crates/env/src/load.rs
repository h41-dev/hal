use alloc::string::ToString;
use core::fmt::{Display, Formatter};

use hal_core::module::ModuleId;
use hal_wasm::WasmParser;
use hal_wat::WatParser;

use crate::{Environment, LoadError, wasm_source, wat_source};

pub trait LoadWasm<SOURCE> {
    fn load(&mut self, source: SOURCE) -> Result<ModuleId, LoadError>;
}

pub trait LoadWat<SOURCE> {
    fn load(&mut self, source: SOURCE) -> Result<ModuleId, LoadError>;
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl<T: AsRef<[u8]>> LoadWasm<wasm_source::Bytes<T>> for Environment {
    fn load(&mut self, source: wasm_source::Bytes<T>) -> Result<ModuleId, LoadError> {
        let wasm = WasmParser::parse(source.as_ref())?;
        let module_id = self.modules.len() as ModuleId;
        let module = self.compiler.compile(module_id, wasm)?;


        self.modules.push(module);


        return Ok(module_id);
    }
}

impl<T: AsRef<str>> LoadWasm<wat_source::String<T>> for Environment {
    fn load(&mut self, source: wat_source::String<T>) -> Result<ModuleId, LoadError> {
        let bytes = WatParser::parse_str(source.as_ref())
            .map(|data| wasm_source::bytes(data))?;
        self.load(bytes)
    }
}


#[cfg(test)]
mod tests {
    mod wat {
        mod string {
            use crate::{Environment, LoadError, LoadWasm, wat_source};

            #[test]
            fn ok() {
                let mut ti = Environment::default();
                let result = ti.load(wat_source::string("(module)"));
                assert!(result.is_ok(), "Loading module via string failed");
                assert_eq!(result.unwrap(), 0);
            }

            #[test]
            fn parsing_fails() {
                let mut ti = Environment::default();
                let result = ti.load(wat_source::string("(module"));
                assert_eq!(result.err(), Some(LoadError::wat_parsing_failed("expected `)`\n     --> <anon>:1:8\n      |\n    1 | (module\n      |        ^")));
            }
        }
    }

    mod wasm {}
}