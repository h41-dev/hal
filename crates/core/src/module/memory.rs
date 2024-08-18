use alloc::vec::Vec;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Memory {
    pub data: Vec<u8>,
    pub max: Option<u32>,
}
