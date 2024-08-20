use alloc::string::ToString;
use alloc::vec;
use core::fmt::{Display, Formatter};

use hal_process::{Process, ProcessState};
use hal_wasm::WasmParser;
use hal_wat::WatParser;

use crate::{State, SingleThreadedEnvironment};
use crate::env::error::LoadError;
use crate::env::source::{wasm_source, wat_source};

pub trait LoadWasm<SOURCE> {
    fn load(&mut self, source: SOURCE) -> Result<State, LoadError>;
}

pub trait LoadWat<SOURCE> {
    fn load(&mut self, source: SOURCE) -> Result<State, LoadError>;
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}


impl<T: AsRef<[u8]>> LoadWasm<wasm_source::Bytes<T>> for SingleThreadedEnvironment {
    fn load(&mut self, source: wasm_source::Bytes<T>) -> Result<State, LoadError> {
        let wasm = WasmParser::parse(source.as_ref())?;
        let module = self.compiler.compile(wasm)?;

        Ok(State {
            processor: &self.processor,
            process: Process {
                state: ProcessState::new(module).unwrap(),
                stack: vec![],
                call_stack: vec![],
            },
        })
    }
}

impl<T: AsRef<str>> LoadWasm<wat_source::String<T>> for SingleThreadedEnvironment {
    fn load(&mut self, source: wat_source::String<T>) -> Result<State, LoadError> {
        let bytes = WatParser::parse_str(source.as_ref())
            .map(|data| wasm_source::bytes(data))?;

        self.load(bytes)
    }
}


#[cfg(test)]
mod tests {
    mod wat {
        mod string {
            use crate::{SingleThreadedEnvironment, wat_source};
            use crate::env::single_threaded::load::{LoadError, LoadWasm};

            #[test]
            fn ok() {
                let mut ti = SingleThreadedEnvironment::default();
                let result = ti.load(wat_source::string("(module)"));
                assert!(result.is_ok(), "Loading module via string failed");
                todo!()
            }

            #[test]
            fn parsing_fails() {
                let mut ti = SingleThreadedEnvironment::default();
                let result = ti.load(wat_source::string("(module"));
                assert_eq!(result.err(), Some(LoadError::wasm_parsing_failed("expected `)`\n     --> <anon>:1:8\n      |\n    1 | (module\n      |        ^")));
                todo!()
            }
        }
    }

    mod wasm {}
}