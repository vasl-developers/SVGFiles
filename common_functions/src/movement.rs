use const_format::formatcp;
use std::fmt;
use std::write;
use std::io::prelude::*;
use regex::Regex;
//
// Local defines.
//
use crate::colors::*;
use crate::defines::*;
use crate::overrides::*;
use crate::text_field::*;
use crate::utils::*;

pub const GP_FONT_SIZE: f64 =		6.0;
pub const GP_FONT_SIZE_STR: &str =	"6.0";

pub const GP_SIX_LOBED_ASTERISK_SVG: &str = formatcp!("<tspan style=\"font-family:{0};{FONT_WEIGHT_BOLD};font-size:{1}px\">{2}</tspan>", FONT_ALT, GP_FONT_SIZE_STR, SIX_LOBED_ASTERISK_UC);

pub const MP_FONTS: [[f64; 4]; 8] = [
	[  11.0,   5.0,  96.0,   8.0 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]		
	[  11.0,   5.0,  72.0,  11.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[  11.0,   5.0,  96.0,  11.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[  11.0,   5.0,  77.0,  14.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  96.0,   6.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  72.0,   8.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  96.0,   8.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  77.0,   9.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const TRUCK_MOVEMENT_TYPE_SUPERSCRIPT: &str =				"<sup>t</sup>";		// Replaces derived MovementType with MovementType::Truck.
pub const TRUCK_MOVEMENT_TYPE_SUPERSCRIPT_AMPHIBIOUS_2: &str =	"<sup>2t</sup>";	// Replaces derived MovementType with MovementType::Truck for amphibious vehicles.
pub const TRUCK_MOVEMENT_TYPE_SUPERSCRIPT_AMPHIBIOUS_3: &str =	"<sup>3t</sup>";	// Replaces derived MovementType with MovementType::Truck for amphibious vehicles.

pub const SIX_LOBED_ASTERISK_MP_FONT_SIZE: f64 =		6.0;
pub const SIX_LOBED_ASTERISK_MP_FONT_SIZE_STR: &str =	"6.0";

pub const SIX_LOBED_BLACK_ASTERISK_MP_SVG: &str =	formatcp!("<tspan style=\"font-family:{0};{FONT_WEIGHT_BOLD};font-size:{1}px;fill:black\" baseline-shift=\"super\">{2}</tspan>", FONT_ALT, SIX_LOBED_ASTERISK_MP_FONT_SIZE_STR, SIX_LOBED_ASTERISK_UC);

fn generate_note_actions(source: &String, note: &Note) -> std::string::String {
	let mut result: String = Default::default();
	let mut temp: String = Default::default();

	if NoteAction::Delete == note.action && source.contains(&note.text) {
		let left: &str;
		let right: &str;
		let stemp: String = source.to_string();

		(left, right) = stemp.split_once(&note.text).unwrap();

		temp.clear();
		temp.push_str(left);
		temp.push_str(right);
	} else {
		temp = source.to_string();
	}

	if NoteAction::Prefix == note.action {
		result.push_str(&note.text);
	}

	result.push_str(&temp);

	if NoteAction::Postfix == note.action {
		result.push_str(&note.text);
	}

	return result;
}

#[derive(PartialEq)]
#[derive(Default)]
pub enum MovementType {
	FullyTracked,
	HalfTracked,
	ArmoredCar,
	Truck,
	Skis,
	Motorcycle,
	Nimbus,
	#[default]
	None,
}

impl fmt::Display for MovementType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MovementType::FullyTracked => write!(f, "FullyTracked"),
            MovementType::HalfTracked => write!(f, "HalfTracked"),
            MovementType::ArmoredCar => write!(f, "ArmoredCar"),
            MovementType::Truck => write!(f, "Truck"),
			MovementType::Skis => write!(f, "Skis"),
			MovementType::Motorcycle => write!(f, "Motorcycle"),
			MovementType::Nimbus => write!(f, "Nimbus"),
			MovementType::None => write!(f, "None"),
        }
    }
}

#[derive(PartialEq)]
#[derive(Default)]
pub enum GroundPressure {
	Low,
	#[default]
	Normal,
	High,
}

#[derive(PartialEq)]
#[derive(Default)]
pub struct VehicleMovementValues {
	pub points: TextField,
	pub mt: MovementType,
	pub mt_color: String,
	pub gp: GroundPressure,
	pub gp_note: Note,
	pub gp_color: String,
	pub rfnm: bool,
}

impl VehicleMovementValues {
	pub fn sanitize(&mut self, name: &String, mps: &String, gp: &String, overrides: &Overrides, open_topped: bool, colors: &Colors) {
		let movement_type;
	
		if !overrides.movement_type.is_empty() {
			movement_type = overrides.movement_type.clone();
		} else if mps.contains(TRUCK_MOVEMENT_TYPE_SUPERSCRIPT) || mps.contains(TRUCK_MOVEMENT_TYPE_SUPERSCRIPT_AMPHIBIOUS_2) || mps.contains(TRUCK_MOVEMENT_TYPE_SUPERSCRIPT_AMPHIBIOUS_3) {
			movement_type = "tr".to_string();
		} else if name.contains("<i>") {
			movement_type = strip_html_italics(name);
		} else {
			movement_type = name.to_string();
		}
	
		match movement_type.as_str() {
			"AAMT" | "AATt" | "AG" | "AG/TD" | "APC" | "APCv" | "HT" | "HTt" | "HTv" | "LT" | "LTv" | "MT" | "MTv" | "PC" | "SPA" | "SPAA" | "TD" | "TDv" | "Tt" | "Ttv" | "aAPC" | "aLT" | "aMT" | "aPC" | "aTt" | "TD/AA" => {
				self.mt = MovementType::FullyTracked;
			}
			"AAht" | "SPAht" | "TDht" | "ht" | "htMC" => {
				self.mt = MovementType::HalfTracked;
			}
			"AAAC" | "AC" | "AC/TD" => {
				self.mt = MovementType::ArmoredCar;
			}
			"SC" | "TDtr" | "tr" | "atr" => {
				self.mt = MovementType::Truck;
			}
			"AS" => {
				self.mt = MovementType::Skis;
			}
			"mc" => {
				self.mt = MovementType::Motorcycle;
			}
			"TDmc" => {
				self.mt = MovementType::Nimbus;
			}
			"AAh-d" | _ => {
				self.mt = MovementType::None;
			}
		}
	
		if open_topped {
			self.mt_color = colors.movement_type_open_topped.to_string();
		} else {
			self.mt_color = colors.movement_type.to_string();
		}
	
		let my_gp: String;
	
		if !overrides.ground_pressure.is_empty() {
			my_gp = overrides.ground_pressure.clone();
		} else {
			my_gp = gp.to_string();
		}
	
		if my_gp.contains('H') {
			self.gp = GroundPressure::High;
		} else if my_gp.contains('L') {
			self.gp = GroundPressure::Low;
		}
	
		if my_gp.contains(DAGGER) {
			self.gp_note.text = GP_SIX_LOBED_ASTERISK_SVG.to_string();
			self.gp_note.action = NoteAction::Postfix;
		}
	
		self.gp_color = colors.text.to_string();
	
		let mut my_mps: String;
	
		if !overrides.mp.is_empty() {
			my_mps = convert_text(&overrides.mp, COPY_FIELD, &mps.to_string());
			if my_mps.contains(DAGGER) {
				my_mps = strip_dagger_and_any_superscript_from_end(&my_mps);
			}
		} else {
			my_mps = mps.to_string();
		}
	
		if my_mps.contains(FIVE_LOBED_ASTERISK_TAG) {
			my_mps = convert_text(&my_mps, FIVE_LOBED_ASTERISK_TAG, &FIVE_LOBED_ASTERISK_UC.to_string());
		}
				
		if my_mps.contains(SIX_LOBED_ASTERISK_TAG) {
			my_mps = convert_text(&my_mps, SIX_LOBED_ASTERISK_TAG, SIX_LOBED_BLACK_ASTERISK_MP_SVG);
		}
	
		if my_mps.contains(BLACK_ASTERISK_TAG) {
			my_mps = convert_text(&my_mps, BLACK_ASTERISK_TAG, BLACK_ASTERISK_SVG);
		} else if my_mps.contains(WHITE_ASTERISK_TAG) {
			my_mps = convert_text(&my_mps, WHITE_ASTERISK_TAG, WHITE_ASTERISK_SVG);
		}
	
		self.points.fonts.initialize(MP_FONTS);
		
		if my_mps.contains("<b>") {
			self.points.color = RED.to_string();
			self.points.text.push_str(&strip_html_bold(&my_mps));
		} else {
			self.points.color = colors.movement_points_text.to_string();
			self.points.text = my_mps;
		}
	
		if MovementType::Skis == self.mt {
			self.points.text = extract_movement_points(&self.points.text);
		} else if self.points.text.contains(DAGGER) {
			self.points.text = strip_dagger_and_any_superscript_from_end(&self.points.text);
			self.points.note.text = BLACK_ASTERISK_SVG.to_string();
			self.points.note.action = NoteAction::Prefix;
		}
	
		if self.points.text.contains(TRUCK_MOVEMENT_TYPE_SUPERSCRIPT) {
			self.points.text = strip_superscript(&self.points.text);
		}
	
		if self.points.text.contains(TRUCK_MOVEMENT_TYPE_SUPERSCRIPT_AMPHIBIOUS_2) || self.points.text.contains(TRUCK_MOVEMENT_TYPE_SUPERSCRIPT_AMPHIBIOUS_3) {
			self.points.text = convert_text(&self.points.text, "t</sup>", "</sup>");
		}
		
		self.rfnm = overrides.rfnm;
	}

	pub fn generate_svg_elements(&mut self, mut counter_file: &std::fs::File, colors: &Colors) {
		if !self.rfnm {
			generate_svg_start_element(counter_file, 1, 30.0, 3.0, 27.0, 12.0, "Movement", &"blue".to_string());
	
			match self.mt {
				MovementType::FullyTracked => {
					write!(counter_file, "\t\t<rect x=\"6.00\" y=\"0.00\" ry=\"6.00\" width=\"21.00\" height=\"12.00\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.mt_color).unwrap();
				}
				MovementType::HalfTracked => {
					write!(counter_file, "\t\t<circle cx=\"10.00\" cy=\"75%\" r=\"3.00\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:6;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", self.mt_color).unwrap();
					write!(counter_file, "\t\t<rect x=\"12.00\" y=\"0.00\" ry=\"6.00\" width=\"15.00\" height=\"12.00\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.mt_color).unwrap();
				}
				MovementType::ArmoredCar => {
					write!(counter_file, "\t\t<circle cx=\"21.00\" cy=\"50%\" r=\"6.00\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:6;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", self.mt_color).unwrap();
				}
				MovementType::Truck => {
					write!(counter_file, "\t\t<circle cx=\"20.00\" cy=\"50%\" r=\"6.00\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:6;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", self.mt_color).unwrap();
					write!(counter_file, "\t\t<circle cx=\"8.00\" cy=\"50%\" r=\"6.00\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:6;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", self.mt_color).unwrap();
				}
				MovementType::Skis => {
					write!(counter_file, "\t\t<rect x=\"7.00\" y=\"50%\" ry=\"0.00\" width=\"19.00\" height=\"3.00\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.mt_color).unwrap();
				}
				MovementType::Motorcycle | MovementType::Nimbus | MovementType::None => {
				}
			}
			//
			// Construct the movement points text while adding any needed asterisk.
			//
			let mut movement_points_svg: String;
			
			if colors.is_ss {
				movement_points_svg = format!("\t\t<text x=\"100%\" y=\"{0:.2}%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};fill:{3};stroke:{4};stroke-width:0.33\">", &self.points.fonts.y_percentage(), &self.points.fonts.size(), &FONT_MAIN.to_string(), &self.points.color, &self.mt_color);
			} else {
				movement_points_svg = format!("\t\t<text x=\"100%\" y=\"{0:.2}%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};fill:{3}\">", &self.points.fonts.y_percentage(), &self.points.fonts.size(), &FONT_MAIN.to_string(), &self.points.color);
			}
			
			let super_size: f64 = self.points.fonts.size() - 3.0;
			movement_points_svg.push_str(&generate_note_actions(&convert_superscripts(&self.points.text, super_size), &self.points.note));
			movement_points_svg.push_str("</text>\n");
	
			write!(counter_file, "{}", movement_points_svg).unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();
			//
			// Generate Ground Pressure elements if needed.
			//
			if GroundPressure::Normal != self.gp || NoteAction::None != self.gp_note.action {
				generate_svg_start_element(counter_file, 1, 2.0, 2.0, 10.0, 10.0, "Ground Pressure", &"white".to_string());
	
				if NoteAction::None != self.gp_note.action {
					write!(counter_file, "\t\t<text x=\"50%\" y=\"75%\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};fill:{1};fill-opacity:1\">{2}</text>\n", GP_FONT_SIZE, colors.text, &self.gp_note.text).unwrap();
				}
	
				if GroundPressure::Low == self.gp {
					write!(counter_file, "\t\t<rect x=\"1.00\" y=\"1.00\" width=\"8.00\" height=\"8.00\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:0.50;stroke-dasharray:none;stroke-opacity:1\"/>\n", colors.text).unwrap();
				} else if GroundPressure::High == self.gp {
					write!(counter_file, "\t\t<circle cx=\"50%\" cy=\"50%\" r=\"4.00\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:0.50;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", colors.text).unwrap();
				}

				write!(counter_file, "\t</svg>\n").unwrap();
			}
			//
			// Handle Motorcycle size element (Nimbus).
			//
			if MovementType::Nimbus == self.mt {
				generate_svg_start_element(counter_file, 1, 3.0, 12.0, 10.0, 10.0, "Nimbus Motorcycle size", &"pink".to_string());
	
				write!(counter_file, "\t\t\t<text x=\"0.00\" y=\"70%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">-1</text>\n", MOTORCYCLE_FONT_SIZE, FONT_MAIN, colors.text).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			} else if MovementType::Motorcycle == self.mt {
				generate_svg_start_element(counter_file, 1, MGS_LINE_X_POSITION, MGS_LINE_2_Y_POSITION - MOTORCYCLE_FONT_SIZE, 36.0, MOTORCYCLE_FONT_SIZE, "Motorcycle size", &"pink".to_string());
	
				write!(counter_file, "\t\t\t<text x=\"100%\" y=\"70%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">-1</text>\n", MOTORCYCLE_FONT_SIZE, FONT_MAIN, colors.text).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			}			
		} else {
			let font_size = self.points.fonts.size() - 3.0;
			
			generate_svg_start_element(counter_file, 1, 33.0, 3.0, 24.0, 12.0, "No Movement", &"cyan".to_string());
			write!(counter_file, "\t\t<text x=\"100%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};fill:{3}\">RFNM</text>\n", self.points.fonts.y_percentage(), font_size, FONT_MAIN, BLACK).unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();
		}
	}
}

pub const MANHANDLING_IMMOBLE: &str = "–†";

pub const MH_NUMBER_FONTS: [[f64; 4]; 8] = [
	[   8.4,   4.8,   5.9,   6.3 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const MH_NOTE_FONTS: [[f64; 4]; 8] = [
	[  10.8,   7.7,   7.6,   8.5 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const MH_M_FONTS: [[f64; 4]; 8] = [
	[   8.4,   4.8,   5.9,   6.3 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const MH_NOTE_FONT_SIZE: f64 =	10.8;
pub const MH_M_FONT_SIZE: f64 =		 8.4;

#[derive(PartialEq)]
#[derive(Default)]
pub struct OrdnanceMovementValues {
	pub manhandling_number: TextField,
	pub color: String,
	pub target_size: i64,
	pub unhooking_penalty: bool,
	pub unhooking_penalty_color: String,
}

impl OrdnanceMovementValues {
	pub fn sanitize(&mut self, manhandling: &String, target_size: i64, special: &String, overrides: &Overrides, colors: &Colors) {
		self.color = colors.manhandling_fill.to_string();	// For 'M' and asterisk (if applicable)
		self.manhandling_number.fonts.initialize(MH_NUMBER_FONTS);
	
		if !overrides.target_size.is_empty() {
			self.target_size = overrides.target_size.parse::<i64>().unwrap_or(0);
		} else {
			self.target_size = target_size;
		}
	
		if 0 < self.target_size {
			self.manhandling_number.color = BLACK.to_string();
			self.unhooking_penalty_color = colors.unhooking_penalty_color.to_string();
		} else if 0 == self.target_size {
			self.manhandling_number.color = colors.manhandling_fill.to_string();
			self.unhooking_penalty_color = colors.unhooking_penalty_color.to_string();
		} else {
			self.manhandling_number.color = RED.to_string();
			self.unhooking_penalty_color = RED.to_string();
		}
	
		self.manhandling_number.text = manhandling.to_string();

		if !overrides.manhandling.is_empty() {
			self.manhandling_number.process_overrides(&overrides.manhandling, LEAVE_ASTERISK_TAGS);
			
			if self.manhandling_number.text.contains(FIVE_LOBED_ASTERISK_TAG) {
				self.manhandling_number.note.action = NoteAction::Infix;
				self.manhandling_number.note.text = FIVE_LOBED_ASTERISK_UC.to_string();
				self.manhandling_number.text = convert_text(&self.manhandling_number.text.to_string(), FIVE_LOBED_ASTERISK_TAG, &"".to_string());
			}
		}
	
		if MANHANDLING_IMMOBLE == manhandling {
			self.manhandling_number.text = FIVE_LOBED_ASTERISK_UC.to_string();
		}
	
		if special.contains("NM") { // Includes "RFNM".
			self.manhandling_number.note.action = NoteAction::Infix;
			self.manhandling_number.note.text = FIVE_LOBED_ASTERISK_UC.to_string();
		}

		if self.manhandling_number.text.contains(DAGGER) {
			self.manhandling_number.text = strip_dagger_and_any_superscript_from_end(&self.manhandling_number.text);
			self.manhandling_number.note.action = NoteAction::Infix;
			self.manhandling_number.note.text = FIVE_LOBED_ASTERISK_UC.to_string();
		}		

		if self.manhandling_number.text.contains("<b>") {
			self.manhandling_number.text = extract_string(&self.manhandling_number.text, &String::from("</b>"), &String::from("<b>"));
			self.unhooking_penalty = true;
		} else {
			self.unhooking_penalty = false;
		}
	}
	
	pub fn set_font_and_color(&mut self, colors: &Colors) {
		self.manhandling_number.fonts.initialize(MH_NUMBER_FONTS);
		self.color = colors.manhandling_fill.to_string();
	}
}

#[derive(PartialEq)]
#[derive(Default)]
pub struct RepairValues {
	pub repair: TextField,
	pub disable: TextField,
}

pub fn generate_manhandling_number(mut counter_file: &std::fs::File, movement: &OrdnanceMovementValues) {
	let manhandling_number_color: String = movement.manhandling_number.color.to_string();

	if 1 != movement.target_size && !movement.unhooking_penalty {
		write!(counter_file, "\t<text x=\"57.00\" y=\"20.40\" dominant-baseline=\"auto\" text-anchor=\"end\"><tspan style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">M</tspan><tspan style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{3};fill-opacity:1\">{4}</tspan></text>\n", movement.manhandling_number.fonts.size(), FONT_MAIN, movement.color, manhandling_number_color, movement.manhandling_number.text).unwrap();
	} else {
		let mut font_size = movement.manhandling_number.fonts.size();
		
		if 2 <= movement.manhandling_number.text.len() {
			font_size -= 1.2;
		}
		
		generate_svg_start_element(counter_file, 1, 48.0, 14.4, 9.0, 9.0, "Manhandling #", &"orange".to_string());

		if 1 == movement.target_size {
			write!(counter_file, "\t\t<circle cx=\"50%\" cy=\"50%\" r=\"4.20\" style=\"display:inline;fill:white;fill-opacity:1;stroke:none;stroke-width:0.36;stroke-dasharray:none;stroke-opacity:1\"></circle>\n").unwrap();
		}

		if movement.unhooking_penalty {
			write!(counter_file, "\t\t<circle cx=\"50%\" cy=\"50%\" r=\"4.20\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:0.36;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", movement.unhooking_penalty_color.to_string()).unwrap();
		}

		write!(counter_file, "\t\t<text x=\"50%\" y=\"80%\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">{3}</text>\n", font_size, FONT_MAIN, manhandling_number_color, movement.manhandling_number.text).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();

		generate_svg_start_element(counter_file, 1, 40.2, 13.8, 10.2, 10.2, "Manhandling", &"yellow".to_string());

		if NoteAction::Infix == movement.manhandling_number.note.action {
			write!(counter_file, "\t\t<text x=\"80%\" y=\"64%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">M</text>\n", MH_M_FONT_SIZE, FONT_MAIN, movement.color).unwrap();
			write!(counter_file, "\t\t<text x=\"80%\" y=\"143%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">{3}</text>\n", MH_NOTE_FONT_SIZE, FONT_MAIN, movement.color, movement.manhandling_number.note.text).unwrap();
		} else if movement.unhooking_penalty || 1 == movement.target_size {
			write!(counter_file, "\t\t<text x=\"75%\" y=\"80%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">M</text>\n", movement.manhandling_number.fonts.size(), FONT_MAIN, movement.color).unwrap();
		} else {
			write!(counter_file, "\t\t<text x=\"100%\" y=\"80%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">M</text>\n", movement.manhandling_number.fonts.size(), FONT_MAIN, movement.color).unwrap();
		}

		write!(counter_file, "\t</svg>\n").unwrap();
	}
}

pub fn generate_motorcycle_manhandling_number_element(mut counter_file: &std::fs::File, mh: &TextField, m_color: &String) {
	let mut x_pos: f64 = 57.0;
	let mut y_pos: f64 = 24.0;
	let mut anchor = "end".to_string();
	
	if MOD_LOCATION_GS == mh.alternate_location {
		x_pos = GUN_COLUMN_X_POSITION;
		y_pos = GUN_COLUMN_Y_POSITION;
		anchor = "start".to_string();
	}
	
	write!(counter_file, "\t<!-- Motorcycle Manhandling -->\n").unwrap();
	write!(counter_file, "\t<text x=\"{0:.2}\" y=\"{1:.2}\" dominant-baseline=\"auto\" text-anchor=\"{anchor}\"><tspan style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4};fill-opacity:1\">M</tspan><tspan style=\"font-size:{2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{5};fill-opacity:1\">{6}</tspan></text>\n", x_pos, y_pos, mh.fonts.size(), FONT_MAIN, m_color, mh.color, mh.text).unwrap();	
}

pub fn generate_boat_manhandling_number_element(mut counter_file: &std::fs::File, mh: &TextField, m_color: &String) {
	let mut fonts: FontsObj = Default::default();
	
	fonts.initialize(MH_NUMBER_FONTS);
	// self.manhandling_number.fonts.initialize(MH_NUMBER_FONTS);
		
	write!(counter_file, "\t<!-- Nimbus Manhandling -->\n").unwrap();
	write!(counter_file, "\t<text x=\"3.00\" y=\"48.00\" dominant-baseline=\"auto\" text-anchor=\"start\"><tspan style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2};fill-opacity:1\">M</tspan><tspan style=\"font-size:{0}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{3};fill-opacity:1\">{4}</tspan></text>\n", fonts.size(), FONT_MAIN, m_color, m_color, mh.text).unwrap();	
}

fn extract_movement_points(original: &String) -> std::string::String {
	let mut result: String = Default::default();

	let re = Regex::new(r"(?<keep>[0-9][0-9]*)(.*)").unwrap();
	let Some(caps) = re.captures(&original) else { panic!("extract_movement_points regex failed!") };

	result.push_str(&caps["keep"]);

	return result;
}
