/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. 
 * 
 * Copyright (c) 2019 - Musmatic authors
 */
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use std::path::PathBuf;


include!(concat!(env!("OUT_DIR"), "/verovio_bindings.rs"));


fn new_instance(resource_path: String) -> (*mut std::ffi::c_void, String) {
    let res_path = CString::new(resource_path).expect("Unable to create CString");
    let vrv_ptr = unsafe {vrvToolkit_constructorResourcePath(res_path.as_ptr())};
    let vrv_options = unsafe { CStr::from_ptr(vrvToolkit_getAvailableOptions(vrv_ptr)).to_str().expect("fail")};
    //println!("{}",vrv_options);
    (vrv_ptr,vrv_options.to_string())
}

extern crate actix_web;

use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, http::header, middleware::cors::Cors
};

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    data: String,
}

fn main(){
    println!("hi");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::new()
                    //.allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            /*.wrap(middleware::DefaultHeaders::new()
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods","GET, POST, PATCH, PUT, DELETE, OPTIONS")
                    .header("Access-Control-Allow-Headers","Origin, Content-Type, X-Auth-Token")
                )*/
            .service(web::resource("/getsvg").route(web::post().to(getsvg)))
            
    })
    .bind("127.0.0.1:8080")
    .unwrap()
.run();
}


fn getsvg(req: web::Json<Request>) -> HttpResponse {
    //println!("model: {:?}", &req);
    println!("getsvg");

    let mut res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_path.push("vendor/verovio/data");
    let vrv_ptr: *mut std::ffi::c_void;
    let mut vrv_options = String::new();
    let (vrv_ptr, vrv_options) = new_instance(res_path.to_str().unwrap().to_string());
    let options = CString::new("{}").expect("render_mei_data unable to create CString");
    let mei_data_clone = req.data.clone();
    //println!("mei data clone{}",mei_data_clone);
    let mei_data = CString::new(mei_data_clone).expect("fail");
    let mei_data_ptr = mei_data.as_ptr();
    std::mem::forget(mei_data);
    //println!("mei data ptr{:?}",mei_data_ptr);
      
    //unsafe {println!("meidata{}",CStr::from_ptr(mei_data_ptr).to_str().unwrap())};
    let svg_data_ptr = unsafe {vrvToolkit_renderData(vrv_ptr,mei_data_ptr,options.as_ptr())};
    //println!("svg data ptr{:?}",svg_data_ptr);
    let svg_data_cstr = unsafe {CStr::from_ptr(svg_data_ptr)};
    //println!("svg data str{}",svg_data_cstr.to_str().unwrap());
    let svg_data_string = svg_data_cstr.to_str().unwrap();
    let out_string: String = svg_data_string.to_string().clone();
    std::mem::forget(svg_data_string);
    //println!("svg data string{}",svg_data_string);
    unsafe {vrvToolkit_destructor(vrv_ptr)};
    HttpResponse::Ok().json(out_string) // <- send response
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
