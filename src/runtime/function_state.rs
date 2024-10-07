use super::local::Local;

struct FunctionState {
    locals: Vec<Local>,
    instruction_index: usize,
}
