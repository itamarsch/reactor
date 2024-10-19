use std::{
    cell::RefMut,
    io::{stdout, IoSlice, Write},
    ops::DerefMut,
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
                        IOVec::parse.map(|iov| {
                            IoSlice::new(memory.get_range(iov.address..iov.address + iov.size))
                        }),
                        amount_of_iovs,
                    )(memory.get_range(iov_addr..iov_addr + amount_of_iovs * 8))
                    .unwrap();
                let result = self.write_iovs(fd, &iovs);
                write_result(n_written_addr, memory.deref_mut(), result);
            }
            _ => {
                panic!("Unknown wasi function: {}", function_name);
            }
        }
    }

    fn write_iovs(&self, fd: i32, iovs: &[IoSlice]) -> Result<u32, WasiError> {
        match fd {
            1 => Ok(stdout().write_vectored(iovs).map_err(|err| err.kind())? as u32),
            _ => {
                panic!("Haven't implemented writing to files")
            }
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}

fn write_result(addr: u32, memory: &mut Memory, result: Result<u32, error::WasiError>) {
    match result {
        Ok(n) => {
            const VARIANT: u32 = 0;
            memory.store_u32(VARIANT, addr);
            memory.store_u32(n, addr + 4);
        }
        Err(err) => {
            const VARIANT: u32 = 1;
            let err = err as u16;

            memory.store_u32(VARIANT, addr);
            memory.store_u16(err, addr);
        }
    }
}

impl Default for Wasi {
    fn default() -> Self {
        Self::new()
    }
}
