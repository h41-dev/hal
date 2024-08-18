use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use hal_core::constant::PAGE_SIZE;
use hal_core::module::Module;
use hal_wasm::{WasmExportDescriptor, WasmImportDescriptor, WasmModule};

pub struct Compiler {}

impl Default for Compiler {
    fn default() -> Self {
        Self{}
    }
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum CompilationError {
    PlaceHolder
}

impl core::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CompilationError::PlaceHolder => todo!()
        }
    }
}


impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, wasm: hal_wasm::WasmModule) -> Result<Module, CompilationError> {
        let func_type_idxs = match wasm.functions {
            ref idxs => idxs.clone(),
            _ => Box::default()
        };

        // let mut functions = vec![];
        // let mut memories = vec![];

        // if let ref import_section = wasm.imports {
        //     for import in import_section {
        //         let module_name = import.module.clone();
        //         let field = import.name.clone();
        //         let func_type = match import.desc {
        //             WasmImportDescriptor::Func(type_idx) => {
        //                 let ref func_types = wasm.types else {
        //                     panic!("not found type_section")
        //                 };
        //
        //                 let Some(func_type) = func_types.get(type_idx as usize) else {
        //                     panic!("not found func types in type_section")
        //                 };
        //
        //                 func_type.clone()
        //             }
        //             WasmImportDescriptor::Table(_) => todo!(),
        //             WasmImportDescriptor::Memory(_) => todo!(),
        //         };
        //
        //         let func = hal_core::module::Function::External(hal_core::module::ExternalFunction {
        //             module: std::str::from_utf8(module_name).unwrap().to_string(),
        //             func: std::str::from_utf8(field).unwrap().to_string(),
        //             func_type: hal_core::module::FuncType {
        //                 params: func_type.params.to_vec(),
        //                 results: func_type.returns.to_vec(),
        //             },
        //         });
        //         functions.push(func);
        //     }
        // }

        // if let ref code_section = wasm.codes {
        //     for (func_body, type_idx) in code_section.iter().zip(func_type_idxs.into_iter()) {
        //         let ref func_types = wasm.types else {
        //             panic!("not found type_section")
        //         };
        //
        //         let Some(func_type) = func_types.get(*type_idx as usize) else {
        //             panic!("not found func types in type_section")
        //         };
        //
        //         let mut locals = Vec::with_capacity(func_body.locals.len());
        //         for local in func_body.locals.iter() {
        //             for _ in 0..local.0 {
        //                 locals.push(local.1.clone());
        //             }
        //         }
        //
        //         let func = hal_core::module::Function::Internal(hal_core::module::InternalFunction {
        //             // func_type: func_type.clone(),
        //             func_type: hal_core::module::FuncType {
        //                 params: func_type.params.to_vec(),
        //                 results: func_type.returns.to_vec(),
        //             },
        //             code: hal_core::module::Func {
        //                 locals,
        //                 body: func_body.code.iter()
        //                     .map(|i|{
        //                         match i {
        //                             WasmInstruction::LocalGet(addr) => Instruction::LocalGet(addr.clone()),
        //                             WasmInstruction::LocalSet(addr) => Instruction::LocalSet(addr.clone()),
        //                             WasmInstruction::I32Store { offset, addr } => Instruction::I32Store { offset: offset.clone(), idx: addr.clone()},
        //                             WasmInstruction::I32Const(addr) => Instruction::I32Const(addr.clone()),
        //                             WasmInstruction::End => Instruction::End,
        //                             WasmInstruction::I32Add => Instruction::I32Add,
        //                             WasmInstruction::Call(addr) => Instruction::Call(addr.clone())
        //                         }
        //                     }).collect(),
        //             },
        //         });
        //         functions.push(func);
        //     }
        // }
        // let mut exports = HashMap::default();
        // if let ref sections = wasm.exports {
        //     for export in sections {
        //         let name = core::str::from_utf8(export.name.clone()).unwrap().to_string();
        //         let export_inst = hal_core::module::Export {
        //             // name: name.clone(),
        //             // desc: match export.desc {
        //             //     WasmExportDescriptor::Func(v) => hal_core::module::ExportDesc::Func(v),
        //             //     WasmExportDescriptor::Table(_) => todo!(),
        //             //     WasmExportDescriptor::Memory(_) => todo!(),
        //             //     WasmExportDescriptor::Global(_) => todo!()
        //             // },
        //         };
        //         exports.insert(name, export_inst);
        //     }
        // };
        // // let module_inst = ModuleInst { exports };
        // if let ref sections = wasm.memories {
        //     for memory in sections {
        //         let min = memory.limits.min * PAGE_SIZE;
        //         // let memory = hal_core::module::Memory {
        //         //     data: vec![0; min as usize],
        //         //     max: memory.limits.max,
        //         // };
        //         // memories.push(memory);
        //     }
        // }
        //
        // if let ref sections = wasm.data {
        //     for data in sections {
        //         let memory = memories
        //             .get_mut(data.memory_index as usize)
        //             .unwrap(); // FIXME
        //         // .ok_or(Error::Placeholder(data.memory_index))?;
        //
        //         let offset = data.offset as usize;
        //         let init = &data.data;
        //         if offset + init.len() > memory.data.len() {
        //             panic!("data is too large to fit in memory");
        //         }
        //
        //         memory.data[offset..offset + init.len()].copy_from_slice(init);
        //     }
        // }


        Ok(
            Module {
                // exports,
                // functions,
                // memories,
            }
        )
    }
}