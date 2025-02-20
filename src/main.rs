/*
 * This file is part of Musmatic Core
 *
 * Musmatic Core is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Musmatic Core is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Musmatic Core. If not, see <https://www.gnu.org/licenses/>.
 * 
 * Copyright (c) 2019-2025 - Musmatic Core authors
 */

use std::ffi::CString;
use std::ffi::CStr;
use std::io;
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::path::PathBuf;


include!("verovio_bindings.rs");


fn new_instance(resource_path: String) -> (*mut std::ffi::c_void, String) {
    let res_path = CString::new(resource_path).expect("Unable to create CString");
    let vrv_ptr = unsafe {vrvToolkit_constructorResourcePath(res_path.as_ptr())};
    let vrv_options = unsafe { CStr::from_ptr(vrvToolkit_getAvailableOptions(vrv_ptr)).to_str().expect("fail")};
    (vrv_ptr,vrv_options.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct VerovioOptions {
    options: HashMap<String,String>
}


fn main(){
    println!("Loading");
    let mut buffer = Vec::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_end(&mut buffer).unwrap();

    println!("{}",getsvg(buffer))
}


fn getsvg(input: Vec<u8>) -> String{
    let res: String = String::from_utf8(input).unwrap();
    println!("getsvg");
    let mut res_path = PathBuf::from("../../");
    res_path.push("vendor/verovio/data");
    let vrv_ptr: *mut std::ffi::c_void;
    let mut vrv_options = String::new();
    let (vrv_ptr, vrv_options) = new_instance(res_path.to_str().unwrap().to_string());

    let mut options = VerovioOptions{
        options: HashMap::new()
    };
    options.options.insert("noFooter".to_string(),"true".to_string());
    options.options.insert("noHeader".to_string(),"true".to_string());
    options.options.insert("adjustPageHeight".to_string(),"true".to_string());
    options.options.insert("breaks".to_string(),"none".to_string());
    options.options.insert("svgViewBox".to_string(),"true".to_string());
    let json_options = serde_json::to_string(&options.options).expect("fail");

    let options_str = CString::new(json_options).expect("render_mei_data unable to create CString");
    let options_ptr = options_str.as_ptr();
    std::mem::forget(options_str);
    let mei_data_clone = res.clone();

    let mei_data = CString::new(mei_data_clone).expect("fail");
    let mei_data_ptr = mei_data.as_ptr();
    std::mem::forget(mei_data);



    let svg_data_ptr = unsafe {vrvToolkit_renderData(vrv_ptr,mei_data_ptr,options_ptr)};
    let svg_data_cstr = unsafe {CStr::from_ptr(svg_data_ptr)};
    let svg_data_string = svg_data_cstr.to_str().unwrap();
    let out_string: String = svg_data_string.to_string().clone();
    std::mem::forget(svg_data_string);
    unsafe {vrvToolkit_destructor(vrv_ptr)};
    return out_string
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
