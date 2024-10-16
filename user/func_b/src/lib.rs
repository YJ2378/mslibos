#![no_std]

extern crate alloc;

use core::{mem::size_of, /* ptr */};

use alloc::{borrow::ToOwned};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use ms_std_proc_macro::FaasData;

const DATA_SIZE: usize = 1024 * 4;

#[derive(FaasData)]
pub struct MyComplexData {
    pub tem: usize,
    pub big_data: [u8; DATA_SIZE],
}

impl Default for MyComplexData {
    fn default() -> Self {
        Self {
            tem: Default::default(),
            big_data: [0; DATA_SIZE],
        }
    }
}

#[no_mangle]
#[allow(clippy::result_unit_err)]
pub fn main() -> Result<MyComplexData> {
    println!("func b");
    let data = DataBuffer::<MyComplexData>::from_buffer_slot("Conference".to_owned());
    if let Some(buffer) = data {
        for i in 0..buffer.big_data.len() {
            let _ = unsafe { core::ptr::read_volatile((&buffer.big_data[i]) as *const u8) };
        }
        Ok(buffer)
    } else {
        Err("buffer is none")?
    }
}
