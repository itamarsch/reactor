use crate::types::{Limit, MemoryArgument};
use paste::paste;

const PAGE_SIZE: usize = 65536;

pub struct Memory {
    data: Vec<u8>,
    limits: Limit,
}

macro_rules! define_load_function {
    ($func_name:ident, $ty:ty) => {
        pub fn $func_name(&self, address_raw: i32, memarg: MemoryArgument) -> $ty {
            let address = Memory::apply_memarg(address_raw, memarg) as usize;
            let bytes = &self.data[address..address + std::mem::size_of::<$ty>()];
            <$ty>::from_le_bytes(bytes.try_into().unwrap())
        }
    };
}

// Basic store functions
macro_rules! define_store_function {
    ($func_name:ident, $ty:ty) => {
        pub fn $func_name(&mut self, value: $ty, address_raw: i32, memarg: MemoryArgument) {
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
            address_raw: i32,
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
            address_raw: i32,
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
        pub fn $func_name(&mut self, value: $value_ty, address_raw: i32, memarg: MemoryArgument) {
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

    fn apply_memarg(address_raw: i32, memarg: MemoryArgument) -> usize {
        let address_raw: usize = address_raw as usize;
        let address = address_raw + memarg.offset as usize;
        println!(
            "Address raw: {}, Memargs: {:?}, Address: {}",
            address_raw, memarg, address
        );
        address
    }

    pub fn fill_data(&mut self, address: i32, data: &[u8]) {
        println!("Filling offset: {}, with data: {:?}", address, data);
        let address = address as usize;
        self.data[address..address + data.len()].copy_from_slice(data);
    }

    // Use the macros to define all required functions
    // Basic load functions
    // Use the macros to define all required functions

    // Load functions
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
    // Store functions
    define_store_function!(store_i32, i32);
    define_store_function!(store_i64, i64);
    define_store_function!(store_f32, f32);
    define_store_function!(store_f64, f64);

    // Extended store functions
    define_store_ext_function!(store_i32_8, i32, 1);
    define_store_ext_function!(store_i32_16, i32, 2);
    define_store_ext_function!(store_i64_8, i64, 1);
    define_store_ext_function!(store_i64_16, i64, 2);
    define_store_ext_function!(store_i64_32, i64, 4);

    // ... other methods ...
}
