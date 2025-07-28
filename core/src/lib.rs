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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

include!("verovio_bindings.rs");

#[derive(Debug, Serialize, Deserialize)]
struct VerovioOptions {
    options: HashMap<String,String>
}

fn new_instance(resource_path: String) -> (*mut std::ffi::c_void, String) {
    let res_path = CString::new(resource_path).expect("Unable to create CString");
    let vrv_ptr = unsafe {vrvToolkit_constructorResourcePath(res_path.as_ptr())};
    let vrv_options = unsafe { CStr::from_ptr(vrvToolkit_getAvailableOptions(vrv_ptr)).to_str().expect("fail")};
    (vrv_ptr,vrv_options.to_string())
}

/// Renders MEI (Music Encoding Initiative) data to SVG using Verovio
///
/// # Arguments
///
/// * `mei_data` - A string containing the MEI XML data to be rendered
///
/// # Returns
///
/// A string containing the rendered SVG data
pub fn render_mei_to_svg(mei_data: String) -> String {
    let mut res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_path.push("vendor/verovio/data");

    let (vrv_ptr, _vrv_options) = new_instance(res_path.to_str().unwrap().to_string());

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
    let mei_data_clone = mei_data.clone();

    let mei_data = CString::new(mei_data_clone).expect("fail");
    let mei_data_ptr = mei_data.as_ptr();
    std::mem::forget(mei_data);

    let svg_data_ptr = unsafe {vrvToolkit_renderData(vrv_ptr,mei_data_ptr,options_ptr)};
    let svg_data_cstr = unsafe {CStr::from_ptr(svg_data_ptr)};
    let svg_data_string = svg_data_cstr.to_str().unwrap();
    let out_string: String = svg_data_string.to_string().clone();
    std::mem::forget(svg_data_string);
    unsafe {vrvToolkit_destructor(vrv_ptr)};
    out_string
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn constructor_works() {
        let mut res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        res_path.push("vendor/verovio/data");
        let (vrv_ptr, vrv_options) = new_instance(res_path.to_str().unwrap().to_string());
        assert_eq!(vrv_options.is_empty(), false);
        unsafe {vrvToolkit_destructor(vrv_ptr)};
    }

    #[test]
    fn render_simple_mei_to_svg() {
        // Simple MEI document with a single whole note C4
        let mei_input = r#"<?xml version="1.0" encoding="UTF-8"?>
<mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.0">
    <music>
        <body>
            <mdiv>
                <score>
                    <scoreDef>
                        <staffGrp>
                            <staffDef n="1" lines="5" clef.shape="G" clef.line="2"/>
                        </staffGrp>
                    </scoreDef>
                    <section>
                        <measure n="1">
                            <staff n="1">
                                <layer n="1">
                                    <note dur="1" oct="4" pname="c"/>
                                </layer>
                            </staff>
                        </measure>
                    </section>
                </score>
            </mdiv>
        </body>
    </music>
</mei>"#;

        let svg_output = render_mei_to_svg(mei_input.to_string());

        // Basic validation that we got SVG output
        assert!(svg_output.contains("<svg"), "Output should contain SVG tag");
        assert!(svg_output.contains("</svg>"), "Output should be well-formed SVG");
        assert!(svg_output.contains("viewBox"), "Output should have viewBox attribute");

        // Check that it contains musical notation elements
        assert!(svg_output.contains("class=\"note"), "Output should contain note elements");
        assert!(svg_output.contains("class=\"staff"), "Output should contain staff elements");

        // Ensure output is not empty or error message
        assert!(svg_output.len() > 1000, "SVG output should be substantial (>1000 chars)");
        assert!(!svg_output.contains("error"), "Output should not contain error messages");
    }
}
