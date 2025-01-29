use std::write;
use std::io::prelude::*;
// Local files.
//
use crate::colors::*;
use crate::defines::*;
use crate::overrides::*;
use crate::text_field::*;
use crate::utils::*;

pub const BREAKDOWN_FONTS: [[f64; 4]; 8] = [
	[   8.4,   6.0,   0.0,   6.6 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

#[derive(PartialEq)]
#[derive(Default)]
pub struct Malf {
	pub category: char,	// 'B' or 'X' or char::default() if not configured.
	pub value: TextField,
	pub low_ammo: bool,
	pub number_width: f64,
	pub superscript: String,
}

impl Malf {
	pub fn sanitize(&mut self, source: &String, superscript: &String, alternate_location: &String, size_adjustment: f64, colors: &Colors) {
		let mut temp: String = source.to_string();

		self.value.fonts.initialize(BREAKDOWN_FONTS);
		
		if 0.0 != size_adjustment {
			self.value.fonts.adjust_size(size_adjustment);
		}
		
		if temp.starts_with(FIVE_LOBED_ASTERISK_TAG) {
			self.value.note.action = NoteAction::Prefix;
			self.value.note.text = FIVE_LOBED_ASTERISK_UC.to_string();
			temp = convert_text(&temp, FIVE_LOBED_ASTERISK_TAG, &"".to_string());
		} else if temp.starts_with(SIX_LOBED_ASTERISK_TAG) {
			self.value.note.action = NoteAction::Prefix;
			self.value.note.text = SIX_LOBED_ASTERISK_SUPER_SVG.to_string();			
			temp = convert_text(&temp, SIX_LOBED_ASTERISK_TAG, &"".to_string());
		}
		
		if temp.contains(SIX_LOBED_ASTERISK_TAG) {
			self.value.note.action = NoteAction::Postfix;
			self.value.note.text = SIX_LOBED_ASTERISK_SUPER_SVG.to_string();			
			temp = convert_text(&temp, SIX_LOBED_ASTERISK_TAG, &"".to_string());
		} else if temp.contains(SIX_LOBED_BLACK_ASTERISK_TAG) {
			self.value.note.action = NoteAction::Postfix;
			self.value.note.text = SIX_LOBED_BLACK_ASTERISK_SUPER_SVG.to_string();			
			temp = convert_text(&temp, SIX_LOBED_BLACK_ASTERISK_TAG, &"".to_string());
		} else if temp.contains(FIVE_LOBED_ASTERISK_TAG) {
			self.value.note.action = NoteAction::Postfix;
			self.value.note.text = FIVE_LOBED_ASTERISK_UC.to_string();
			temp = convert_text(&temp, &FIVE_LOBED_ASTERISK_TAG.to_string(), &"".to_string());
		}

		if temp.contains("<i>") {
			self.low_ammo = true;
			temp = strip_html_italics_only(&temp);
		} else {
			self.low_ammo = false;
		}

		self.superscript = superscript.to_string();

		if temp.contains("<b>") {
			temp = strip_html_bold(&temp);
			self.value.color = RED.to_string();
		} else {
			self.value.color = colors.text.to_string();
		}
		
		self.category = 'B';

		if temp.contains("X") {
			self.category = 'X';
			temp.remove(0);
		} else if temp.contains("B") {
			temp.remove(0);
		}

		self.value.text = temp;
		self.value.alternate_location = alternate_location.to_string();
	}
}

#[derive(PartialEq)]
#[derive(Default)]
pub struct Malfunction {
	pub breakdown: Malf,
	pub disable: Malf,
}

impl Malfunction {
	pub fn sanitize(&mut self, source: &String, overrides: &ArmamentOverrides, colors: &Colors) {
		if !overrides.malf.ignore && !overrides.ignore {
			let mut breakdown: String = source.clone();
			let mut superscript: String = Default::default();
			let mut size_adjustment: f64 = 0.0;
			let mut alternate_location: String = Default::default();
		
			if !overrides.malf.text.is_empty() {
				let entries: Vec<std::string::String> = extract_vector(&overrides.malf.text, MOD_DELIMITER2);
				let mut malf_value_read: bool = false;
				
				for entry in &entries {
					if !malf_value_read {
						breakdown = strip_dagger_and_any_superscript_from_end(&source);
						breakdown = convert_text(&entry, COPY_FIELD, &breakdown);
						malf_value_read = true;
					} else if entry.contains(MOD_INC_SIZE) {
						let temp: String = extract_from(&entry, MOD_INC_SIZE);
					
						size_adjustment = temp.parse::<f64>().unwrap_or(0.0);
					} else if entry.contains(MOD_DEC_SIZE) {
						let temp: String = extract_from(&entry, MOD_DEC_SIZE);
						
						size_adjustment = temp.parse::<f64>().unwrap_or(0.0) * -1.0;
					} else if is_alternate_location(entry.to_string()) {
						alternate_location = entry.to_string();
					} else {
						panic!("sanitize_special_ammo()@{0}: unimplemented! entry '{1}'", line!(), entry);
					}
				}				
			} else {
				breakdown = strip_dagger_and_any_superscript_from_end(&breakdown);
			}
			
			if breakdown.contains(SUPER) {
				let entries: Vec<std::string::String> = extract_vector(&breakdown, SUPER);
		
				breakdown = entries[0].clone();
				superscript = entries[1].clone();
			}
			
			if !breakdown.is_empty() && DAGGER.to_string() != breakdown {
				self.breakdown.sanitize(&breakdown, &superscript, &alternate_location, size_adjustment, &colors);
			}
		}
	}
}

pub fn generate_malfunction_element(mut counter_file: &std::fs::File, malf: &Malf, x_position: f64, y_position: f64, anchor: &str) -> f64 {
	let mut my_x_position = x_position;
	let mut my_y_position = y_position - GUN_COLUMN_BREAKDOWN_HEIGHT;
	let mut my_anchor = anchor;
	let mut result: f64 = malf.value.fonts.height();

	if MOD_LOCATION_ABOVE_MGS == malf.value.alternate_location {
		//
		// Thanks POA-CWS-H5 Flame Tank!
		//
		my_x_position = 58.5; // Magic!
		my_y_position = MGS_LINE_2_Y_POSITION - GUN_COLUMN_BREAKDOWN_HEIGHT;
		my_anchor = "end";
		result = 0.0;
	}

	if !malf.low_ammo { // Simple case, construct a single B/X element.
		let mut breakdown_string: String = Default::default();
		my_x_position = 3.0;

		my_y_position = y_position;

		if NoteAction::Prefix == malf.value.note.action {
			breakdown_string.push_str(&malf.value.note.text.to_string());
		}

		breakdown_string.push_str(&malf.category.to_string());
		breakdown_string.push_str(&malf.value.text);

		if NoteAction::Postfix == malf.value.note.action {
			if SIX_LOBED_ASTERISK_UC == malf.value.note.text {
				breakdown_string.push_str(SIX_LOBED_ASTERISK_SUPER_SVG);
			} else {
				breakdown_string.push_str(&malf.value.note.text.to_string());
			}
		}

		if !malf.superscript.is_empty() {
			breakdown_string.push_str(&wrap_superscripts(&malf.superscript, malf.value.fonts.sup_size()));
		}

		if my_anchor.contains("end") {
			my_x_position = GUN_COLUMN_BREAKDOWN_WIDTH;
		}

		generate_svg_start_element(counter_file, 1, my_x_position, my_y_position - malf.value.fonts.height(), 24.0, malf.value.fonts.height(), "Malfunction", "yellow");
		write!(counter_file, "\t\t<text x=\"0\" y=\"98%\" dominant-baseline=\"auto\" text-anchor=\"{0}\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_NORM};font-family:{2};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", anchor, malf.value.fonts.size(), FONT_MAIN, malf.value.color, breakdown_string).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
		result += gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, my_y_position - malf.value.fonts.height(), "green");
	} else {
		result = GUN_COLUMN_BREAKDOWN_HEIGHT;
		
		if !my_anchor.contains("end") {
			if NoteAction::Prefix == malf.value.note.action {
				generate_svg_start_element(counter_file, 1, my_x_position, my_y_position, 3.0, GUN_COLUMN_BREAKDOWN_HEIGHT, "Malfunction *", "white");
				write!(counter_file, "\t\t<text x=\"0\" y=\"100%\" dominant-baseline=\"auto\" text-anchor=\"{0}\"><tspan style=\"font-size:{1:.2}px;font-family:{2};{FONT_WEIGHT_NORM};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", my_anchor, malf.value.fonts.size() + BREAKDOWN_NOTE_FONT_POS_DELTA, FONT_ALT, malf.value.color, malf.value.note.text).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
				my_x_position += 3.0;
			}

			generate_svg_start_element(counter_file, 1, my_x_position, my_y_position, 5.4, GUN_COLUMN_BREAKDOWN_HEIGHT, "Malfunction B/X", "yellow");
			write!(counter_file, "\t\t<text x=\"0\" y=\"76%\" dominant-baseline=\"auto\" text-anchor=\"{0}\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_NORM};font-family:{2};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", my_anchor, malf.value.fonts.size(), FONT_MAIN, malf.value.color, malf.category).unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();
			my_x_position += 5.4;

			generate_svg_start_element(counter_file, 1, my_x_position, my_y_position, GUN_COLUMN_BREAKDOWN_HEIGHT, GUN_COLUMN_BREAKDOWN_HEIGHT, "Malfunction B/X Number", "orange");

			let mut font_size = malf.value.fonts.size();
			
			if malf.low_ammo {
				if 2 <= malf.value.text.len() {
					font_size -= 1.2;
				}
				
				write!(counter_file, "\t\t<circle cx=\"50%\" cy=\"50%\" r=\"4.5\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:0.36;stroke-dasharray:none;stroke-opacity:1\"></circle> <!-- Low Ammo circle. Stroke width increases the radius! -->\n", malf.value.color).unwrap();
			}
			
			write!(counter_file, "\t\t<text x=\"50%\" y=\"76%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{0:.2}px;{FONT_WEIGHT_NORM};font-family:{1};fill:{2};fill-opacity:1;stroke-width:0.2\">{3}</tspan></text>\n", font_size, FONT_MAIN, malf.value.color, malf.value.text).unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();

			my_x_position += GUN_COLUMN_BREAKDOWN_HEIGHT;

			if !malf.superscript.is_empty() {
				let date = wrap_superscripts(&malf.superscript, malf.value.fonts.sup_size());
				
				generate_svg_start_element(counter_file, 1, my_x_position, my_y_position, 8.0 /* Magic! */, GUN_COLUMN_BREAKDOWN_HEIGHT, "Malfunction B/X Date", "crimson");
				write!(counter_file, "\t\t<text x=\"0\" y=\"86%\" dominant-baseline=\"auto\" text-anchor=\"{0}\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_NORM};font-family:{2};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", my_anchor, malf.value.fonts.size(), FONT_MAIN, malf.value.color, date).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			}
		} else {
			//
			// Draw elements right-to-left.
			//
			my_anchor = "start";
			
			if !malf.superscript.is_empty() {
				// Not sure if this is a case we need to implement yet ...
				let date = wrap_superscripts(&malf.superscript, malf.value.fonts.sup_size());
				
				my_x_position -= 6.0;
				generate_svg_start_element(counter_file, 1, my_x_position, my_y_position, 8.0 /* Magic! */, GUN_COLUMN_BREAKDOWN_HEIGHT, "Malfunction B/X Date", "crimson");
				write!(counter_file, "\t\t<text x=\"0\" y=\"86%\" dominant-baseline=\"auto\" text-anchor=\"{0}\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_NORM};font-family:{2};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", my_anchor, malf.value.fonts.size(), FONT_MAIN, malf.value.color, date).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			}

			my_x_position -= GUN_COLUMN_BREAKDOWN_HEIGHT;
			generate_svg_start_element(counter_file, 1, my_x_position, my_y_position, GUN_COLUMN_BREAKDOWN_HEIGHT, GUN_COLUMN_BREAKDOWN_HEIGHT, "Malfunction B/X Number", "orange");
			write!(counter_file, "\t\t<text x=\"50%\" y=\"76%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{0:.2}px;{FONT_WEIGHT_NORM};font-family:{1};fill:{2};fill-opacity:1;stroke-width:0.2\">{3}</tspan></text>\n", malf.value.fonts.size(), FONT_MAIN, malf.value.color, malf.value.text).unwrap();

			if malf.low_ammo {
				write!(counter_file, "\t\t<circle cx=\"50%\" cy=\"50%\" r=\"4.5\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:0.36;stroke-dasharray:none;stroke-opacity:1\"></circle> <!-- Low Ammo circle. Stroke width increases the radius! -->\n", malf.value.color).unwrap();
			}
			write!(counter_file, "\t</svg>\n").unwrap();		
			
			my_x_position -= 5.4; 
			generate_svg_start_element(counter_file, 1, my_x_position, my_y_position, 5.4, GUN_COLUMN_BREAKDOWN_HEIGHT, "Malfunction B/X", "yellow");
			write!(counter_file, "\t\t<text x=\"0\" y=\"76%\" dominant-baseline=\"auto\" text-anchor=\"{0}\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_NORM};font-family:{2};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", my_anchor, malf.value.fonts.size(), FONT_MAIN, malf.value.color, malf.category).unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();

			if NoteAction::Prefix == malf.value.note.action {
				my_x_position -= 4.2;
				generate_svg_start_element(counter_file, 1, my_x_position, my_y_position, 4.2, GUN_COLUMN_BREAKDOWN_HEIGHT, "Malfunction *", "white");
				write!(counter_file, "\t\t<text x=\"0\" y=\"100%\" dominant-baseline=\"auto\" text-anchor=\"{0}\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_NORM};fill:{2};fill-opacity:1;stroke-width:0.2\">{3}</tspan></text>\n", my_anchor, malf.value.fonts.size() + BREAKDOWN_NOTE_FONT_POS_DELTA, malf.value.color, malf.value.note.text).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			}
		}

		result += gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, my_y_position, "lightgreen");
	}

	return result;
}

pub fn generate_malfunction_elements(counter_file: &std::fs::File, malfunction: &Malfunction, y_position: f64) -> f64 {
	let mut result: f64 = 0.0;

	if char::default() != malfunction.disable.category { // 'X' takes precedence?
		result += generate_malfunction_element(&counter_file, &malfunction.breakdown, GUN_COLUMN_X_POSITION, y_position, "start");

		if char::default() != malfunction.breakdown.category {
			result += generate_malfunction_element(&counter_file, &malfunction.breakdown, ALT_BREAKDOWN_X_POSITION, y_position, "end");
		}
	} else if char::default() != malfunction.breakdown.category {
		result += generate_malfunction_element(&counter_file, &malfunction.breakdown, GUN_COLUMN_X_POSITION, y_position, "start");
	}

	return result;
}
