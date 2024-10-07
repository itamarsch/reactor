mod expr;
mod function;
mod instruction;
mod local;
mod memory_argument;

pub use expr::Expr;
pub use function::FunctionCode;
pub use instruction::Instruction;
pub use local::LocalIdx;
pub use local::LocalTypes;
pub use memory_argument::MemoryArgument;
