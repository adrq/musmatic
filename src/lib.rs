/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. 
 * 
 * Copyright (c) 2019
 * Authors: Adrian Quiroga
 */
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};


include!(concat!(env!("OUT_DIR"), "/verovio_bindings.rs"));


fn new_instance(resource_path: String) -> (*mut std::ffi::c_void, String) {
    let res_path = CString::new(resource_path).expect("Unable to create CString");
    let vrv_ptr = unsafe {vrvToolkit_constructorResourcePath(res_path.as_ptr())};
    let vrv_options = unsafe { CStr::from_ptr(vrvToolkit_getAvailableOptions(vrv_ptr)).to_str().expect("fail")};
    println!("{}",vrv_options);
    return (vrv_ptr,vrv_options.to_string());
}

#[no_mangle]
pub extern fn initialize()

#[no_mangle]
pub extern fn render_mei_data(mei_data:*const i8) -> *mut u8{
    let mut res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_path.push("vendor/verovio/data");
    let vrv_ptr: *mut std::ffi::c_void;
    let mut vrv_options = String::new();
    let (vrv_ptr, vrv_options) = crate::new_instance(res_path.to_str().unwrap().to_string());
    let options = CString::new("{}").expect("render_mei_data unable to create CString");
    let svg_data_ptr = vrvToolkit_renderData(vrv_ptr,mei_data,options.as_ptr());
    let svg_data_str = CString::from_ptr(svg_data_ptr).to_str(); 

    return svg_data;
}

#[cfg(test)]
mod tests {

    use std::ffi::CString;
use std::ffi::CStr;
use std::path::PathBuf;

include!(concat!(env!("OUT_DIR"), "/verovio_bindings.rs"));
    #[test]
    fn constructor_works() {
        let mut res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        res_path.push("vendor/verovio/data");
        let vrv_ptr: *mut std::ffi::c_void;
        let mut vrv_options = String::new();
        let (vrv_ptr, vrv_options) = crate::new_instance(res_path.to_str().unwrap().to_string());
        assert_eq!(vrv_options.is_empty(), false);
    }

    fn options_valid() {

    }
}
