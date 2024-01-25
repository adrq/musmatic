/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. 
 *
 * Copyright (c) 2019-2024 - Musmatic authors
 */

const { initialize } = wasm_bindgen;
const {get_new_document_js} = wasm_bindgen;
const {receive_example_from_js} = wasm_bindgen;
const {create_note} = wasm_bindgen;

async function run() {
  await wasm_bindgen('./musmatic/musmatic_ui_bg.wasm');
  initialize();

  
}

class MeiDocument {
  constructor (meiXML){
    this.dom = $.parseXML(meiXML);
  }

  insert_note(note,octave,measure,duration){
    var layer = $(this.dom).find("staff[xml\\:id='"+measure+"'] layer");
    var note = create_note(note,"",octave,measure,duration);
    if (note==""){
      console.log("unable to create note");
      return note;
    }
    if (note=="nospace"){
      console.log("no space left in measure");
      return note;
    }
    $(layer).append(note);
    var mei = new XMLSerializer().serializeToString(this.dom);
    
    return mei;
  }
}

run();

var svg_zoom_instance;
var selected_measure = "none";
var selected_duration = "1";
var Document;
var old_svg_width;



//adjusts height of svg container when an orientation change is detected
$( window ).on( "orientationchange", function() {
  console.log("orientation change")
  setTimeout(function(){
  var starting_height = $("#score-container").position().top;
  var max_height = $("#input-controls").position().top -starting_height;
  $("#score-container").height(max_height);
  },200);
});

var setup_svg = function(svg){
  console.log("loading svg");
  $("#score-container").html(svg);
  var starting_height = $("#score-container").position().top;
  var max_height = $("#input-controls").position().top -starting_height;
  $("#score-container").height(max_height);
  $("#btn-zoom-in").click(function(){
    var cur_width = $("#score-container svg").width();
    old_svg_width = cur_width;
    $("#score-container svg").width(cur_width + 50);
    console.log("zoom in");
  });
  $("#btn-zoom-out").click(function(){
    var cur_width = $("#score-container svg").width();
    old_svg_width = cur_width;
    $("#score-container svg").width(cur_width - 50);
    console.log("zoom out");
  });

  //setup clickable rectangles for each measure > staff
  $("#score-container .measure .staff").each(function(){
    //parse d coords from top and bottom lines in meach staff
    var first_d  = $(this).children("path").first().attr("d");
    var regex = /[0-9]+/g;
    var matches = first_d.match(regex);
    var x = matches[0];
    var y = matches[1];
    var last_d = $(this).children("path").last().attr("d");
    var regex = /[0-9]+/g;
    var matches = last_d.match(regex);
    var width = matches[2]-x;
    var height = matches[3]-y;

    //create new transparent rect over staff
    d3.select($(this)[0]).append("rect")
      .attr("x", x)
      .attr("y", y)
      .attr("width", width)
      .attr("height", height)
      .attr("style", "fill:blue;stroke:red;stroke-width:15")
      .attr("fill-opacity",0)
      .attr("stroke-opacity",0);
    
    $(this).find("rect").click(function(){
      selected_measure = $(this).parent().attr("id");
      $("svg rect").removeClass("selected");
      $(this).addClass("selected");
      console.log(selected_measure)
    });
  });
}

$(document).ready(function(){
  $("#piano-container .octave").each(function(){
    $(this).load("svg/piano.svg",function(){
      $(this).children().children().each(function(){
        if ($(this).is("text")) {
          var octave = $(this).parent().parent().data("octave");
          $(this).text("C"+octave);
          return;
        }
        $(this).on("click",function(){
          var note = $(this).data("note");
          var octave = $(this).parent().parent().data("octave");
          if ($(this).hasClass("ivory")){
            $(this).css({fill:"ivory"});
          }
          else {
            $(this).css({fill:"black"});
          }
          if (selected_measure == "none"){
            alert("Please select a measure before entering notes")
          }
          else {
            var mei = Document.insert_note(note,parseInt(octave),selected_measure,selected_duration);
            switch(mei){
              case "":
                alert("Unknown error, unable to create note");
                break;
              case "nospace":
                alert("No space left in measure");
                break;
              default:
                get_svg_from_server(mei);
            }
          }
        });
        $(this).on("mousedown touchstart",function(){
          $(this).css({fill:"orange"});
        });
        $(this).on("mouseout touchmove",function(){
          if ($(this).hasClass("ivory")){
            $(this).css({fill:"ivory"});
          }
          else {
            $(this).css({fill:"black"});
          }
        });
      });
    });
  });

  $("#modal-container").load("modals.html",function(){
    $("#create-new-document").click(function(){
      var data = {};
      $("#new-document-form").serializeArray().map(function(x){data[x.name] = x.value;});
      var staff_grp = {
        attributes: {
          symbol: "line",
          //barthru: "true"
        },
        staves: [
          {
            "xml:id":"P1",
            "lines": "5",
            "clef.line": (data["clef"]=="G")?"2":"4",
            "clef.shape": data["clef"]
          }
        ]
      }
      var document_options = {
        music_options : {
          "meter.count":data["meter.count"],
          "meter.unit":data["meter.unit"]
        },
        num_measures: parseInt(data["num-measures"]),
        staff_grp : staff_grp,
        title: data["new-document-title"]
      }
      var mei = get_new_document_js(JSON.stringify(document_options));
      Document = new MeiDocument(mei);
      get_svg_from_server(mei);
      $("#btn-zoom-out").click();
      $("#btn-zoom-out").click();      
    });
  });

  //setup note entry buttons
  $("#note-entry-tab button").click(function (){
    $("#note-entry-tab button").removeClass("active");
    $(this).addClass("active");
    selected_duration = $(this).data("duration");
  });
});

function get_svg_from_server(mei){
  $("#mei-tab-pre").text(mei);
  var req_data = {"data":mei};
  $.ajax({
    type: "POST",
    crossDomain: true,
    url: api_host+"/getsvg",
    data: JSON.stringify(req_data),
    contentType: "application/json",
    dataType: "json",
    success: setup_svg,
    failure: function(errMsg) {
        alert(errMsg);
    }
  });
}

// Changes XML to JSON
// Modified version from here: http://davidwalsh.name/convert-xml-json
function xmlToJson(xml) {

	// Create the return object
	var obj = {};

	if (xml.nodeType == 1) { // element
		// do attributes
		if (xml.attributes.length > 0) {
		obj["@attributes"] = {};
			for (var j = 0; j < xml.attributes.length; j++) {
				var attribute = xml.attributes.item(j);
				obj["@attributes"][attribute.nodeName] = attribute.nodeValue;
			}
		}
	} else if (xml.nodeType == 3) { // text
		obj = xml.nodeValue;
	}

	// do children
	// If just one text node inside
	if (xml.hasChildNodes() && xml.childNodes.length === 1 && xml.childNodes[0].nodeType === 3) {
		obj = xml.childNodes[0].nodeValue;
	}
	else if (xml.hasChildNodes()) {
		for(var i = 0; i < xml.childNodes.length; i++) {
			var item = xml.childNodes.item(i);
			var nodeName = item.nodeName;
			if (typeof(obj[nodeName]) == "undefined") {
				obj[nodeName] = xmlToJson(item);
			} else {
				if (typeof(obj[nodeName].push) == "undefined") {
					var old = obj[nodeName];
					obj[nodeName] = [];
					obj[nodeName].push(old);
				}
				obj[nodeName].push(xmlToJson(item));
			}
		}
	}
	return obj;
}
