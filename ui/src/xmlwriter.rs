/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. 
 *
 * Copyright (c) 2019-2023 - Musmatic authors
 */

use std::collections::HashMap;


#[derive(Serialize,Deserialize,Debug,Default,Clone)]
pub struct XMLNode {
  pub attr: HashMap<String, String>,
  pub name: String,
  pub text: String,
  pub children: Vec<XMLNode>,
}

pub fn write_xml_from_struct(document: XMLNode) -> String{
  let string = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\
    <?xml-model href=\"http://music-encoding.org/schema/4.0.0/mei-all.rng\" type=\"application/xml\" schematypens=\"http://relaxng.org/ns/structure/1.0\"?>\
    <?xml-model href=\"http://music-encoding.org/schema/4.0.0/mei-all.rng\" type=\"application/xml\" schematypens=\"http://purl.oclc.org/dsdl/schematron\"?>";
  format!("{}{}",string,xml_from_node(document))
}

pub fn xml_from_node(node: XMLNode) -> String{
  let mut xml_string: String = format!("<{}",node.name);
  for (attribute,value) in node.attr {
    xml_string = format!("{} {}=\"{}\"",xml_string,attribute,value);
  }
  xml_string = format!("{}>",xml_string);

  xml_string = format!("{}{}",xml_string,node.text);
  
  for child in node.children{
    xml_string = format!("{}{}",xml_string,xml_from_node(child));
  }
  xml_string = format!("{}</{}>",xml_string,node.name);
  format!("{}",xml_string)//return static slice
}
