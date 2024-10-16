use crate::types::Limit;

const PAGE_SIZE: usize = 65536;

pub struct Memory {
    data: Vec<u8>,
    limits: Limit,
}
impl Memory {
    pub fn new(limit: Limit) -> Memory {
        Memory {
            data: vec![0; limit.min as usize * PAGE_SIZE],
            limits: limit,
        }
    }
}
