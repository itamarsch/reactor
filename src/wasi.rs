use std::{
    cell::RefMut,
    io::{stderr, stdout, IoSlice, Write},
    ops::Range,
    process::exit,
};

use nom::{
    combinator::cut, multi::count, number::complete::le_u32, sequence::pair, IResult, Parser,
};
mod error;

use crate::runtime::{memory::Memory, stack::Stack};

use self::error::WasiError;

#[derive(Debug, Clone, Copy)]
struct IOVec {
    address: usize,
    size: usize,
}
impl IOVec {
    pub fn parse(input: &[u8]) -> IResult<&[u8], IOVec> {
        let (input, (address, size)) = pair(le_u32, le_u32)(input)?;
        Ok((
            input,
            IOVec {
                address: address as usize,
                size: size as usize,
            },
        ))
    }

    pub fn as_range(&self) -> Range<usize> {
        self.address..self.address + self.size
    }
}

pub struct Wasi {
    args: Vec<String>,
    envs: Vec<String>,
}

impl Wasi {
    pub fn run_function(
        &mut self,
        function_name: &str,
        mut stack: RefMut<Stack>,
        mut memory: RefMut<Memory>,
    ) {
        match function_name {
            "proc_exit" => {
                let exit_code = stack.pop_i32();
                exit(exit_code);
            }
            "fd_write" => {
                let n_written_addr = stack.pop_u32();
                let amount_of_iovs = stack.pop_u32() as usize;
                let iov_addr = stack.pop_u32() as usize;
                let fd = stack.pop_i32();

                let (_, iovs) =
                    count(
                        cut(IOVec::parse.map(|iov| IoSlice::new(memory.get_range(iov.as_range())))),
                        amount_of_iovs,
                    )(memory.get_range(iov_addr..iov_addr + amount_of_iovs * 8))
                    .unwrap();

                let result = self.write_iovs(fd, &iovs);
                let (n_written, result) = match result {
                    Ok(n) => (n as u32, WasiError::Success),
                    Err(e) => (0, e),
                };

                memory.store_u32(n_written, n_written_addr);
                stack.push_i32(result as u16 as i32);
            }
            "args_sizes_get" => {
                let buf_size_addr = stack.pop_u32();
                let argc_addr = stack.pop_u32();

                // Calculate argc (number of arguments)
                let argc = self.args.len() as u32;

                // Calculate argv_buf_size (total size of all arguments including null terminators)
                let argv_buf_size: u32 = self
                    .args
                    .iter()
                    .map(|arg| arg.len() as u32 + 1) // +1 for the null terminator
                    .sum();

                // Store argc at the specified memory address
                memory.store_u32(argc, argc_addr);

                // Store argv_buf_size at the specified memory address
                memory.store_u32(argv_buf_size, buf_size_addr);

                // Push the success error code (0) onto the stack
                stack.push_i32(0);
            }
            "args_get" => {
                let argv_buf_addr = stack.pop_u32();
                let argv_ptr = stack.pop_u32();
                let mut current_offset = 0;

                for (i, arg) in self.args.iter().enumerate() {
                    // Calculate the address where the argument string will be stored
                    let arg_str_addr = argv_buf_addr + current_offset;

                    // Calculate where to store the pointer to the argument string
                    let arg_ptr_addr = argv_ptr + (i as u32 * 4); // 4 bytes per pointer (assuming 32-bit pointers)

                    // Write the pointer to the argument string into argv[i]
                    memory.store_u32(arg_str_addr, arg_ptr_addr);

                    // Write the argument string into argv_buf
                    memory.fill_data(arg_str_addr, arg.as_bytes());

                    // Add null terminator after the argument string
                    memory.store_i32_8(0, arg_str_addr + arg.len() as u32, Default::default());

                    // Update current_offset (length of arg + 1 for null terminator)
                    current_offset += arg.len() as u32 + 1;
                }

                // Push success code onto the stack
                stack.push_i32(0);
            }
            "environ_sizes_get" => {
                // Pop the addresses from the stack
                let environ_buf_size_addr = stack.pop_u32(); // Address to store environ_buf_size
                let environ_count_addr = stack.pop_u32(); // Address to store environ_count

                // Calculate environ_count (number of environment variables)
                let environ_count = self.envs.len() as u32;

                // Calculate environ_buf_size (total size of all environment variables including null terminators)
                let environ_buf_size: u32 = self
                    .envs
                    .iter()
                    .map(|env| env.len() as u32 + 1) // +1 for the null terminator
                    .sum();

                // Store environ_count at the specified memory address
                memory.store_u32(environ_count, environ_count_addr);

                // Store environ_buf_size at the specified memory address
                memory.store_u32(environ_buf_size, environ_buf_size_addr);

                // Push the success error code (0) onto the stack
                stack.push_i32(0);
            }
            "environ_get" => {
                let environ_buf_addr = stack.pop_u32();
                let environ_ptr = stack.pop_u32();

                let mut current_offset = 0;

                for (i, env) in self.envs.iter().enumerate() {
                    let env_str_addr = environ_buf_addr + current_offset;
                    let env_ptr_addr = environ_ptr + (i as u32 * 4);

                    memory.store_u32(env_str_addr, env_ptr_addr);
                    memory.fill_data(env_str_addr, env.as_bytes());
                    memory.store_i32_8(0, env_str_addr + env.len() as u32, Default::default());

                    current_offset += env.len() as u32 + 1;
                }

                stack.push_i32(0);
            }
            _ => {
                panic!("Unknown wasi function: {}", function_name);
            }
        }
    }

    fn write_iovs(&self, fd: i32, iovs: &[IoSlice]) -> Result<usize, WasiError> {
        match fd {
            1 => stdout()
                .write_vectored(iovs)
                .map_err(|err| err.kind().into()),
            2 => stderr()
                .write_vectored(iovs)
                .map_err(|err| err.kind().into()),

            _ => {
                panic!("Haven't implemented writing to files {}", fd)
            }
        }
    }

    pub fn new() -> Self {
        let mut args = std::env::args();
        args.next();
        let args = args.collect::<Vec<_>>();

        let envs = std::env::vars()
            .map(|e| format!("{}={}", e.0, e.1))
            .collect::<Vec<_>>();

        println!("{:?}", args);
        Self { args, envs }
    }
}

impl Default for Wasi {
    fn default() -> Self {
        Self::new()
    }
}
