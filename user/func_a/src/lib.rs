#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned};
use ms_std::{
    agent::{DataBuffer, FaaSFuncResult as Result},
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use ms_std_proc_macro::FaasData;

// const DATA_SIZE: usize = 1024 * 1024 * 256 / 8;
const DATA_SIZE: usize = 1024 * 4 / 8;

#[derive(FaasData)]
struct VecArg {
    data: [u64; DATA_SIZE],
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
    d.data = [1u64; DATA_SIZE];
    // let access_end = SystemTime::now().duration_since(UNIX_EPOCH).as_micros();
    // println!("write_dur={}", access_end - register_start);
    // println!("write_dur={}", access_end - register_start);
    let register_start = SystemTime::now().duration_since(UNIX_EPOCH).as_nanos();
    let result = DataBuffer::<VecArg>::from_buffer_slot("Conference".to_owned());
    let access_end1 = SystemTime::now().duration_since(UNIX_EPOCH).as_nanos();
    // println!("read_dur={}", access_end - register_start);
    // println!("{}", x);
    // d.data = [0; DATA_SIZE];
    // println!("{}", result.unwrap().data[0]);
    if let Some(buffer) = result {
        for i in 0..buffer.data.len() {
                    let _ = unsafe { core::ptr::read_volatile((&buffer.data[i]) as *const u64) };
        }
    }
    let access_end2 = SystemTime::now().duration_since(UNIX_EPOCH).as_nanos();
    println!("phase3.0: {}", register_start);
    unsafe {
        println!("phase3.1: {}", ms_std::agent::PHASE31);
        println!("phase3.2: {}", ms_std::agent::PHASE32);
        println!("phase3.3: {}", ms_std::agent::PHASE33);
        println!("phase3.4: {}", ms_std::agent::PHASE34);
    }
    println!("phase3.x: {}", access_end1);
    println!("phase3_dur={}", access_end1 - register_start);
    println!("phase4_dur={}", access_end2 - access_end1);
    println!("phase34_dur={}", access_end2 - register_start);
    let test_start = SystemTime::now().duration_since(UNIX_EPOCH).as_nanos();
    SystemTime::now().duration_since(UNIX_EPOCH).as_nanos();
    let test_end = SystemTime::now().duration_since(UNIX_EPOCH).as_nanos();
    println!("timestamp: {}", test_end - test_start);

    // if let Some(buffer) = data {
    //     for i in 0..buffer.data.len() {
    //         let _ = unsafe { core::ptr::read_volatile((&buffer.data[i]) as *const u8) };
    //     }
    // } else {
    //     Err("buffer is none")?
    // }
    Ok(().into())
}
