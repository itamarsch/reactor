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
    types::{BlockIdx, ElementMode, Expr, FuncIdx, Instruction, ValueType},
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

macro_rules! numeric_operation {
    (
        $self:expr,
        pops { $( $ident:ident : $type:ident ),* $(,)? },
        push $result_type:ident => $expr:expr
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

    pub fn run_expr<T>(&self, expr: Expr, mut get_result_after_expr: impl FnMut() -> T) -> T {
        let idx = self.module.borrow_mut().add_expr(expr);

        // Swaped in next line
        let mut function_state_before_expr = FunctionState::new_function(Locals::empty(), idx);
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
        self.module.borrow_mut().remove_expr(idx);

        result
    }

    fn initilize_elements(&self) {
        let mut module_borrow = self.module.borrow_mut();
        let elements = module_borrow.elements();
        drop(module_borrow);

        for element in elements {
            match element.mode {
                ElementMode::Declarative => {}
                ElementMode::Passive => {
                    todo!()
                }
                ElementMode::Active {
                    table,
                    offset_in_table,
                } => {
                    let offset = self.run_expr(offset_in_table, || {
                        TableElementIdx(self.stack.borrow_mut().pop_u32() as usize)
                    });
                    let refs = element
                        .init
                        .into_iter()
                        .map(|init| self.run_expr(init, || self.stack.borrow_mut().pop_ref()))
                        .collect::<Vec<_>>();

                    self.tables.borrow_mut().table(table).fill(offset, &refs);
                }
            }
        }
    }

    fn initialize_globals(&self) {
        let borrow = self.module.borrow();

        let initializers = borrow.global_initializers();
        let initializers = initializers.to_vec();
        drop(borrow);

        let globals = initializers
            .iter()
            .map(|global| {
                let value = self.run_expr(global.init.clone(), || {
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
        let mut offset_calulations_to_run = vec![];
        for (i, data) in self.module.borrow().datas().iter().enumerate() {
            match data.mode {
                crate::types::DataMode::Passive => continue,
                crate::types::DataMode::Active { ref offset, .. } => {
                    offset_calulations_to_run.push((offset.clone(), i))
                }
            }
        }
        for (offset_instructions, data_index) in offset_calulations_to_run {
            let offset = self.run_expr(offset_instructions, || self.stack.borrow_mut().pop_u32());
            self.memory
                .borrow_mut()
                .fill_data(offset, &self.module.borrow().datas()[data_index].init);
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

            Instruction::I32Const(value) => {
                self.stack.borrow_mut().push_i32(*value);
            }
            Instruction::I64Const(value) => {
                self.stack.borrow_mut().push_i64(*value);
            }
            Instruction::F32Const(value) => {
                self.stack.borrow_mut().push_f32(*value);
            }
            Instruction::F64Const(value) => {
                self.stack.borrow_mut().push_f64(*value);
            }
            Instruction::I32Eqz => {
                numeric_operation!(self,
                    pops { a: i32, },
                    push bool => a == 0
                );
            }
            Instruction::I32Eq => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push bool => a == b
                );
            }
            Instruction::I32Ne => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push bool => a != b
                );
            }
            Instruction::I32LtS => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push bool => a < b
                );
            }
            Instruction::I32LtU => {
                numeric_operation!(self,
                    pops { b: u32, a: u32 },
                    push bool => a < b
                );
            }
            Instruction::I32GtS => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push bool => a > b
                );
            }
            Instruction::I32GtU => {
                numeric_operation!(self,
                    pops { b: u32, a: u32 },
                    push bool => a > b
                );
            }
            Instruction::I32LeS => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push bool => a <= b
                );
            }
            Instruction::I32LeU => {
                numeric_operation!(self,
                    pops { b: u32, a: u32 },
                    push bool => a <= b
                );
            }
            Instruction::I32GeS => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push bool => a >= b
                );
            }
            Instruction::I32GeU => {
                numeric_operation!(self,
                    pops { b: u32, a: u32 },
                    push bool => a >= b
                );
            }
            Instruction::I64Eqz => {
                numeric_operation!(self,
                    pops { a: i64 },
                    push bool => a == 0
                );
            }
            Instruction::I64Eq => {
                numeric_operation!(self,
                    pops { b:i64, a: i64 },
                    push bool => a == b
                );
            }
            Instruction::I64Ne => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push bool => a != b
                );
            }
            Instruction::I64LtS => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push bool => a < b
                );
            }
            Instruction::I64LtU => {
                numeric_operation!(self,
                    pops { b: u64, a: u64 },
                    push bool => a < b
                );
            }
            Instruction::I64GtS => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push bool => a > b
                );
            }
            Instruction::I64GtU => {
                numeric_operation!(self,
                    pops { b: u64, a: u64 },
                    push bool => a > b
                );
            }
            Instruction::I64LeS => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push bool => a <= b
                );
            }
            Instruction::I64LeU => {
                numeric_operation!(self,
                    pops { b: u64, a: u64 },
                    push bool => a <= b
                );
            }
            Instruction::I64GeS => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push bool => a >= b
                );
            }
            Instruction::I64GeU => {
                numeric_operation!(self,
                    pops { b: u64, a: u64 },
                    push bool => a >= b
                );
            }
            Instruction::I32Add => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a.wrapping_add(b)
                );
            }
            Instruction::I32Sub => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a.wrapping_sub(b)
                );
            }
            Instruction::I32Mul => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a.wrapping_mul(b)
                );
            }
            Instruction::I32DivS => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a / b
                );
            }
            Instruction::I32DivU => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a / b
                );
            }
            Instruction::I32RemS => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a % b
                );
            }
            Instruction::I32RemU => {
                numeric_operation!(self,
                    pops { b: u32, a: u32 },
                    push u32 => a % b
                );
            }
            Instruction::I32And => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a & b
                );
            }
            Instruction::I32Or => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a | b
                );
            }
            Instruction::I32Xor => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a ^ b
                );
            }
            Instruction::I32Shl => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a << (b % 32)
                );
            }
            Instruction::I32ShrS => {
                numeric_operation!(self,
                    pops { b: i32, a: i32 },
                    push i32 => a >> (b % 32)
                );
            }
            Instruction::I32ShrU => {
                numeric_operation!(self,
                    pops { b: u32, a: u32 },
                    push u32 => a >> (b % 32)
                );
            }
            Instruction::I64Add => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push i64 => a.wrapping_add(b)
                );
            }
            Instruction::I64Sub => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push i64 => a.wrapping_sub(b)
                );
            }
            Instruction::I64Mul => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push i64 => a.wrapping_mul(b)
                );
            }
            Instruction::I64DivS => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push i64 => a / b
                );
            }
            Instruction::I64RemS => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push i64 => a % b
                );
            }
            Instruction::I64RemU => {
                numeric_operation!(self,
                    pops { b: u64, a: u64 },
                    push u64 => a % b
                );
            }
            Instruction::I64Or => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push i64 => a | b
                );
            }
            Instruction::I64ShrS => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push i64 => a >> (b % 64)
                );
            }
            Instruction::I64ShrU => {
                numeric_operation!(self,
                    pops { b: u64, a: u64 },
                    push u64 => a >> (b % 64)
                );
            }
            Instruction::I64Shl => {
                numeric_operation!(self,
                    pops { b: i64, a: i64 },
                    push i64 => a << (b % 64)
                );
            }
            Instruction::F32Add => {
                numeric_operation!(self,
                    pops { b: f32, a: f32 },
                    push f32 => a + b
                );
            }
            Instruction::F32Sub => {
                numeric_operation!(self,
                    pops { b: f32, a: f32 },
                    push f32 => a - b
                );
            }
            Instruction::F32Mul => {
                numeric_operation!(self,
                    pops { b: f32, a: f32 },
                    push f32 => a * b
                );
            }
            Instruction::F32Div => {
                numeric_operation!(self,
                    pops { b: f32, a: f32 },
                    push f32 => a / b
                );
            }
            Instruction::F32Sqrt => {
                numeric_operation!(self,
                    pops { a: f32 },
                    push f32 => a.sqrt()
                );
            }
            Instruction::F64Add => {
                numeric_operation!(self,
                    pops { b: f64, a: f64 },
                    push f64 => a + b
                );
            }
            Instruction::F64Sub => {
                numeric_operation!(self,
                    pops { b: f64, a: f64 },
                    push f64 => a - b
                );
            }
            Instruction::F64Mul => {
                numeric_operation!(self,
                    pops { b: f64, a: f64 },
                    push f64 => a * b
                );
            }
            Instruction::F64Div => {
                numeric_operation!(self,
                    pops { b: f64, a: f64 },
                    push f64 => a / b
                );
            }
            Instruction::I32WrapI64 => {
                numeric_operation!(self,
                    pops { a: i64 },
                    push i32 => a as i32
                );
            }
            Instruction::I32TruncF32S => {
                numeric_operation!(self,
                    pops { a: f32 },
                    push i32 => a as i32
                );
            }
            Instruction::I32TruncF64S => {
                numeric_operation!(self,
                    pops { a: f64 },
                    push i32 => a as i32
                );
            }
            Instruction::I64ExtendI32S => {
                numeric_operation!(self,
                    pops { a: i32 },
                    push i64 => a as i64
                );
            }
            Instruction::I64ExtendI32U => {
                numeric_operation!(self,
                    pops { a: u32 },
                    push u64 => a as u64
                );
            }
            Instruction::I64TruncF32S => {
                numeric_operation!(self,
                    pops { a: f32 },
                    push i64 => a as i64
                );
            }
            Instruction::I64TruncF32U => {
                numeric_operation!(self,
                    pops { a: f32 },
                    push u64 => a as u64
                );
            }
            Instruction::I64TruncF64S => {
                numeric_operation!(self,
                    pops { a: f64 },
                    push i64 => a as i64
                );
            }
            Instruction::I64TruncF64U => {
                numeric_operation!(self,
                    pops { a: f64 },
                    push u64 => a as u64
                );
            }
            Instruction::F32ConvertI32S => {
                numeric_operation!(self,
                    pops { a: i32 },
                    push f32 => a as f32
                );
            }
            Instruction::F64ConvertI32S => {
                numeric_operation!(self,
                    pops { a: i32 },
                    push f64 => a as f64
                );
            }
            Instruction::Memcpy => {
                let len = self.stack.borrow_mut().pop_u32() as usize;
                let src = self.stack.borrow_mut().pop_u32() as usize;
                let dst = self.stack.borrow_mut().pop_u32() as usize;
                self.memory.borrow_mut().cpy(src, dst, len);
            }

            Instruction::PushFuncRef(func) => self.stack.borrow_mut().push_ref(Some(*func)),
            _ => panic!("Instruction: {:?} not implemented ", instruction,),
        }
    }
}
