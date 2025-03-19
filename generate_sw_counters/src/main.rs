use std::io::prelude::*;
use std::path::Path;
use std::{error::Error, io, process};
// This lets us write `#[derive(Deserialize)]`.
use serde::Deserialize;
//
// Command line argument processing.
//
use clap::Parser;
//
// Local files.
//
use common_functions::*;
use common_functions::arguments::*;
use common_functions::colors::*;
use common_functions::debugging::*;
use common_functions::defines::*;
use common_functions::overrides::*;
use common_functions::text_field::*;
use common_functions::utils::*;

pub const LEADERSHIP_NA: &str =	"&#x25B3;";

pub const STRENGTH_FACTORS_X_POSITION: f64 =	 3.0;
pub const STRENGTH_FACTORS_Y_POSITION: f64 =	32.0;
pub const STRENGTH_FACTORS_Y_BASELINE: f64 =	11.0;
pub const STRENGTH_FACTORS_HEIGHT: f64 =		16.0;
pub const STRENGTH_FACTORS_FONT_SIZE: f64 =		12.0;
pub const STRENGTH_FACTORS_SUP_FONT_SIZE: f64 =	 8.0;

pub const CLASS_X_POSITION: f64 =	 	35.0;
pub const CLASS_Y_POSITION: f64 =		 2.0;
pub const CLASS_SIZE: f64 =				11.0;
pub const CLASS_FONT_SIZE: f64 =		 9.0;
pub const CLASS_STROKE_WIDTH: f64 =		 0.75;
pub const CLASS_CIRCLE_RADIUS: f64 =	 4.5;

pub const AE_X_POSITION: f64 =	 	 2.0;
pub const AE_Y_POSITION: f64 =		 2.0;
pub const AE_HEIGHT: f64 =			15.0;
pub const AE_WIDTH: f64 =			10.0;

pub const COL_X_POSITION: f64 =	 	 2.0;
pub const COL_Y_POSITION: f64 =		12.0;
pub const COL_HEIGHT: f64 =			12.0;
pub const COL_WIDTH: f64 =			 8.0;

pub const ROF_SIZE: f64 =			12.0;
pub const ROF_STROKE_WIDTH: f64 =	 0.75;
//
// Sanitized and parsed aircraft-specific record fields.
//
#[derive(Default)]
struct Record {
	args: Arguments,
	nationality: String,
	counter: String,
	value: String,
	range: String,
	spraying_fire: bool,
	rof: String,
	pp: usize,
	pp_x: f64,
	pp_y: f64,
	breakdown: String,
	low_ammo: bool,
	repair: usize,
	disable: usize,
	repair_text: String,
	disable_text: String,
	leadership_na: bool,
	leadership_x: f64,
	leadership_y: f64,
	pieces: Vec<std::string::String>,	// Possibly multiple pieces per entry.
	piece: String,						// Each individual piece from "pieces" above.
	version: String,
	overrides: Overrides,
	colors: Colors,
	svg_image_transform: String,
	comments: String,
}

impl Record {
	fn sanitize_range(&mut self, range: &str) {
		self.range = range.to_string();
		
		if self.range.contains(FIVE_LOBED_ASTERISK_TAG) {
			self.range = convert_text(&self.range, FIVE_LOBED_ASTERISK_TAG, &FIVE_LOBED_ASTERISK_UC.to_string());
		} 
	}
	
	fn sanitize_rof(&mut self, rof: &str) {
		self.rof = rof.to_string();
		
		if self.rof.contains(FIVE_LOBED_ASTERISK_TAG) {
			self.rof = convert_text(&self.rof, FIVE_LOBED_ASTERISK_TAG, &FIVE_LOBED_ASTERISK_UC.to_string());
		}
	}

	fn sanitize_pp(&mut self, pp: &str) {
		if !pp.contains(MOD_DELIMITER2) {
			self.pp = pp.parse::<usize>().unwrap_or(0);
			self.pp_x = 30.0;
			self.pp_y = 24.0;
		} else {
			let mut entries: Vec<std::string::String> = extract_vector(&pp.to_string(), MOD_DELIMITER2);
			
			self.pp = entries[0].parse::<usize>().unwrap_or(0);
			
			entries = extract_vector(&entries[1], MOD_DELIMITER1);
			self.pp_x = entries[0].parse::<f64>().unwrap_or(0.0);
			self.pp_y = entries[1].parse::<f64>().unwrap_or(0.0);
		}
	}

	fn sanitize_breakdown(&mut self, breakdown: &str) {
		self.breakdown = breakdown.to_string();
		
		if self.breakdown.contains('(') {
			self.low_ammo = true;
			self.breakdown = strip_all_occurances(&self.breakdown, '(');
			self.breakdown = strip_all_occurances(&self.breakdown, ')');
		}
	}
	
	fn sanitize_repair(&mut self, repair: &str) {
		if !repair.is_empty() {
			if repair.contains("/") {
				let (left, right) = repair.split_once('/').unwrap();
			
				self.repair = left.parse::<usize>().unwrap_or(0);
				self.disable = right.parse::<usize>().unwrap_or(0);
				
				if 0 == self.repair && 0 == self.disable {
					self.repair_text = left.to_string();
					self.disable_text = right.to_string();
				}
			} else {
				self.repair = repair.parse::<usize>().unwrap_or(0);
				self.disable = self.repair;
			}
		}
	}
	
	fn sanitize_leadership(&mut self, leadership: &str) {
		self.leadership_na = !leadership.is_empty();
		self.leadership_x = 39.0;
		self.leadership_y = 22.0;

		if leadership.contains(MOD_DELIMITER2) {
			let mut entries: Vec<std::string::String> = extract_vector(&leadership.to_string(), MOD_DELIMITER2);
			
			entries = extract_vector(&entries[1], MOD_DELIMITER1);
			self.leadership_x = entries[0].parse::<f64>().unwrap_or(0.0);
			self.leadership_y = entries[1].parse::<f64>().unwrap_or(0.0);
		}
	}	
	
	fn generate_unit_depiction_svg_elements(&mut self, mut output: &std::fs::File, root_path: &String) {
		if INCLUDE_IMAGES {
			let path_prefix = "svg/";
			let file_type_svg = ".svg";
			let file_type_png = ".png";
	
			let paths: Vec<std::string::String> = [
				format!("{}{}{}", path_prefix, &self.piece, file_type_svg),
				format!("{}{}{}", path_prefix, &self.piece, file_type_png)
			].to_vec();
	
			for mut path in paths {
				let mut pathname: String = root_path.to_string();
				pathname.push_str(&path.to_string());
	
				if Path::new(&pathname).exists() {
					let mut transform: String = "scale(1.00)".to_string();
	
					if !self.svg_image_transform.is_empty() {
						if self.svg_image_transform.contains("scale") {
							transform = self.svg_image_transform.to_string();
						} else {
							transform = format!("{transform} {0}", self.svg_image_transform);
						}
					}
	
					if path.contains(SPACE) {
						path = path.replace(SPACE, "%20");
					}
	
					write!(output, "\t<!-- Unit Depiction -->\n").unwrap();
					write!(output, "\t<image x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" transform=\"{transform}\" href=\"{path}\" xlink:href=\"{path}\"/>\n").unwrap();
					break;	// Our work here is done.
				}
			}
		}
	}

	fn generate_counter_svg_elements(&mut self, mut output: &std::fs::File) {
		let mut size: usize = 11;
		
		if 7 < self.counter.len() {
			size = 8;
		} else if 5 < self.counter.len(){
			size = 10;
		}
		
		write!(output, "\t<text x=\"44.5\" y=\"11\" style=\"font-size:{size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.counter).unwrap();
		
		if self.counter.contains("MTR") {
			self.generate_mortar_svg_elements(output);
		} else if self.counter.contains("INF") {
			self.generate_inf_svg_elements(output);			
		} else if self.counter.contains("RCL") {
			self.generate_rcl_svg_elements(output);
		} else {
			self.generate_strength_svg_elements(output);
		}
		
		self.generate_pp_svg_elements(output);
		self.generate_extra_info_svg_elements(output);
		self.generate_breakdown_svg_elements(output);
		self.generate_leadership_svg_elements(output);
		self.generate_rof_svg_elements(output);
	}

	fn generate_sfcp_type_svg_elements(&mut self, mut output: &std::fs::File) {
		let size: usize = 9;
		
		write!(output, "\t<text x=\"44.5\" y=\"10\" style=\"font-size:{size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.counter).unwrap();
		write!(output, "\t<text x=\"44.5\" y=\"18\" style=\"font-size:{size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">Radio</text>\n", self.colors.text).unwrap();
		
		self.generate_strength_svg_elements(output);
		self.generate_pp_svg_elements(output);
		self.generate_extra_info_svg_elements(output);
		self.generate_breakdown_svg_elements(output);
		self.generate_leadership_svg_elements(output);
		self.generate_rof_svg_elements(output);
	}

	fn generate_baz_back_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		//
		// Common elements, "Range" and "TH#".
		//
		write!(output, "\t<text transform=\"translate(11,24) rotate(-90)\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Range</text>\n").unwrap();
		write!(output, "\t<text transform=\"translate(32,24) rotate(-90)\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">TH#</text>\n").unwrap();
		//
		// '43 & '44 Bazookas all have 4 hex range, everything else has 5 hex range.
		//
		if self.piece.contains("43") || self.piece.contains("44") {
			write!(output, "\t<text x=\"17\" y=\"12\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">0</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"19\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">1</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"26\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">2</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"33\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">3</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"40\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">4</text>\n").unwrap();			
		} else {
			write!(output, "\t<text x=\"17\" y=\"10\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">0</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"17\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">1</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"24\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">2</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"31\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">3</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"38\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">4</text>\n").unwrap();
			write!(output, "\t<text x=\"17\" y=\"45\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">5</text>\n").unwrap();			
		}
		//
		// Now the variations on the TH#s gets kinda wonky.
		//
		if self.piece.contains("43") || self.piece.contains("44") {
			let mut th_0 = 11;
			let mut th_1 =  9;
			let mut th_2 =  8;
			let mut th_3 =  7;
			let mut th_4 =  4;
			
			if self.piece.contains("cc") {
				th_0 = 10;
				th_1 =  8;
				th_2 =  7;
				th_3 =  6;
				th_4 =  3;
			}

			write!(output, "\t<text x=\"38\" y=\"12\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_0}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"19\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_1}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"26\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_2}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"33\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_3}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"40\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_4}</text>\n").unwrap();				
		} else {
			let mut th_0 = 11;
			let mut th_1 = 10;
			let mut th_2 =  9;
			let mut th_3 =  8;
			let mut th_4 =  6;
			let mut th_5 =  4;
			
			if self.piece.contains("cc") && self.piece.contains("50") || self.piece.contains("51") {
				th_0 = 10;
				th_1 =  9;
				th_2 =  8;
				th_3 =  7;
				th_4 =  5;
				th_5 =  3;
			}
			
			write!(output, "\t<text x=\"38\" y=\"10\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_0}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"17\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_1}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"24\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_2}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"31\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_3}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"38\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_4}</text>\n").unwrap();
			write!(output, "\t<text x=\"38\" y=\"45\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">{th_5}</text>\n").unwrap();			
		}
	}	

	fn generate_dc_back_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		
		write!(output, "\t<text x=\"50%\" y=\"10.00\" style=\"font-size:7px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Place MPh</text>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"17.00\" style=\"font-size:7px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Attack AFPh</text>\n").unwrap();
		write!(output, "\t<line x1=\"3\" y1=\"19\" x2=\"45\" y2=\"19\" style=\"stroke:{color};stroke-width:1.00\"/>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"26.00\" style=\"font-size:7px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Thrown</text>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"33.00\" style=\"font-size:7px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">+2/+3</text>\n").unwrap();
		write!(output, "\t<line x1=\"3\" y1=\"35\" x2=\"45\" y2=\"35\" style=\"stroke:{color};stroke-width:1.00\"/>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"42.00\" style=\"font-size:7px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Set: 36 FP</text>\n").unwrap();		
	}
	
	fn generate_ft_back_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		
		write!(output, "\t<text x=\"50%\" y=\"12.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">No AFPh</text>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"20.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">penalty</text>\n").unwrap();
		write!(output, "\t<line x1=\"3\" y1=\"24\" x2=\"45\" y2=\"24\" style=\"stroke:{color};stroke-width:1.00\"/>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"34.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">-1 DRM</text>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"42.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">vs Owner</text>\n").unwrap();			
	}

	fn generate_pf_back_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		
		write!(output, "\t<text transform=\"translate(11,24) rotate(-90)\" style=\"font-size:9.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\"\">Range</text>\n").unwrap();
		write!(output, "\t<text x=\"17.00\" y=\"12.00\" style=\"font-size:8.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}L\">0</text>\n").unwrap();
		write!(output, "\t<text x=\"17.00\" y=\"22.50\" style=\"font-size:8.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}L\">1</text>\n").unwrap();
		write!(output, "\t<text x=\"17.00\" y=\"32.50\" style=\"font-size:8.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}L\">2</text>\n").unwrap();
		write!(output, "\t<text x=\"17.00\" y=\"42.00\" style=\"font-size:8.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}L\">3</text>\n").unwrap();
		write!(output, "\t<text transform=\"translate(32,24) rotate(-90)\" style=\"font-size:9.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">TH#</text>\n").unwrap();
		write!(output, "\t<text x=\"38.00\" y=\"12.00\" style=\"font-size:8.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">10</text>\n").unwrap();
		write!(output, "\t<text x=\"38.00\" y=\"22.50\" style=\"font-size:8.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">8</text>\n").unwrap();
		write!(output, "\t<text x=\"38.00\" y=\"32.50\" style=\"font-size:8.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">6</text>\n").unwrap();
		write!(output, "\t<text x=\"38.00\" y=\"42.00\" style=\"font-size:8.00px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">4</text>\n").unwrap();
	}

	fn generate_pfk_back_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;

		write!(output, "\t<text transform=\"translate(11,24) rotate(-90)\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Range</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"22.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">0</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"32.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">1</text>\n").unwrap();
		write!(output, "\t<text transform=\"translate(32,24) rotate(-90)\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">TH#</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"22.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">10</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"32.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">8</text>\n").unwrap();		
	}
	
	fn generate_phone_back_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		
		write!(output, "\t<text x=\"50%\" y=\"12.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">No Move</text>\n").unwrap();
		write!(output, "\t<line x1=\"3\" y1=\"16\" x2=\"45\" y2=\"16\" style=\"stroke:{color};stroke-width:1.00\"/>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"25.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Security</text>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"35.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Area 2 DR</text>\n").unwrap();
		write!(output, "\t<text x=\"50%\" y=\"44.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Removes</text>\n").unwrap();	
	}

	fn generate_piat_back_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		let x_color = &self.colors.malfunction_x;

		write!(output, "\t<line x1=\"5\" y1=\"5\" x2=\"43\" y2=\"43\" style=\"stroke:{x_color}; stroke-width:3.00\"/>\n").unwrap();
		write!(output, "\t<line x1=\"5\" y1=\"43\" x2=\"43\" y2=\"5\" style=\"stroke:{x_color}; stroke-width:3.00\"/>\n").unwrap();
		write!(output, "\t<text x=\"3\" y=\"10\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:start;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">R2</text>\n").unwrap();
		write!(output, "\t<text x=\"45\" y=\"10\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">X6</text>\n").unwrap();
		write!(output, "\t<text transform=\"translate(11,28) rotate(-90)\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Range</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"19.0\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">0</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"26.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">1</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"34.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">2</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"42.0\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">3</text>\n").unwrap();
		write!(output, "\t<text transform=\"translate(32,28) rotate(-90)\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">TH#</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"19.0\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">11</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"26.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">9</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"34.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">7</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"42.0\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">5</text>\n").unwrap();		
	}

	fn generate_psk_back_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;

		write!(output, "\t<text transform=\"translate(11,24) rotate(-90)\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">Range</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"12\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">0</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"19\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">1</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"26\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">2</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"33\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">3</text>\n").unwrap();
		write!(output, "\t<text x=\"17\" y=\"40\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">4</text>\n").unwrap();
		write!(output, "\t<text transform=\"translate(32,24) rotate(-90)\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">TH#</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"12\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">11</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"19\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">9</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"26\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">8</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"33\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">7</text>\n").unwrap();
		write!(output, "\t<text x=\"38\" y=\"40\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">4</text>\n").unwrap();		
	}
	
	fn generate_counter_malfunction_x_svg_elements(&mut self, mut output: &std::fs::File) {
		write!(output, "\t<line x1=\"5\" y1=\"5\" x2=\"43\" y2=\"43\" style=\"stroke:{0}; stroke-width:3.00\"/>\n", self.colors.malfunction_x).unwrap();
		write!(output, "\t<line x1=\"5\" y1=\"43\" x2=\"43\" y2=\"5\" style=\"stroke:{0}; stroke-width:3.00\"/>\n", self.colors.malfunction_x).unwrap();
	}
	
	fn generate_counter_malfunction_svg_elements(&mut self, mut output: &std::fs::File) {
		self.generate_counter_malfunction_x_svg_elements(output);
		
		if 0 != self.repair {
			write!(output, "\t<text x=\"2\" y=\"11\" style=\"font-size:11px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:start;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">R{1}</text>\n", self.colors.text, self.repair).unwrap();
		}
		
		if 0 != self.disable {
			write!(output, "\t<text x=\"45\" y=\"45\" style=\"font-size:11px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">X{1}</text>\n", self.colors.text, self.disable).unwrap();
		}
		
		if !self.repair_text.is_empty() {
			write!(output, "\t<text x=\"24\" y=\"11\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.repair_text).unwrap();
		}
		
		if !self.disable_text.is_empty() {
			write!(output, "\t<text x=\"24\" y=\"43\" style=\"font-size:9px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.disable_text).unwrap();
		}		
	}
	
	fn generate_strength_svg_elements(&mut self, mut output: &std::fs::File) {
		if !self.value.is_empty() {
			let mut values = self.value.to_string();
			let size = if 2 < values.len() { 9 } else { 11 };
			
			if !self.range.is_empty() {
				if self.spraying_fire {
					values = format!("{0}-<tspan style=\"text-decoration:underline\">{1}</tspan>", values, self.range);
				} else {
					values = format!("{0}-{1}", values, self.range);
				}
			}
		
			write!(output, "\t<text x=\"44.5\" y=\"42.0\" style=\"font-size:{size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{values}</text>\n", self.colors.text).unwrap();
		}
	}

	fn generate_mortar_svg_elements(&mut self, mut output: &std::fs::File) {
		let mut value = self.value.to_string();
		
		if value.contains("&#x273D;") {
			value = convert_text(&value, "&#x273D;", "<tspan style=\"font-size:5\" baseline-shift=\"super\">&#x273D;</tspan>");
		}
		
		if self.counter.contains("dmMTR") {
			write!(output, "\t<text x=\"45.0\" y=\"43.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, value).unwrap();
		} else {
			write!(output, "\t<text x=\"2.0\" y=\"43.5\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:start;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, value).unwrap();
		}
		
		if !self.range.is_empty() {
			write!(output, "\t<text x=\"46\" y=\"43.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.range).unwrap();
		}
	}
	
	fn generate_inf_svg_elements(&mut self, mut output: &std::fs::File) {
		let mut value = self.value.to_string();
		
		if value.contains("&#x273D;") {
			value = convert_text(&value, "&#x273D;", "<tspan style=\"font-size:5\" baseline-shift=\"super\">&#x273D;</tspan>");
		}
		
		if self.counter.contains("dmINF") {
			write!(output, "\t<text x=\"46.0\" y=\"43.5\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, value).unwrap();
		} else {
			write!(output, "\t<text x=\"2.0\" y=\"43.5\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:start;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, value).unwrap();
		}
		write!(output, "\t<text x=\"46\" y=\"43.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.range).unwrap();
	}
	
	fn generate_rcl_svg_elements(&mut self, mut output: &std::fs::File) {
		let mut value = self.value.to_string();
		
		if value.contains("&#x273D;") {
			value = convert_text(&value, "&#x273D;", "<tspan style=\"font-size:5\" baseline-shift=\"super\">&#x273D;</tspan>");
		}
		
		write!(output, "\t<text x=\"2.0\" y=\"43.5\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:start;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, value).unwrap();
		write!(output, "\t<text x=\"46\" y=\"43.5\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.range).unwrap();
	}	
	
	fn generate_pp_svg_elements(&mut self, mut output: &std::fs::File) {
		if 0 != self.pp {
			write!(output, "\t<text x=\"{1:.2}\" y=\"{2:.2}\" style=\"font-size:8px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\" transform=\"rotate(-90,{1},{2})\">{3}PP</text>\n", self.colors.text, self.pp_x, self.pp_y, self.pp).unwrap();
		}
	}
	
	fn generate_extra_info_svg_elements(&mut self, mut output: &std::fs::File) {
		if !self.overrides.extra_info.is_empty() {
			let entries: Vec<std::string::String> = extract_vector(&self.overrides.extra_info, MOD_DELIMITER3);
			
			for entry in entries {
				let mut extra_info: String = entry.to_string();
				let mut extra_info_anchor = "end".to_string();
				let mut extra_info_x = 44.5;
				let mut extra_info_y = 26.0;
				let mut extra_info_rotate = false;
				let mut extra_info_degrees = 0.0;
				let mut extra_info_font_size = 7.0;
				//
				// Format is extra=<XXX>[!<XXX>]*
				//
				// Where <XXX> is <AAA>[@<BBB>]* or <AAA>@<BBB>[@<CCC>]
				//
				// Where <AAA> is the text to display, <BBB> is the location to display it at (with 'S'/'M'/'E' prefix for text alignment), and <CCC> is either "@sz-<XX>" or "@sz+<XX>" to modify the font size.
				//
				if entry.contains(MOD_DELIMITER2) {
					let sub_entries: Vec<std::string::String> = extract_vector(&entry.to_string(), MOD_DELIMITER2);
					
					extra_info = sub_entries[0].to_string();
					
					let mut length = sub_entries.len();
					
					if 3 == length {
						if sub_entries[2].contains(MOD_INC_SIZE) {
							let temp: String = extract_from(&sub_entries[2], MOD_INC_SIZE);
							
							extra_info_font_size += temp.parse::<f64>().unwrap_or(0.0);
						} else if sub_entries[2].contains(MOD_DEC_SIZE) {
							let temp: String = extract_from(&sub_entries[2], MOD_DEC_SIZE);
							
							extra_info_font_size -= temp.parse::<f64>().unwrap_or(0.0);
						}
					}
					
					let sub_sub_entries = extract_vector(&sub_entries[1], MOD_DELIMITER1);
					
					if "S" == sub_sub_entries[0] {
						extra_info_anchor = "start".to_string();
					} else if "M" == sub_sub_entries[0] {
						extra_info_anchor = "middle".to_string();
					} else if "E" == sub_sub_entries[0] {
						extra_info_anchor = "end".to_string();
					}
					
					length = sub_sub_entries.len();
					
					if 2 <= length {
						extra_info_x = sub_sub_entries[1].parse::<f64>().unwrap_or(0.0);
					}
					
					if 3 <= length {
						extra_info_y = sub_sub_entries[2].parse::<f64>().unwrap_or(0.0);
					}
					
					if 4 <= length {
						extra_info_rotate = true;
						extra_info_degrees = sub_sub_entries[3].parse::<f64>().unwrap_or(0.0);
						extra_info_anchor = "middle".to_string();
					}
				}
				
				if extra_info.contains("<sup>") {
					extra_info = convert_superscripts(&extra_info, 5.0);
				}
				
				if extra_info_rotate {
					write!(output, "\t<text x=\"{3:.2}\" y=\"{4:.2}\" style=\"font-size:{5:.2}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:{2};fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\" transform=\"rotate({6:.2},{3:.2},{4:.2})\">{1}</text>\n", self.colors.text, extra_info, extra_info_anchor, extra_info_x, extra_info_y, extra_info_font_size, extra_info_degrees).unwrap();
				} else {
					write!(output, "\t<text x=\"{3:.2}\" y=\"{4:.2}\" style=\"font-size:{5:.2}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:{2};fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, extra_info, extra_info_anchor, extra_info_x, extra_info_y, extra_info_font_size).unwrap();
				}
			}
		}
	}
	
	fn generate_breakdown_svg_elements(&mut self, mut output: &std::fs::File) {
		if !self.breakdown.is_empty() && "X" != self.breakdown {
			if self.low_ammo {
				let breakdown = strip_all_occurances(&self.breakdown, 'B');
				
				write!(output, "\t<text x=\"44.50\" y=\"33.00\" style=\"font-size:7px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">B {1}</text>\n", self.colors.text, breakdown).unwrap();
				write!(output, "\t<circle cx=\"41.25\" cy=\"30.50\" r=\"4.25\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:0.36;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", self.colors.text).unwrap();
			} else {
				write!(output, "\t<text x=\"44.50\" y=\"33.00\" style=\"font-size:7px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:end;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.breakdown).unwrap();
			}
		}
	}
	
	fn generate_leadership_svg_elements(&mut self, mut output: &std::fs::File) {
		if self.leadership_na {
			write!(output, "\t<text x=\"{0:.2}\" y=\"{1:.2}\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-anchor:middle;fill:{2};fill-opacity:1;font-family:{FONT_MAIN}\">{LEADERSHIP_NA}</text>\n", self.leadership_x, self.leadership_y, self.colors.text).unwrap();
		}
	}	

	fn generate_rof_svg_elements(&mut self, mut output: &std::fs::File) {
		if !self.rof.is_empty() {
			let size = ROF_SIZE - (2.0 * ROF_STROKE_WIDTH);
			
			generate_svg_start_element(output, 1, 33.0, 14.0, ROF_SIZE, ROF_SIZE, "ROF", "white");
			write!(output, "\t\t<rect x=\"1\" y=\"1\" width=\"{size}\" height=\"{size}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{0};stroke-width:{1};stroke-dasharray:none;stroke-opacity:1\"/>\n", self.colors.text, ROF_STROKE_WIDTH).unwrap();
			write!(output, "\t\t<text x=\"50%\" y=\"80%\" style=\"font-size:9px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};text-anchor:middle;fill:{0};fill-opacity:1;font-family:{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.rof).unwrap();
			write!(output, "\t</svg>\n").unwrap();
		}
	}
}

fn generate_svg_counter(record: &mut Record) {
	let path = format!("{0}{1}/", &record.args.destination.to_string(), &record.nationality);
	let size: u32 = if 0 != record.overrides.counter_size { record.overrides.counter_size } else { 48 };
	//
	// Create the counter file.
	//
	let output = match open_counter_file(&path, &record.piece) {
		Err(why) => panic!("couldn't create file: {0} {1}", record.piece, why),
		Ok(output) => output,
	};

	generate_counter_header_svg_elements("vasl_sw_counters", &output, size, &record.piece, &"".to_string(), &record.comments, &record.version);
	generate_counter_background_svg(&output, size, &record.colors, &record.overrides);
	generate_debug_working_area_svg(&output);

	match record.counter.as_str() {
		"Legacy" => {
			record.generate_unit_depiction_svg_elements(&output, &path);
		}
		"Malf" => {
			generate_sw_malf_counter_svg_elements(&output, record, &path);
		}
		"SFCP" => {
			generate_sfcp_counter_svg_elements(&output, record, &path);
		}
		&_ => {
			if record.counter.starts_with('_') {
				generate_counter_back_svg_elements(&output, record);
			} else {
				generate_sw_counter_svg_elements(&output, record, &path);
			}
		}
	}
	
	generate_footer_svg(&output);
	drop(output);
}

fn generate_sw_counter_svg_elements(output: &std::fs::File, record: &mut Record, path: &String) {
	
	if "X" == record.breakdown {
		record.generate_counter_malfunction_x_svg_elements(output);
	}

	record.generate_unit_depiction_svg_elements(&output, &path);
	record.generate_counter_svg_elements(&output);
}

fn generate_sw_malf_counter_svg_elements(output: &std::fs::File, record: &mut Record, path: &String) {
	record.generate_counter_malfunction_svg_elements(&output);
	record.generate_pp_svg_elements(output);
	record.generate_extra_info_svg_elements(output);
	record.generate_unit_depiction_svg_elements(&output, &path);
}

fn generate_sfcp_counter_svg_elements(output: &std::fs::File, record: &mut Record, path: &String) {
	record.generate_unit_depiction_svg_elements(&output, &path);
	record.generate_sfcp_type_svg_elements(&output);
}

fn generate_counter_back_svg_elements(output: &std::fs::File, record: &mut Record) {
	match record.counter.as_str() {
		"_baz" => {
			record.generate_baz_back_svg_elements(&output);
		}
		"_dc" => {
			record.generate_dc_back_svg_elements(&output);
		}
		"_ft" => {
			record.generate_ft_back_svg_elements(&output);
		}
		"_pf" => {
			record.generate_pf_back_svg_elements(&output);
		}
		"_pfk" => {
			record.generate_pfk_back_svg_elements(&output);
		}		
		"_phone" => {
			record.generate_phone_back_svg_elements(&output);
		}
		"_piat" => {
			record.generate_piat_back_svg_elements(&output);
		}
		"_psk" => {
			record.generate_psk_back_svg_elements(&output);
		}
		&_ => {
			panic!("Unknown counter back token '{0}'", record.counter);
		}
	}
	
}

fn generate_svg_counter_announcer(record: &mut Record) {
	if record.overrides.copy {
		let _ = copy_counter("", &record.nationality, &record.piece, &"".to_string(), &record.args);
	} else if "Ignore" != record.counter {
		if !record.args.quiet {
			print!("Generating '{0}.svg' ...", record.piece);
		} else {
			println!("{0}", record.piece);
		}
		
		generate_svg_counter(record);
		
		if !record.args.quiet {
			println!(" done.");
		}
	}
}			
//
// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
//
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "lowercase")]
struct SpreadsheetRecord {
	nationality: String,
	counter: String,
	value: String,
	range: String,
	spraying_fire: String,
	rof: String,
	pp: String,
	breakdown: String,
	repair: String,
	leadership: String,
	piece: String,
	version: String,
	overrides: String,
	svg_image_transform: String,
	comments: String,
}

impl SpreadsheetRecord {
	fn sanitize(&mut self, nat: &String, args: &Arguments) -> Record {
		let mut result: Record = Default::default();
		
		result.nationality = if nat.is_empty() { self.nationality.to_string() } else { nat.to_string() };

		result.args = args.clone();

		result.overrides.sanitize(&self.overrides);

		result.counter = self.counter.to_string();

		result.value = self.value.to_string();
		
		result.sanitize_range(&self.range);
		
		result.spraying_fire = "yes" == self.spraying_fire;
		
		result.sanitize_rof(&self.rof);

		result.sanitize_pp(&self.pp);

		result.sanitize_breakdown(&self.breakdown);

		result.sanitize_repair(&self.repair);

		result.sanitize_leadership(&self.leadership);
		
		result.pieces = extract_vector(&self.piece, OVERRIDE_DELIMITER);
		
		result.version = self.version.to_string();
		
		if !result.overrides.background_color.is_empty() {
			result.colors = nationality_to_colors(&result.overrides.background_color);
		} else {
			result.colors = nationality_to_colors(&result.nationality);
		}

		result.svg_image_transform = self.svg_image_transform.to_string();

		result.comments = self.comments.to_string();
		
		return result;
	}
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = Arguments::parse();
	
	args.sanitize_destination();
	
	let mut rdr = csv::Reader::from_reader(io::stdin());

	for result in rdr.deserialize() {
		let mut spreadsheet_record: SpreadsheetRecord = result?;

		if !spreadsheet_record.nationality.is_empty() {
			let mut record: Record = spreadsheet_record.sanitize(&"".to_string(), &args);
			let pieces = record.pieces.clone();
			
			for piece in pieces {
				if !piece.contains('@') {
					record.piece = piece.to_string();
					generate_svg_counter_announcer(&mut record);
				} else {
					let (piece, nationality) = piece.split_once("@").unwrap();
					let mut alt_record: Record = spreadsheet_record.sanitize(&nationality.to_string(), &args);
					
					alt_record.colors = nationality_to_colors(&nationality.to_string());
					alt_record.piece = piece.to_string();
					
					generate_svg_counter_announcer(&mut alt_record);
				}
			}			
		}
	}
	Ok(())
}

fn main() {
	if let Err(err) = run() {
		println!("{}", err);
		process::exit(1);
	}
}
