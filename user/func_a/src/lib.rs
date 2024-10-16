#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use ms_std_proc_macro::FaasData;

const DATA_SIZE: usize = 1024 * 1024 * 256;

#[derive(FaasData)]
struct VecArg {
    data: [u8; DATA_SIZE],
}

impl Default for VecArg {
    fn default() -> Self {
        Self {
            data: [1; DATA_SIZE],
        }
    }
}

#[allow(clippy::result_unit_err)]
#[no_mangle]
pub fn main() -> Result<()> {
    let mut d = DataBuffer::<VecArg>::with_slot("Conference".to_owned());
    
    // for (_, val) in &mut d.data.iter_mut().enumerate() {
    //     *val = 0 as u8
    // }
    // let mut x = VecArg::default();
    // x.data = [1; DATA_SIZE];
    d.data = [1; DATA_SIZE];
    // let access_end = SystemTime::now().duration_since(UNIX_EPOCH).as_micros();
    // println!("write_dur={}", access_end - register_start);
    // println!("write_dur={}", access_end - register_start);
    let register_start = SystemTime::now().duration_since(UNIX_EPOCH).as_micros();
    let result = DataBuffer::<VecArg>::from_buffer_slot("Conference".to_owned());
    // let access_end = SystemTime::now().duration_since(UNIX_EPOCH).as_micros();
    // println!("read_dur={}", access_end - register_start);
    // println!("{}", x);
    // d.data = [0; DATA_SIZE];
    // println!("{}", result.unwrap().data[0]);
    if let Some(buffer) = result {
        for i in 0..buffer.data.len() {
                    let _ = unsafe { core::ptr::read_volatile((&buffer.data[i]) as *const u8) };
        }
    }
    let access_end = SystemTime::now().duration_since(UNIX_EPOCH).as_micros();
    println!("opt_dur={}", access_end - register_start);

    // if let Some(buffer) = data {
    //     for i in 0..buffer.data.len() {
    //         let _ = unsafe { core::ptr::read_volatile((&buffer.data[i]) as *const u8) };
    //     }
    // } else {
    //     Err("buffer is none")?
    // }
    Ok(().into())
}
