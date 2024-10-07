use crate::types::FuncIdx;

use super::local::Local;

#[derive(Debug)]
pub struct FunctionState {
    pub locals: Vec<Local>,
    pub instruction_index: usize,
    pub function_idx: FuncIdx,
}
