use std::ops::Range;

use crate::types::{Limit, MemoryArgument};
use paste::paste;

const PAGE_SIZE: usize = 65536;

pub struct Memory {
    data: Vec<u8>,
    limits: Limit,
}

macro_rules! define_load_function {
    ($func_name:ident, $ty:ty) => {
        pub fn $func_name(&self, address_raw: u32, memarg: MemoryArgument) -> $ty {
            let address = Memory::apply_memarg(address_raw, memarg) as usize;
            let bytes = &self.data[address..address + std::mem::size_of::<$ty>()];
            <$ty>::from_le_bytes(bytes.try_into().unwrap())
        }
    };
}

// Basic store functions
macro_rules! define_store_function {
    ($func_name:ident, $ty:ty) => {
        pub fn $func_name(&mut self, value: $ty, address_raw: u32, memarg: MemoryArgument) {
            let address = Memory::apply_memarg(address_raw, memarg) as usize;
            let bytes = value.to_le_bytes();
            self.data[address..address + bytes.len()].copy_from_slice(&bytes);
        }
    };
}

macro_rules! define_load_ext_function_signed {
    ($func_name:ident, $ty:ty, $num_bits:expr) => {
        pub fn $func_name(
            &mut self,
            address_raw: u32,
            memarg: MemoryArgument,
        ) -> $ty {
            let address = Memory::apply_memarg(address_raw, memarg);
            const NUM_BYTES: usize = $num_bits / 8;
            paste! {
                [<i $num_bits>]::from_le_bytes(self.data[address..address + NUM_BYTES].try_into().unwrap()) as $ty
            }
        }
    };
}

macro_rules! define_load_ext_function_unsigned {
    ($func_name:ident, $ty:ty, $num_bits:expr) => {
        pub fn $func_name(
            &mut self,
            address_raw: u32,
            memarg: MemoryArgument,
        ) -> $ty {
            let address = Memory::apply_memarg(address_raw, memarg);
            const NUM_BYTES: usize = $num_bits / 8;
            paste! {
                [<u $num_bits>]::from_le_bytes(self.data[address..address + NUM_BYTES].try_into().unwrap()) as $ty
            }
        }
    };
}

// Extended store functions
macro_rules! define_store_ext_function {
    ($func_name:ident, $value_ty:ty, $num_bytes:expr) => {
        pub fn $func_name(&mut self, value: $value_ty, address_raw: u32, memarg: MemoryArgument) {
            let address = Memory::apply_memarg(address_raw, memarg) as usize;
            let bytes = value.to_le_bytes();
            self.data[address..address + $num_bytes].copy_from_slice(&bytes[..$num_bytes]);
        }
    };
}

// Use the macros to define all required functions

impl Memory {
    pub fn new(limit: Limit) -> Memory {
        Memory {
            data: vec![0; limit.min as usize * PAGE_SIZE],
            limits: limit,
        }
    }

    pub fn grow(&mut self, amount_of_pages: u32) -> i32 {
        let prev_size = (self.data.len() / PAGE_SIZE) as u32;
        let new_size = prev_size + amount_of_pages;
        if self.limits.max.is_some_and(|max| new_size > max) {
            return -1;
        }

        self.data
            .extend(std::iter::repeat(0).take(amount_of_pages as usize * PAGE_SIZE));

        prev_size as i32
    }

    fn apply_memarg(address_raw: u32, memarg: MemoryArgument) -> usize {
        let address_raw: usize = address_raw as usize;

        address_raw + memarg.offset as usize
    }

    pub fn fill_data(&mut self, address: u32, data: &[u8]) {
        let address = address as usize;
        self.data[address..address + data.len()].copy_from_slice(data);
    }

    pub fn get_range(&self, range: Range<usize>) -> &[u8] {
        &self.data[range]
    }

    define_load_function!(load_i32, i32);
    define_load_function!(load_i64, i64);
    define_load_function!(load_f32, f32);
    define_load_function!(load_f64, f64);

    define_load_ext_function_signed!(load_i32_8, i32, 8);
    define_load_ext_function_signed!(load_i32_16, i32, 16);

    define_load_ext_function_signed!(load_i64_8, i64, 8);
    define_load_ext_function_signed!(load_i64_16, i64, 16);
    define_load_ext_function_signed!(load_i64_32, i64, 32);

    define_load_ext_function_unsigned!(load_u32_8, u32, 8);
    define_load_ext_function_unsigned!(load_u32_16, u32, 16);

    define_load_ext_function_unsigned!(load_u64_8, u64, 8);
    define_load_ext_function_unsigned!(load_u64_16, u64, 16);
    define_load_ext_function_unsigned!(load_u64_32, u64, 32);

    define_store_function!(store_i32, i32);

    pub fn store_u32(&mut self, value: u32, addr: u32) {
        self.store_i32(
            i32::from_le_bytes(value.to_le_bytes()),
            addr,
            MemoryArgument::default(),
        );
    }
    pub fn store_u16(&mut self, value: u16, addr: u32) {
        let value = value as u32;
        self.store_i32_16(
            i32::from_le_bytes(value.to_le_bytes()),
            addr,
            MemoryArgument::default(),
        );
    }

    define_store_function!(store_i64, i64);
    define_store_function!(store_f32, f32);
    define_store_function!(store_f64, f64);

    define_store_ext_function!(store_i32_8, i32, 1);
    define_store_ext_function!(store_i32_16, i32, 2);
    define_store_ext_function!(store_i64_8, i64, 1);
    define_store_ext_function!(store_i64_16, i64, 2);
    define_store_ext_function!(store_i64_32, i64, 4);

    pub fn cpy(&mut self, src: usize, dst: usize, len: usize) {
        let mut buf = vec![0; len];
        buf.copy_from_slice(&self.data[src..src + len]);
        self.data[dst..dst + len].copy_from_slice(&buf);
    }

    pub fn size(&self) -> u32 {
        (self.data.len() / PAGE_SIZE) as u32
    }
}
