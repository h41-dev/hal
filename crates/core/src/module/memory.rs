use alloc::vec::Vec;
use core::cell::RefCell;


pub type MemoryOffset = u32;
pub type MemoryFlags = u32;
pub type MemoryAddress = u32;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Memory {
    pub data: RefCell<Vec<u8>>,
    pub max: Option<u32>,
}


impl Memory {

}

