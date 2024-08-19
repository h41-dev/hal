use alloc::vec::Vec;
use core::cell::RefCell;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Memory {
    pub data: RefCell<Vec<u8>>,
    pub max: Option<u32>,
}
