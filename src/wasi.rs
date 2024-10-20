use std::{
    cell::RefMut,
    io::{stdout, IoSlice, Write},
    ops::Range,
    process::exit,
};

use nom::{multi::count, number::complete::le_u32, sequence::pair, IResult, Parser};
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

pub struct Wasi {}

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
                        IOVec::parse.map(|iov| IoSlice::new(memory.get_range(iov.as_range()))),
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

            _ => {
                panic!("Haven't implemented writing to files")
            }
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Wasi {
    fn default() -> Self {
        Self::new()
    }
}
