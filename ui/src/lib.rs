/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. 
 *
 * Copyright (c) 2019-2024 - Musmatic authors
 */
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod xmlwriter;
use xmlwriter::*;
use std::collections::HashMap;
use std::cell::RefCell;
#[macro_use]
extern crate serde_derive;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    //logging
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

enum PropValue{
    String(String),
    f32(f32)

}

thread_local!(static MEASURE_CLEFS: RefCell<HashMap<String,String>> = RefCell::new(HashMap::new()));
thread_local!(static MEASURE_PROPERTIES: RefCell<HashMap<String,HashMap<String,PropValue>>> = RefCell::new(HashMap::new()));
thread_local!(static MEASURE_KEY_SIGS: RefCell<HashMap<String,String>> = RefCell::new(HashMap::new()));

thread_local!(static CURRENT_NODE: RefCell<Option<&'static XMLNode>> = RefCell::new(None));

macro_rules! console_log {
    //log to js like using println!()
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn initialize(data: &str) {
    console_log!("initialized");
}

#[derive(Serialize,Deserialize)]
struct DocumentOptions {
    music_options: HashMap<String,String>,
    title: String,
    num_measures: i8,
    staff_grp: StaffGroup
}

#[derive(Serialize,Deserialize)]
struct StaffGroup {
    attributes: HashMap<String,String>,
    staves: Vec<HashMap<String,String>>
}

#[wasm_bindgen]
pub fn get_new_document_js(options: String) -> JsValue {
    console_log!("get new document");
    let mut attributes = HashMap::new();
    attributes.insert(String::from("xmlns"), String::from("http://www.music-encoding.org/ns/mei"));
    attributes.insert(String::from("xmlns:xlink"), String::from("http://www.w3.org/1999/xlink"));
    attributes.insert(String::from("meiversion"), String::from("4.0.0"));
    let mut mei = XMLNode {
        attr: attributes,
        name: String::from("mei"),
        ..Default::default()
    };
    let mut mei_head = XMLNode {
        name: String::from("meiHead"),
        ..Default::default()
    };
    let mut music = XMLNode {
        name: String::from("music"),
        ..Default::default()
    };
    let mut body = XMLNode {
        name: String::from("body"),
        ..Default::default()
    };
    let mut mdiv = XMLNode {
        name: String::from("mdiv"),
        ..Default::default()
    };
    let mut score = XMLNode {
        name: String::from("score"),
        ..Default::default()
    };

    let document_options: DocumentOptions = serde_json::from_str(&options).expect("Invalid document options");

    attributes = HashMap::new();
    let mut meter_count = "".to_string();
    let mut meter_unit = "".to_string();
    for (key,val) in document_options.music_options {
        attributes.insert(key.clone(),val.clone());
        if key=="meter.count" {
            meter_count = val.clone();
        }
        if key=="meter.unit" {
            meter_unit = val.clone();
        }
    }
    let mut meter: f32 = meter_count.parse::<f32>().unwrap() / meter_unit.parse::<f32>().unwrap();
    console_log!("meter: {}",meter);

    let mut file_desc = XMLNode {
        name: String::from("fileDesc"),
        ..Default::default()
    };
    let mut title_stmt = XMLNode {
        name: String::from("titleStmt"),
        ..Default::default()
    };
    let mut title = XMLNode {
        text: String::from(document_options.title),
        name: String::from("title"),
        ..Default::default()
    };

    title_stmt.children.push(title);
    file_desc.children.push(title_stmt);
    mei_head.children.push(file_desc);
    
    let mut score_def = XMLNode {
        attr: attributes,
        name: String::from("scoreDef"),
        ..Default::default()
    };

    let staff_grp_attributes = document_options.staff_grp.attributes;
    attributes = HashMap::new();
    for (key,val) in staff_grp_attributes {
        attributes.insert(key, val);
    }

    let mut staff_grp = XMLNode {
        attr: attributes,
        name: String::from("staffGrp"),
        ..Default::default()
    };

    let mut staves = document_options.staff_grp.staves;

    let mut counter: i8 = 1;

    let mut staff_names: Vec<String> = vec![];
    let mut staff_clefs: HashMap<String,String> = HashMap::new();
    for staff in staves {
        attributes = HashMap::new();
        for (attr,value) in staff{
            attributes.insert(attr,value);
        }
        attributes.insert(String::from("n"),format!("{}",counter));
        staff_clefs.insert(format!("{}",counter),attributes["clef.shape"].clone());
        staff_names.push(format!("{}",counter));
        let mut staff_n = XMLNode {
            attr: attributes,
            name: String::from("staffDef"),
            ..Default::default()
        };
        staff_grp.children.push(staff_n);

        counter += 1;
    }

     //initialize <section>
    let mut section = XMLNode {
        name: String::from("section"),
        ..Default::default()
    };

    for measure_number in 0..document_options.num_measures {
        attributes = HashMap::new();
        attributes.insert(String::from("n"), format!("{}",measure_number+1));
        attributes.insert(String::from("metcon"), String::from("false"));
        attributes.insert(String::from("xml:id"), format!("mes{}",measure_number+1));
        let mut measure = XMLNode {
            attr: attributes,
            name: String::from("measure"),
            ..Default::default()
        };
        for n in &staff_names{
            attributes = HashMap::new();
            attributes.insert(String::from("n"),format!("{}",n));
            let measure_id = format!("staff-{}-{}",measure_number+1,n);
            MEASURE_CLEFS.with(|m_cell|{
                let mut map = m_cell.borrow_mut();
                map.insert(measure_id.clone(),staff_clefs[&format!("{}",n)].clone());
            });
            MEASURE_PROPERTIES.with(|m_cell|{
                let mut map = m_cell.borrow_mut();
                let mut props = HashMap::new();
                props.insert("meter".to_string(),PropValue::f32(meter));
                props.insert("fill".to_string(),PropValue::f32(0.0));
                props.insert("clef".to_string(),PropValue::String(staff_clefs[&format!("{}",n)].clone()));
                props.insert("key_sig".to_string(),PropValue::String("".to_string()));
                map.insert(measure_id.clone(),props);
            });
            attributes.insert(String::from("xml:id"),measure_id);
            let mut staffx = XMLNode {
                attr: attributes,
                name: String::from("staff"),
                ..Default::default()
            };
            attributes = HashMap::new();
            attributes.insert(String::from("n"),String::from("1"));
                let mut layer = XMLNode {
                attr: attributes,
                name: String::from("layer"),
                ..Default::default()
            };
            staffx.children.push(layer);
            measure.children.push(staffx);
        }
        section.children.push(measure);
    }

    score_def.children.push(staff_grp);
    score.children.push(score_def);

    score.children.push(section);

    mdiv.children.push(score);

    body.children.push(mdiv);

    music.children.push(body);

    mei.children.push(mei_head);
    mei.children.push(music);

    //let json_doc = serde_json::to_string(&mei).expect("fail");

    let xml = xmlwriter::write_xml_from_struct(mei);

    //console_log!("{}",xml);
    JsValue::from_str(&xml)
}

#[wasm_bindgen]
pub fn create_note(note: String, accid: String, octave: String, measure: String, duration: String) -> JsValue{
    let mut clef: String;
    let mut key_sig: String;
    let dur: f32;
    let meter: f32;
    let fill: f32;
    match duration.parse::<f32>() {
        Ok(d) => dur = 1.0 / d,
        Err(_e) => return JsValue::from_str("Invalid duration"),
    }

    let mut measure_props: HashMap<String,PropValue> = HashMap::new();
    MEASURE_PROPERTIES.with(|cell|{
        let mut m_props = cell.borrow_mut();
        for (key, val) in &m_props[&measure]{
            let new_propvalue: PropValue;
            match val {
                PropValue::f32(n) => new_propvalue = PropValue::f32(*n),
                PropValue::String(n) => new_propvalue = PropValue::String(n.to_string()),
            }
            measure_props.insert(key.to_string(),new_propvalue);
        }
    });

    
    match &measure_props["clef"]{
        PropValue::f32(_n) => return JsValue::from_str("invalidclef"),
        PropValue::String(n) => clef = n.to_string(),
    }
    match &measure_props["meter"]{
        PropValue::f32(n) => meter = *n,
        PropValue::String(_n) => return JsValue::from_str("invalidmeter"),
    }
    match &measure_props["fill"]{
        PropValue::f32(n) => fill = *n,
        PropValue::String(_n) => return JsValue::from_str("invalidfill"),
    }
    let (fits,fill) = space_in_measure(dur,meter,fill);
    
    if !fits {
        return JsValue::from_str("nospace");
    }
    //update fill
    measure_props.insert("fill".to_string(),PropValue::f32(fill));


    let mut attributes = HashMap::new();
    let (note_pname,accid_ges) = get_note_pname(note);
    attributes.insert(String::from("pname"),note_pname);
    attributes.insert(String::from("oct"),octave);
    attributes.insert(String::from("dur"),duration);
    attributes.insert(String::from("stem.dir"),"up".to_string());
    let mut node = XMLNode{
        attr: attributes,
        name: "note".to_string(),
        ..Default::default()
    };
    MEASURE_PROPERTIES.with(|cell|{
        let mut m_props = cell.borrow_mut();
        m_props.insert(measure,measure_props);
    });
    let xml = xmlwriter::xml_from_node(node);
    JsValue::from_str(&xml)
}

fn space_in_measure(duration: f32, meter: f32, fill: f32) -> (bool,f32){
    let new_fill = fill + duration;
    if new_fill > meter {
        return (false,fill);
    }
    else{
        return (true,new_fill);
    }
}

//returns tuple containing the note pname and accid.ges
fn get_note_pname(note: String, key_sig: String)-> (String,String) {
    let mut pname;
    let mut accid_ges = "".to_string();
    match note.as_ref() {
        "0" => pname = "c".to_string(),
        "1" => pname = "c".to_string(),
        "2" => pname = "d".to_string(),
        "3" => pname = "d".to_string(),
        "4" => pname = "e".to_string(),
        "5" => pname = "f".to_string(),
        "6" => pname = "f".to_string(),
        "7" => pname = "g".to_string(),
        "8" => pname = "g".to_string(),
        "9" => pname = "a".to_string(),
        "10" => pname = "a".to_string(),
        "11" => pname = "b".to_string(),
        _ => pname = "".to_string()
    }
    (pname,accid_ges)
}

#[wasm_bindgen]
pub fn receive_from_js(val: &JsValue) {
    let event: String = val.into_serde().expect("fail");
    
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t0_get_note_pname() {
        let mut note = "0".to_string();
        let (note_pname,accid_ges) = get_note_pname(note);
        assert_eq!(("c".to_string(),"".to_string()),(note_pname,accid_ges));
    }

    #[test]
    fn t1_get_note_pname() {
        let mut note = "2".to_string();
        let (note_pname,accid_ges) = get_note_pname(note);
        assert_eq!(("d".to_string(),"".to_string()),(note_pname,accid_ges));
    }

    #[test]
    fn t0_space_in_measure(){
        let (fits,new_fill): (bool,f32) = space_in_measure(0.5,1.0,0.25);
        assert_eq!(fits,true);
        assert_eq!(new_fill,0.75);
    }

    #[test]
    fn t1_space_in_measure(){
        let (fits,new_fill): (bool,f32) = space_in_measure(0.5,1.0,0.75);
        assert_eq!(fits,false);
        assert_eq!(new_fill,0.75);
    }

}
