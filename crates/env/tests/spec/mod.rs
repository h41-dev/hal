extern crate std;

use std::fs;
use std::path::{Path, PathBuf};

use wast::{QuoteWat, Wast, WastExecute};
use wast::core::{WastArgCore, WastRetCore};
use wast::lexer::Lexer;
use wast::parser::ParseBuffer;

use hal_core::module::Value;
use hal_env::{Environment, LoadWasm, wasm_source};

mod core;
mod incubator;

fn run_test(category: &str, file: &str) {
    let mut env = Environment::default();

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(Path::new(format!("tests/spec/{}/{}.wast", category, file).as_str()));
    let test_file = fs::read(file_path).expect(format!("Unable to read file {}", file).as_str());
    let wast = std::str::from_utf8(test_file.as_ref()).expect("failed to convert wast to utf8");
    let mut lexer = Lexer::new(wast.clone());
    lexer.allow_confusing_unicode(true);
    let buf = ParseBuffer::new_with_lexer(lexer).expect("failed to create parse buffer");
    let wast_data = wast::parser::parse::<Wast>(&buf).expect("failed to parse wat");


    for (i, directive) in wast_data.directives.into_iter().enumerate() {
        let span = directive.span();
        use wast::WastDirective::*;

        match directive {
            Wat(module) => {
                let (name, bytes) = read_quote_wat(module);
                env.load(wasm_source::bytes(bytes));

                println!("module")
            }

            AssertReturn {
                span, exec, results
            } => {
                let expected = map_wast_return_value(results.into_iter());

                match exec {
                    WastExecute::Invoke(invoke) => {
                        let args = map_wast_args(invoke.args.into_iter());
                        match env.invoke(invoke.name, args) {
                            Ok(results) => {
                                assert_eq!(expected, results, "expected {:?}, got {:?}", expected, results)
                            }
                            Err(e) => {
                                panic!("{:?}", e)
                            }
                        };
                    }
                    WastExecute::Wat(_) => todo!(),
                    WastExecute::Get { .. } => todo!(),
                }
            }

            AssertMalformed { span, mut module, message } => {}

            AssertInvalid { span, mut module, message: _ } => {}

            AssertExhaustion { call, message, span } => {}

            AssertTrap { exec, message, span } => {}

            AssertUnlinkable { mut module, span, message } => {}

            Invoke(invoke) => {}

            Register { span, name, .. } => {}

            AssertException { .. } => {}

            Thread(_) => {}

            Wait { .. } => {}
        }
    }
}

fn read_quote_wat(module: QuoteWat) -> (Option<String>, Box<[u8]>) {
    match module {
        QuoteWat::Wat(mut wat) => {
            let wast::Wat::Module(ref module) = wat else {
                unimplemented!("Not supported");
            };
            (module.id.map(|id| id.name().to_string()), Box::from(wat.encode().expect("failed to encode module")))
        }
        _ => unimplemented!("Not supported"),
    }
}


pub fn map_wast_return_value<'a>(args: impl Iterator<Item=wast::WastRet<'a>>) -> Vec<Value> {
    args.map(|ret| {
        let wast::WastRet::Core(ret) = ret else {
            panic!("unsupported type");
        };
        match ret {
            WastRetCore::I32(v) => Value::I32(v),
            WastRetCore::I64(_) => todo!(),
            WastRetCore::F32(_) => todo!(),
            WastRetCore::F64(_) => todo!(),
            WastRetCore::V128(_) => todo!(),
            WastRetCore::RefNull(_) => todo!(),
            WastRetCore::RefExtern(_) => todo!(),
            WastRetCore::RefHost(_) => todo!(),
            WastRetCore::RefFunc(_) => todo!(),
            WastRetCore::RefAny => todo!(),
            WastRetCore::RefEq => todo!(),
            WastRetCore::RefArray => todo!(),
            WastRetCore::RefStruct => todo!(),
            WastRetCore::RefI31 => todo!(),
            WastRetCore::Either(_) => todo!()
        }
    }).collect()
}


pub fn map_wast_args<'a>(args: impl Iterator<Item=wast::WastArg<'a>>) -> Vec<Value> {
    args.map(|ret| {
        let wast::WastArg::Core(arg) = ret else {
            panic!("unsupported type");
        };
        match arg {
            WastArgCore::I32(v) => Value::I32(v),
            WastArgCore::I64(_) => todo!(),
            WastArgCore::F32(_) => todo!(),
            WastArgCore::F64(_) => todo!(),
            WastArgCore::V128(_) => todo!(),
            WastArgCore::RefNull(_) => todo!(),
            WastArgCore::RefExtern(_) => todo!(),
            WastArgCore::RefHost(_) => todo!(),
        }
    }).collect()
}