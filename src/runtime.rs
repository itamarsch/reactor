use std::{
    cell::{Cell, RefCell},
    ops::{Deref, DerefMut},
    process::exit,
};

use crate::{
    module::{
        functions::{Function, LocalFunction},
        Module,
    },
    types::{BlockIdx, ElementMode, FuncIdx, Instruction, ValueType},
    wasi::Wasi,
};

use self::{
    function_state::FunctionState,
    globals::{Global, Globals},
    locals::Locals,
    memory::Memory,
    stack::Stack,
    table::{TableElementIdx, Tables},
    value::Value,
};
use paste::paste;

pub mod function_state;
mod globals;
mod locals;
pub mod memory;
pub mod stack;
mod table;
mod value;
mod variable;

#[cfg(test)]
mod test;

pub struct Runtime<'a> {
    stack: RefCell<Stack>,
    module: RefCell<Module<'a>>,
    current_function_state: RefCell<FunctionState>,
    function_depth: Cell<usize>,
    memory: RefCell<Memory>,
    globals: RefCell<Globals>,
    wasi: RefCell<Wasi>,
    tables: RefCell<Tables>,
}

macro_rules! op {
    (
        $self:expr,
        { $( $ident:ident : $type:ident ),* $(,)? },
        $result_type:ident => $expr:expr
    ) => {
        {
            paste! {
                let mut stack_borrow = $self.stack.borrow_mut();
                $(
                    let $ident = stack_borrow.[<pop_ $type>]();
                )*
                stack_borrow.[<push_ $result_type>]($expr);
            }
        }
    };
}

macro_rules! memory_load {
    ($self:expr, $ty:ident, $mem_func:ident, $memarg:expr) => {
        paste! {
            {
                let address = $self.stack.borrow_mut().pop_u32();
                let value = $self.memory.borrow_mut().$mem_func(address, *$memarg);
                $self.stack.borrow_mut().[<push_ $ty>](value);
            }
        }
    };
}

macro_rules! memory_store {
    ($self:expr,$type:ident, $mem_func:ident, $memarg:expr) => {
        paste! {
            {
                let value = $self.stack.borrow_mut().[<pop_ $type>]();
                let address = $self.stack.borrow_mut().pop_u32();
                $self.memory.borrow_mut().$mem_func(value, address, *$memarg);
            }
        }
    };
}

macro_rules! block_type_to_slice {
    ($block_type:expr) => {
        match $block_type.0 {
            Some(t) => &[t][..],
            None => &[][..],
        }
    };
}

impl<'a> Runtime<'a> {
    pub fn new(module: Module<'a>) -> Self {
        let (start_idx, Function::Local(starting_function)) = module.get_starting_function() else {
            panic!("Cannot start from imported function")
        };

        assert!(
            starting_function.signature.params.is_empty(),
            "_start function cannot take arguments"
        );

        let stack = Stack::new();

        let initial_function_state = FunctionState::new_function(
            Locals::new_no_function_parameters(&starting_function.code.locals),
            start_idx,
        );

        let tables = Tables::new(module.tables());

        let runtime = Runtime {
            memory: RefCell::new(Memory::new(module.memory_limit())),
            stack: RefCell::new(stack),
            globals: RefCell::new(Globals::new()),
            tables: RefCell::new(tables),
            module: RefCell::new(module),
            current_function_state: RefCell::new(initial_function_state),
            wasi: RefCell::new(Wasi::new()),
            function_depth: Cell::new(0),
        };

        runtime.initilize_elements();
        runtime.initialize_globals();
        runtime.run_datas();

        runtime
    }

    fn wasi_function(&self, name: &str) {
        self.wasi.borrow_mut().run_function(
            name,
            self.stack.borrow_mut(),
            self.memory.borrow_mut(),
        );
    }

    pub fn run_expr<T>(&self, expr: FuncIdx, mut get_result_after_expr: impl FnMut() -> T) -> T {
        // Swaped in next line
        let mut function_state_before_expr = FunctionState::new_function(Locals::empty(), expr);
        std::mem::swap(
            &mut function_state_before_expr,
            self.current_function_state.borrow_mut().deref_mut(),
        );
        self.execute();

        let result = get_result_after_expr();

        assert!(self.stack.borrow().is_empty(), "Stack is empty");
        std::mem::swap(
            &mut function_state_before_expr,
            self.current_function_state.borrow_mut().deref_mut(),
        );

        result
    }

    fn initilize_elements(&self) {
        let module_borrow = self.module.borrow();

        for element in module_borrow.elements() {
            match element.mode {
                ElementMode::Declarative => {}
                ElementMode::Passive => {}
                ElementMode::Active {
                    table,
                    offset_in_table,
                } => {
                    let offset = self.run_expr(offset_in_table, || {
                        TableElementIdx(self.stack.borrow_mut().pop_u32() as usize)
                    });
                    let refs = element
                        .init
                        .iter()
                        .map(|init| self.run_expr(*init, || self.stack.borrow_mut().pop_ref()))
                        .collect::<Vec<_>>();

                    self.tables.borrow_mut().table(table).fill(offset, &refs);
                }
            }
        }
    }

    fn initialize_globals(&self) {
        let borrow = self.module.borrow();

        let initializers = borrow.global_initializers();

        let globals = initializers
            .iter()
            .map(|global| {
                let value = self.run_expr(global.init, || {
                    self.stack
                        .borrow_mut()
                        .pop_value_by_type(global.signature.valtype)
                });
                Global::new(value, global.signature.mutability)
            })
            .collect::<Vec<_>>();

        self.globals.borrow_mut().fill(globals);
    }

    fn run_datas(&self) {
        let module = self.module.borrow();
        for data in module.datas().iter() {
            match data.mode {
                crate::types::DataMode::Passive => continue,
                crate::types::DataMode::Active { ref offset, .. } => {
                    let offset = self.run_expr(*offset, || self.stack.borrow_mut().pop_u32());
                    self.memory.borrow_mut().fill_data(offset, &data.init);
                }
            }
        }
    }

    fn call_function(&self, func_idx: FuncIdx) {
        let module_borrow = self.module.borrow();
        let next_function = module_borrow.get_function(func_idx).unwrap();
        match next_function {
            Function::Local(function) => {
                self.function_depth.set(self.function_depth.get() + 1);
                let locals = Locals::new(
                    &function.code.locals,
                    &function.signature.params,
                    self.stack.borrow_mut().deref_mut(),
                );

                let mut new_function_state = FunctionState::new_function(locals, func_idx);

                std::mem::swap(
                    &mut new_function_state,
                    self.current_function_state.borrow_mut().deref_mut(),
                );
                self.stack
                    .borrow_mut()
                    .push_function_state(new_function_state);
            }
            Function::Imported(function) => {
                if function.mod_name == "wasi_snapshot_preview1" {
                    self.wasi_function(function.name);
                } else {
                    panic!("Unkown module import: {:?}", function.mod_name);
                }
            }
        }
    }

    fn break_from_block(&self, break_from_idx: BlockIdx, current_function: &LocalFunction) {
        let block_type = current_function
            .code
            .instructions
            .get_block_type(break_from_idx);
        let block_type_slice = block_type_to_slice!(block_type);

        self.return_from_context(block_type_slice, || {
            self.stack
                .borrow_mut()
                .push_function_state(self.current_function_state.borrow().clone());
            let mut new_function_state = self.stack.borrow_mut().break_from_block(break_from_idx);
            if current_function
                .code
                .instructions
                .is_block_loop(break_from_idx)
            {
                new_function_state.repeat_instruction();
            }
            new_function_state
        });
    }

    fn execute_block(&self, block_idx: BlockIdx) {
        let mut new_function_state = self.current_function_state.borrow().new_block(block_idx);
        std::mem::swap(
            &mut new_function_state,
            self.current_function_state.borrow_mut().deref_mut(),
        );
        self.stack
            .borrow_mut()
            .push_function_state(new_function_state);
    }

    fn pop_returns(&self, signature_returns: &[ValueType]) -> Vec<Value> {
        let amount_of_returns = signature_returns.len();
        let mut returns = Vec::with_capacity(amount_of_returns);
        for return_type in signature_returns.iter().rev() {
            let value = self.stack.borrow_mut().pop_value_by_type(*return_type);
            returns.push(value);
        }
        returns
    }

    fn reassemble_returns(&self, returns: &mut Vec<Value>) {
        for _ in 0..returns.len() {
            self.stack
                .borrow_mut()
                .push_value(returns.pop().expect("Pushed enough elements"));
        }
    }

    fn return_from_context(
        &self,
        signature_returns: &[ValueType],
        mut get_next_function_state: impl FnMut() -> FunctionState,
    ) {
        let mut returns = self.pop_returns(signature_returns);
        let function_state = get_next_function_state();
        self.reassemble_returns(&mut returns);
        *self.current_function_state.borrow_mut() = function_state;
    }

    fn return_function_end(&self, signature_returns: &[ValueType]) {
        self.return_from_context(signature_returns, || {
            self.stack.borrow_mut().pop_function_state()
        })
    }

    fn return_immediate(&self, signature_returns: &[ValueType]) {
        self.return_from_context(signature_returns, || {
            if !self.current_function_state.borrow().in_block() {
                self.stack
                    .borrow_mut()
                    .push_function_state(self.current_function_state.borrow().clone())
            }
            self.stack
                .borrow_mut()
                .pop_until_function_state(self.current_function_state.borrow().deref())
        });
        self.function_depth.set(self.function_depth.get() - 1);
    }

    pub fn execute(&self) {
        loop {
            let module_borrow = self.module.borrow();

            let Some(Function::Local(current_function)) =
                module_borrow.get_function(self.current_function_state.borrow().function_idx())
            else {
                unreachable!(
                    "Current runing function cannot be imported and its index has to exist"
                )
            };

            let current_function_state_borrow = self.current_function_state.borrow();
            if current_function
                .code
                .instructions
                .done(current_function_state_borrow.instruction_index())
            {
                if self.function_depth.get() > 0 || current_function_state_borrow.in_block() {
                    let index = current_function_state_borrow.instruction_index();
                    drop(current_function_state_borrow);

                    match index {
                        function_state::InstructionIndex::IndexInFunction(_) => {
                            self.return_function_end(&current_function.signature.returns);
                            self.function_depth.set(self.function_depth.get() - 1);
                        }
                        function_state::InstructionIndex::IndexInBlock { block_idx, .. } => {
                            let block_type =
                                current_function.code.instructions.get_block_type(block_idx);
                            let block_type_slice = block_type_to_slice!(block_type);
                            self.return_function_end(block_type_slice)
                        }
                    }
                    continue;
                } else {
                    break;
                }
            }

            let instruction = &current_function
                .code
                .instructions
                .get_instruction(current_function_state_borrow.instruction_index());
            drop(current_function_state_borrow);

            self.current_function_state.borrow_mut().next_instruction();

            self.run_instruction(instruction, current_function);
        }
    }

    fn run_instruction(&self, instruction: &Instruction, current_function: &LocalFunction) {
        match instruction {
            Instruction::Unreachable => {
                panic!("Unreachable!")
            }
            Instruction::Nop => {}
            Instruction::Block(block_idx) => self.execute_block(*block_idx),
            Instruction::Loop(block_idx) => self.execute_block(*block_idx),
            Instruction::If { if_expr, else_expr } => {
                let condition = self.stack.borrow_mut().pop_bool();
                self.execute_block(if condition { *if_expr } else { *else_expr });
            }
            Instruction::Break(break_from_idx) => {
                self.break_from_block(*break_from_idx, current_function);
            }
            Instruction::BreakIf(break_from_idx) => {
                let should_break = self.stack.borrow_mut().pop_bool();
                if should_break {
                    self.break_from_block(*break_from_idx, current_function);
                }
            }
            Instruction::BreakTable { labels, default } => {
                let index = self.stack.borrow_mut().pop_i32() as usize;
                let block_index = *labels.get(index).unwrap_or(default);
                self.break_from_block(block_index, current_function);
            }
            Instruction::Return => {
                if self.function_depth.get() == 0 {
                    exit(0);
                }
                self.return_immediate(&current_function.signature.returns[..]);
            }

            Instruction::Call(func_idx) => {
                self.call_function(*func_idx);
            }
            Instruction::CallIndirect(type_idx, table_idx) => {
                let mut tables_borrow = self.tables.borrow_mut();
                let table = tables_borrow.table(*table_idx);
                let table_element_idx = self.stack.borrow_mut().pop_table_element_idx();
                let Some(func_idx) = table.get(table_element_idx) else {
                    panic!("Issued call_indirect on a null reference.")
                };

                let module_borrow = self.module.borrow();
                let Some(func) = module_borrow.get_function(func_idx) else {
                    panic!("Function index in table isn't a valid function index");
                };
                let Some(signature) = module_borrow.function_signature(*type_idx) else {
                    panic!("call_indirect has invalid type index");
                };
                if func.signature().deref() != signature.deref() {
                    panic!("call_indirect signature doesn't fit actual function signature");
                }

                self.call_function(func_idx);
            }
            Instruction::Drop => {
                self.stack.borrow_mut().drop_value();
            }
            Instruction::Select => {
                let predicate = self.stack.borrow_mut().pop_bool();
                let false_value = self.stack.borrow_mut().pop_value();
                let true_value = self.stack.borrow_mut().pop_value();
                self.stack.borrow_mut().push_value(if predicate {
                    true_value
                } else {
                    false_value
                });
            }
            Instruction::LocalGet(idx) => {
                let value = self.current_function_state.borrow().get_local_value(*idx);
                self.stack.borrow_mut().push_value(value);
            }
            Instruction::LocalSet(idx) => {
                let value = self.stack.borrow_mut().pop_value();
                self.current_function_state
                    .borrow_mut()
                    .set_local_value(*idx, value);
            }
            Instruction::LocalTee(idx) => {
                let value = self.stack.borrow_mut().pop_value();
                self.current_function_state
                    .borrow_mut()
                    .set_local_value(*idx, value);
                self.stack.borrow_mut().push_value(value);
            }

            Instruction::GlobalGet(idx) => {
                let value = self.globals.borrow().get(*idx);
                self.stack.borrow_mut().push_value(value);
            }
            Instruction::GlobalSet(idx) => {
                let value = self.stack.borrow_mut().pop_value();
                self.globals.borrow_mut().set(value, *idx);
            }
            Instruction::TableGet(table_idx) => {
                let index_in_table = self.stack.borrow_mut().pop_table_element_idx();
                let ref_value = self
                    .tables
                    .borrow_mut()
                    .table(*table_idx)
                    .get(index_in_table);

                self.stack.borrow_mut().push_ref(ref_value);
            }

            Instruction::TableSet(table_idx) => {
                let ref_value = self.stack.borrow_mut().pop_ref();
                let index_in_table = self.stack.borrow_mut().pop_table_element_idx();
                self.tables
                    .borrow_mut()
                    .table(*table_idx)
                    .set(index_in_table, ref_value);
            }

            Instruction::I32Load(memarg) => memory_load!(self, i32, load_i32, memarg),
            Instruction::I64Load(memarg) => memory_load!(self, i64, load_i64, memarg),
            Instruction::F32Load(memarg) => memory_load!(self, f32, load_f32, memarg),
            Instruction::F64Load(memarg) => memory_load!(self, f64, load_f64, memarg),

            Instruction::I32Load8S(memarg) => memory_load!(self, i32, load_i32_8, memarg),
            Instruction::I32Load8U(memarg) => memory_load!(self, u32, load_u32_8, memarg),
            Instruction::I32Load16S(memarg) => memory_load!(self, i32, load_i32_16, memarg),
            Instruction::I32Load16U(memarg) => memory_load!(self, u32, load_u32_16, memarg),

            Instruction::I64Load8S(memarg) => memory_load!(self, i64, load_i64_8, memarg),
            Instruction::I64Load8U(memarg) => memory_load!(self, u64, load_u64_8, memarg),
            Instruction::I64Load16S(memarg) => memory_load!(self, i64, load_i64_16, memarg),
            Instruction::I64Load16U(memarg) => memory_load!(self, u64, load_u64_16, memarg),
            Instruction::I64Load32S(memarg) => memory_load!(self, i64, load_i64_32, memarg),
            Instruction::I64Load32U(memarg) => memory_load!(self, u64, load_u64_32, memarg),

            Instruction::I32Store(memarg) => memory_store!(self, i32, store_i32, memarg),
            Instruction::I64Store(memarg) => memory_store!(self, i64, store_i64, memarg),
            Instruction::F32Store(memarg) => memory_store!(self, f32, store_f32, memarg),
            Instruction::F64Store(memarg) => memory_store!(self, f64, store_f64, memarg),

            Instruction::I32Store8(memarg) => memory_store!(self, i32, store_i32_8, memarg),
            Instruction::I32Store16(memarg) => memory_store!(self, i32, store_i32_16, memarg),
            Instruction::I64Store8(memarg) => memory_store!(self, i64, store_i64_8, memarg),
            Instruction::I64Store16(memarg) => memory_store!(self, i64, store_i64_16, memarg),
            Instruction::I64Store32(memarg) => memory_store!(self, i64, store_i64_32, memarg),

            Instruction::MemorySize => self
                .stack
                .borrow_mut()
                .push_u32(self.memory.borrow().size()),
            Instruction::MemoryGrow => {
                let delta = self.stack.borrow_mut().pop_u32();
                self.stack
                    .borrow_mut()
                    .push_i32(self.memory.borrow_mut().grow(delta));
            }

            Instruction::I32Const(value) => self.stack.borrow_mut().push_i32(*value),
            Instruction::I64Const(value) => self.stack.borrow_mut().push_i64(*value),
            Instruction::F32Const(value) => self.stack.borrow_mut().push_f32(*value),
            Instruction::F64Const(value) => self.stack.borrow_mut().push_f64(*value),

            Instruction::I32Eqz => op!(self, { a: i32, }, bool => a == 0),
            Instruction::I32Eq => op!(self, { b: i32, a: i32 }, bool => a == b),
            Instruction::I32Ne => op!(self, { b: i32, a: i32 }, bool => a != b),
            Instruction::I32LtS => op!(self, { b: i32, a: i32 }, bool => a < b),
            Instruction::I32LtU => op!(self, { b: u32, a: u32 }, bool => a < b),
            Instruction::I32GtS => op!(self, { b: i32, a: i32 }, bool => a > b),
            Instruction::I32GtU => op!(self, { b: u32, a: u32 }, bool => a > b),
            Instruction::I32LeS => op!(self, { b: i32, a: i32 }, bool => a <= b),
            Instruction::I32LeU => op!(self, { b: u32, a: u32 }, bool => a <= b),
            Instruction::I32GeS => op!(self, { b: i32, a: i32 }, bool => a >= b),
            Instruction::I32GeU => op!(self, { b: u32, a: u32 }, bool => a >= b),
            Instruction::I64Eqz => op!(self, { a: i64 }, bool => a == 0),
            Instruction::I64Eq => op!(self, { b:i64, a: i64 }, bool => a == b),
            Instruction::I64Ne => op!(self, { b: i64, a: i64 }, bool => a != b),
            Instruction::I64LtS => op!(self, { b: i64, a: i64 }, bool => a < b),
            Instruction::I64LtU => op!(self, { b: u64, a: u64 }, bool => a < b),
            Instruction::I64GtS => op!(self, { b: i64, a: i64 }, bool => a > b),
            Instruction::I64GtU => op!(self, { b: u64, a: u64 }, bool => a > b),
            Instruction::I64LeS => op!(self, { b: i64, a: i64 }, bool => a <= b),
            Instruction::I64LeU => op!(self, { b: u64, a: u64 }, bool => a <= b),
            Instruction::I64GeS => op!(self, { b: i64, a: i64 }, bool => a >= b),
            Instruction::I64GeU => op!(self, { b: u64, a: u64 }, bool => a >= b),
            Instruction::F32Eq => op!(self, { b: f32, a: f32 }, bool => a == b),
            Instruction::F32Ne => op!(self, { b: f32, a: f32 }, bool => a != b),
            Instruction::F32Lt => op!(self, { b: f32, a: f32 }, bool => a < b),
            Instruction::F32Gt => op!(self, { b: f32, a: f32 }, bool => a > b),
            Instruction::F32Le => op!(self, { b: f32, a: f32 }, bool => a <= b),
            Instruction::F32Ge => op!(self, { b: f32, a: f32 }, bool => a >= b),
            Instruction::I32Add => op!(self, { b: i32, a: i32 }, i32 => a.wrapping_add(b)),
            Instruction::I32Sub => op!(self, { b: i32, a: i32 }, i32 => a.wrapping_sub(b)),
            Instruction::I32Mul => op!(self, { b: i32, a: i32 }, i32 => a.wrapping_mul(b)),
            Instruction::I32DivS => op!(self, { b: i32, a: i32 }, i32 => a / b),
            Instruction::I32DivU => op!(self, { b: u32, a: u32 }, u32 => a / b),
            Instruction::I32RemS => op!(self, { b: i32, a: i32 }, i32 => a % b),
            Instruction::I32RemU => op!(self, { b: u32, a: u32 }, u32 => a % b),
            Instruction::I32And => op!(self, { b: i32, a: i32 }, i32 => a & b),
            Instruction::I32Or => op!(self, { b: i32, a: i32 }, i32 => a | b),
            Instruction::I32Xor => op!(self, { b: i32, a: i32 }, i32 => a ^ b),
            Instruction::I32Shl => op!(self, { b: i32, a: i32 }, i32 => a << (b % 32)),
            Instruction::I32ShrS => op!(self, { b: i32, a: i32 }, i32 => a >> (b % 32)),
            Instruction::I32ShrU => op!(self, { b: u32, a: u32 }, u32 => a >> (b % 32)),
            Instruction::I32Rotr => op!(self, { b: u32, a: u32 }, u32 => a.rotate_right(b)),
            Instruction::I32Rotl => op!(self, { b: u32, a: u32 }, u32 => a.rotate_left(b)),
            Instruction::I64Add => op!(self, { b: i64, a: i64 }, i64 => a.wrapping_add(b)),
            Instruction::I64Sub => op!(self, { b: i64, a: i64 }, i64 => a.wrapping_sub(b)),
            Instruction::I64Mul => op!(self, { b: i64, a: i64 }, i64 => a.wrapping_mul(b)),
            Instruction::I64DivS => op!(self, { b: i64, a: i64 }, i64 => a / b),
            Instruction::I64RemS => op!(self, { b: i64, a: i64 }, i64 => a % b),
            Instruction::I64RemU => op!(self, { b: u64, a: u64 }, u64 => a % b),
            Instruction::I64And => op!(self, { b: i64, a: i64 }, i64 => a & b),
            Instruction::I64Or => op!(self, { b: i64, a: i64 }, i64 => a | b),
            Instruction::I64Xor => op!(self, { b: i64, a: i64 }, i64 => a ^ b),
            Instruction::I64ShrS => op!(self, { b: i64, a: i64 }, i64 => a >> (b % 64)),
            Instruction::I64ShrU => op!(self, { b: u64, a: u64 }, u64 => a >> (b % 64)),
            Instruction::I64Shl => op!(self, { b: i64, a: i64 }, i64 => a << (b % 64)),
            Instruction::F32Abs => op!(self, { a: f32 }, f32 => a.abs()),
            Instruction::F32Add => op!(self, { b: f32, a: f32 }, f32 => a + b),
            Instruction::F32Sub => op!(self, { b: f32, a: f32 }, f32 => a - b),
            Instruction::F32Mul => op!(self, { b: f32, a: f32 }, f32 => a * b),
            Instruction::F32Div => op!(self, { b: f32, a: f32 }, f32 => a / b),
            Instruction::F32Sqrt => op!(self, { a: f32 }, f32 => a.sqrt()),
            Instruction::F32Copysign => op!(self, { b: f32, a: f32 }, f32 => a .copysign(b)),
            Instruction::F64Add => op!(self, { b: f64, a: f64 }, f64 => a + b),
            Instruction::F64Sub => op!(self, { b: f64, a: f64 }, f64 => a - b),
            Instruction::F64Mul => op!(self, { b: f64, a: f64 }, f64 => a * b),
            Instruction::F64Div => op!(self, { b: f64, a: f64 }, f64 => a / b),
            Instruction::F64Neg => op!(self, { a: f64 }, f64 => -a),
            Instruction::I32WrapI64 => op!(self, { a: i64 }, i32 => a as i32),
            Instruction::I32TruncF32S => op!(self, { a: f32 }, i32 => a as i32),
            Instruction::I32TruncF64S => op!(self, { a: f64 }, i32 => a as i32),
            Instruction::I64ExtendI32S => op!(self, { a: i32 }, i64 => a as i64),
            Instruction::I64ExtendI32U => op!(self, { a: u32 }, u64 => a as u64),
            Instruction::I64TruncF32S => op!(self, { a: f32 }, i64 => a as i64),
            Instruction::I64TruncF32U => op!(self, { a: f32 }, u64 => a as u64),
            Instruction::I64TruncF64S => op!(self, { a: f64 }, i64 => a as i64),
            Instruction::I64TruncF64U => op!(self, { a: f64 }, u64 => a as u64),
            Instruction::F32DemoteF64 => op!(self, { a: f64 }, f32 => a as f32),
            Instruction::F32ConvertI32S => op!(self, { a: i32 }, f32 => a as f32),
            Instruction::F64ConvertI32S => op!(self, { a: i32 }, f64 => a as f64),
            Instruction::F64ConvertI64U => op!(self, { a: u64 }, f64 => a as f64),
            Instruction::I32ReinterpretF32 => {
                op!(self, { a: f32 }, i32 => i32::from_le_bytes(a.to_le_bytes()))
            }
            Instruction::F32ReinterpretI32 => {
                op!(self, { a: i32 }, f32 => f32::from_le_bytes(a.to_le_bytes()))
            }
            Instruction::F64ReinterpretI64 => {
                op!(self, { a: i64 }, f64 => f64::from_le_bytes(a.to_le_bytes()))
            }
            Instruction::I32Extend16S => op!(self, { a: i32 }, i32 => (a & 0xFFFF) as i16 as i32),

            Instruction::PushFuncRef(func) => self.stack.borrow_mut().push_ref(Some(*func)),

            Instruction::Memcpy => {
                let len = self.stack.borrow_mut().pop_u32() as usize;
                let src = self.stack.borrow_mut().pop_u32() as usize;
                let dst = self.stack.borrow_mut().pop_u32() as usize;
                self.memory.borrow_mut().cpy(src, dst, len);
            }
            Instruction::Memfill => {
                let len = self.stack.borrow_mut().pop_u32() as usize;
                let value = self.stack.borrow_mut().pop_u32() as u8;
                let addr = self.stack.borrow_mut().pop_u32() as usize;
                self.memory.borrow_mut().fill_value(len, addr, value);
            }
            _ => panic!("Instruction: {:?} not implemented ", instruction,),
        }
    }
}
