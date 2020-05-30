use std::result::Result;
use std::option::Option;

#[repr(C)]
#[derive(Debug)]
pub struct xyz {
    x: u32, y: u32, z: f32
}

use std::convert::TryInto;

#[no_mangle]
pub unsafe extern "C" fn parse_xyz_data(ptr: *const xyz, length: u32) {
    let v = std::slice::from_raw_parts(ptr, length.try_into().unwrap());
    println!("The length of the read-in slice is: {}", v.len());
    for i in 0..v.len() {
        println!("{:?}",v[i]);
    }
}