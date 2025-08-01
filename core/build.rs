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

extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
fn main() {
    let verovio_build_dir = "vendor/verovio/cmake";
    let verovio_base_dir = "vendor/verovio/tools";
    //run bash script to get/update Verovio sources if needed
    Command::new("bash")
        .arg("get-dep.sh")
        .status()
        .expect("Unable to update verovio sources");

    

    //Run cmake and then make to build Verovio
    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd
        .current_dir(verovio_build_dir)
        .arg("-DBUILD_AS_LIBRARY=ON")
        .arg(".")
        .status()
        .expect("Error executing cmake");

    let mut make_cmd = Command::new("make");
    let num_jobs = "-j".to_string() + &env::var("NUM_JOBS").unwrap();
make_cmd
        .current_dir(verovio_build_dir)
        .arg(num_jobs) //enable multithreaded build
        .status()
        .expect("Error executing make");
 

/*    Command::new("make")
        .arg(num_jobs)
        .status()
        .expect("Error executing make");
*/
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join(verovio_build_dir).display()
    );
    println!("cargo:rustc-link-lib=static=verovio");

    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else {
        unimplemented!();
    }

    //Edit c_wrapper.h to add necessary typedef and header
    //sed -i '1s/^/typedef void Toolkit;\n/' c_wrapper.h
    let mut sed_cmd = Command::new("bash");
    sed_cmd
        .arg("fix-c-wrapper.sh")
        .status()
        .expect("Error executing sed");
    

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(verovio_base_dir.to_string()+&"/c_wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("verovio_bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut cp_cmd = Command::new("cp");
    cp_cmd
        .arg(out_path.join("verovio_bindings.rs"))
        .arg("src/verovio_bindings.rs")
        .status()
        .expect("Error copying verovio bindings");

}
