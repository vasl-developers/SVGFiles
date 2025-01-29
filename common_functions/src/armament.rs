use const_format::formatcp;
use regex::Regex;
use std::write;
use std::io::prelude::*;
//
// Local files.
//
use crate::colors::*;
use crate::common_record::*;
use crate::defines::*;
use crate::overrides::*;
use crate::text_field::*;
use crate::turret::*;
use crate::utils::*;

pub const ROF_HEIGHT: f64 =			11.22;
pub const ROF_STROKE_WIDTH: f64 =	 0.50;
pub const ROF_OFFSET: f64 =			ROF_STROKE_WIDTH / 2.0;
pub const ROF_BOX_SIZE: f64 =		ROF_HEIGHT - ROF_STROKE_WIDTH;

pub const MA_FONTS: [[f64; 4]; 8] = [
	[  12.0,   6.0,  75.00,  12.0 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]		
	[  12.0,   6.0,  60.00,  15.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[  12.0,   6.0,  80.00,  15.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[  12.0,   6.0,  66.67,  18.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  66.67,   9.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  54.55,  11.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  72.73,  11.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  61.54,  13.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const SA_FONTS: [[f64; 4]; 8] = [
	[   8.0,   4.0,  98.00,   6.0 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  75.00,   8.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  98.00,   8.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  80.00,  10.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  98.00,   6.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  75.00,   8.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  98.00,   8.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  80.00,  10.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const ROF_FONTS: [[f64; 4]; 8] = [
	[   8.4,   6.0,  75.0,   6.6 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const IFE_FONTS: [[f64; 4]; 8] = [
	[   8.4,   6.0,  82.0,   8.6 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const MA_MOVT_WHITE_CIRCLE: &str =					formatcp!("<tspan style=\"font-size:{0}px;fill:white\" baseline-shift=\"super\">{1}</tspan></text>", "6.00" /* MA_FONTS[FONT_NORMAL][FA_SUP_SIZE]*/, CIRCLE);
pub const SA_MOVT_WHITE_CIRCLE: &str =					formatcp!("<tspan style=\"font-size:{0}px;fill:white\" baseline-shift=\"super\">{1}</tspan></text>", "4.80" /* SA_FONTS[FONT_NORMAL][FA_SUP_SIZE]*/, CIRCLE);

#[derive(PartialEq)]
#[derive(Default)]
pub struct Armament {
	pub caliber_note: Note,
	pub location: String,		// "B", "T", "S", also used for Flamethrower indicators ...
	pub caliber: String,		// Includes "CMG", "BMG", "TTACP", etc.
	pub velocity: String,		// '*', 'L', 'LL'

	pub rof: TextField,
	pub ife: TextField,
	pub range: TextField,
	pub range2: TextField,
	pub special_ammo: TextField,
	pub multiple_hits: bool,
	pub color: String,
	pub underline_note: Note,
	pub raw_caliber: String,
	pub overline: bool,
	pub underline: bool,
	pub fonts: FontsObj,
	pub is_secondary: bool,
}

impl Armament {
	pub fn sanitize(&mut self, source: &String, range: &String, rof_ife: &String, overrides: &Overrides, colors: &Colors) {
		self.raw_caliber = source.to_string();
		self.color = colors.text.to_string();
		
		if false == self.is_secondary {
			if !overrides.ma.ignore {
				self.sanitize_caliber(&source, &overrides);
				self.range.sanitize(&range, &overrides.range_values, RANGE_FONTS, &colors);
				
				if !overrides.range2_values.is_empty() {
					self.range2.sanitize(&"".to_string(), &overrides.range2_values, RANGE_FONTS, &colors);
					self.range2.alternate_location = MOD_LOCATION_ABOVE_MGS.to_string();
				}
					
				let my_rof_ife: String = rof_ife.to_string();
	
				self.rof = sanitize_rof(&my_rof_ife, &overrides.rof, colors);
				self.ife = sanitize_ife(&my_rof_ife, &overrides.ife, colors);
			}
		} else {
			if !source.is_empty() || !overrides.sa.text.is_empty() {
				self.sanitize_caliber(&source, &overrides);
			}
		}
	}
	
	pub fn sanitize_caliber(&mut self, source: &String, overrides: &Overrides) {
		let mut overrides_text = &overrides.ma.text;
		
		if self.is_secondary {
			overrides_text = &overrides.sa.text;
			self.fonts.initialize(SA_FONTS);
		} else {
			self.fonts.initialize(MA_FONTS);
		}

		self.location = "".to_string();
		self.caliber = "".to_string();
		self.overline = false;
		self.underline = false;
		self.caliber_note.text = "".to_string();
		self.velocity = "".to_string();
			
		let mut gun: TextField = self.process_overrides(&source, &overrides_text);
		
		if "VBM NA" == gun.text {
			self.fonts.selected_font = FONT_ALT_NORMAL;
		}
		//
		// FIVE_LOBED_ASTERISK_TAG as a caliber note has to come first to disambiguate it from
		// five-lobed asterisks elsewhere in the gun caliber specification. In particular the
		// UNDERLINED_CALIBER_NOTE_TAG should come after it.
		//
		if gun.text.contains(FIVE_LOBED_ASTERISK_TAG) {
			self.caliber_note.text = FIVE_LOBED_ASTERISK_UC.to_string();				

			if gun.text.starts_with(FIVE_LOBED_ASTERISK_TAG) {
				self.caliber_note.action = NoteAction::Prefix;
			} else {
				self.caliber_note.action = NoteAction::Postfix;
			}

			gun.text = convert_text(&gun.text, &FIVE_LOBED_ASTERISK_TAG, "");
		}

		if gun.text.contains(SIX_LOBED_ASTERISK_TAG) {
			if gun.text.starts_with(SIX_LOBED_ASTERISK_TAG) {
				self.caliber_note.text = SIX_LOBED_ASTERISK_UC.to_string();
				self.caliber_note.action = NoteAction::Prefix;
			} else {
				self.velocity = SIX_LOBED_ASTERISK_UC.to_string();
			}
			
			gun.text = convert_text(&gun.text, &SIX_LOBED_ASTERISK_TAG, "");				
		}

		if gun.text.contains(&UNDERLINED_CALIBER_NOTE_TAG.to_string()) {
			gun.text = convert_text(&gun.text, &UNDERLINED_CALIBER_NOTE_TAG.to_string(), "");
			self.underline_note.initialize(&FIVE_LOBED_ASTERISK_UC.to_string(), NoteAction::Prefix);
		}
		
		if gun.text.contains(FIVE_LOBED_ASTERISK) {
			if gun.text.starts_with(FIVE_LOBED_ASTERISK) {
				self.caliber_note.text = FIVE_LOBED_ASTERISK_UC.to_string();
				self.caliber_note.action = NoteAction::Prefix;
			} else {
				self.velocity = SIX_LOBED_ASTERISK_UC.to_string();
			}
			
			gun.text = convert_text(&gun.text, &FIVE_LOBED_ASTERISK.to_string(), "");				
		}
				
		if gun.text.contains("text-decoration:underline") {
			self.underline = true;
			self.fonts.selected_font = FONT_UNDERLINED;
		}
		
		if gun.text.contains("text-decoration:overline") {
			self.overline = true;
			self.fonts.selected_font = FONT_OVERLINED;
		}
		
		if self.overline && self.underline {
			self.fonts.selected_font = FONT_BOTHLINED;
		}
		
		if gun.text.contains("</span>") {
			let re1 = Regex::new(r"</span>").unwrap();

			gun.text = re1.replace_all(&gun.text, "").to_string();
	
			if gun.text.contains("<span") {
				let re2 = Regex::new(r"<span(.*?)>").unwrap();

				gun.text = re2.replace_all(&gun.text, "").to_string();
			}
		}

		if gun.text.contains("MG") {
			if self.is_secondary {
				gun.text = strip_dagger_and_any_superscript_from_end(&gun.text);
			} else {
				self.fonts.selected_font = FONT_ALT_NORMAL;
			}
		} else if gun.text.contains("ATR") {
			if self.is_secondary {
				gun.text = strip_dagger_and_any_superscript_from_end(&gun.text);
			} else {
				self.fonts.selected_font = FONT_ALT_NORMAL;
			
				if gun.text.starts_with('B') || gun.text.starts_with('T') {
					gun.text.remove(0);
				}
			}
		} else if gun.text.contains("TTACP") {
			self.fonts.selected_font = FONT_ALT_NORMAL;
			gun.text.remove(0);
		} else if gun.text.contains("BF") || gun.text.contains("TF") || gun.text.contains("SF") {
			if gun.text.contains("<b>") {
				gun.text = strip_html_bold(&gun.text);
				self.color = RED.to_string();
			}

			let char_vec: Vec<char> = gun.text.chars().collect();
				
			self.location.push_str(&char_vec[0].to_string());
			self.location.push_str(&char_vec[1].to_string());
			gun.text.remove(0);
			gun.text.remove(0);
		} else if gun.text.starts_with('B') || gun.text.starts_with('T') || gun.text.starts_with('R') {
			if self.is_secondary {
				let char_vec: Vec<char> = gun.text.chars().collect();
				
				self.location.push_str(&char_vec[0].to_string());
			}

			gun.text.remove(0);
		} else if self.is_secondary {
			gun.text = strip_dagger_and_any_superscript_from_end(&gun.text);
		}
		//
		// A dagger means insert a five-lobed asterisk before the main armament.
		// Any entries that shouldn't have this behavior will have to be covered with a main armament override.
		//
		if gun.text.contains(DAGGER) {
			gun.text = strip_dagger_and_any_superscript_from_end(&gun.text);
			self.caliber_note.text = FIVE_LOBED_ASTERISK_UC.to_string();
			self.caliber_note.action = NoteAction::Prefix;
		}
		
		if gun.text.contains("LL") {
			self.caliber = strip_all_occurances(&gun.text, 'L');
			self.velocity = "LL".to_string();
		} else if gun.text.contains("L") {
			self.caliber = strip_all_occurances(&gun.text, 'L');
			self.velocity = "L".to_string();
		} else if gun.text.contains(&FIVE_LOBED_ASTERISK.to_string()) {
			if let Some(result) = gun.text.find(&FIVE_LOBED_ASTERISK.to_string()) {
				if 0 != result {
					self.caliber = strip_all_occurances(&gun.text, '*');
					self.velocity = SIX_LOBED_ASTERISK_UC.to_string();
				}
			}	
		} else {
			self.caliber = gun.text.clone();
		}
	}	

	pub fn process_overrides(&mut self, original: &String, overrides: &String) -> TextField {
		let mut result: TextField = Default::default();
		let entries: Vec<std::string::String> = extract_vector(&overrides, MOD_DELIMITER2);
		let mut value_read = false;
		
		result.text = original.to_string();
		
		if !overrides.is_empty() {
			for entry in &entries {
				if !value_read {
					result.text = convert_text(&entry, COPY_FIELD, &result.text.to_string());
					//
					// If NOVR_MA/NOVR_SA contains "$$" (COPY_FIELD) then we should remove any dagger so it doesn't generate a caliber_note.
					// Any such entry that needs a caliber_note needs to explicitly include it.
					//
					result.text = strip_dagger_and_any_superscript_from_end(&result.text);
					value_read = true;
				} else if entry.contains(MOD_INC_SIZE) {
					let temp: String = extract_from(&entry, MOD_INC_SIZE);
				
					self.fonts.adjust_size(temp.parse::<f64>().unwrap_or(0.0));
				} else if entry.contains(MOD_DEC_SIZE) {
					let temp: String = extract_from(&entry, MOD_DEC_SIZE);
				
					self.fonts.adjust_size(temp.parse::<f64>().unwrap_or(0.0) * -1.0);
				}
			}
		}
		
		return result;
	}
}

pub fn sanitize_rof(source: &String, overrides: &String, colors: &Colors) -> TextField {
	let mut result: TextField = Default::default();
	let rof_overrides: String;
	
	if !overrides.is_empty() {
		let entry = extract_from(&overrides, NOVR_ROF);
		
		result.text = "".to_string();
		rof_overrides = entry.to_string();
	} else if source.contains(SPACE) {
		let (left, _right) = source.split_once(' ').unwrap();
	
		result.text = left.to_string();
		rof_overrides = "".to_string();
	} else {
		result.text = source.to_string();
		rof_overrides = "".to_string();		
	}

	result.sanitize(&source, &rof_overrides, ROF_FONTS, colors);
	
	result.text = strip_dagger_and_any_superscript_from_end(&result.text);
	
	if result.text.contains("<b>") {
		result.text = strip_html_bold(&result.text);
	}	

	return result;
}

pub fn sanitize_ife(source: &String, overrides: &String, colors: &Colors) -> TextField {
	let mut result: TextField = Default::default();
	let mut ife_overrides: String = Default::default();

	if !overrides.is_empty() {
		let entry = extract_from(&overrides, NOVR_IFE);
		
		result.text = "".to_string();
		ife_overrides = entry.to_string();
	} else if source.contains(SPACE) {
		let (_left, right) = source.split_once(' ').unwrap();

		result.text = right.to_string();
		ife_overrides = "".to_string();
	}

	result.sanitize(&source, &ife_overrides, IFE_FONTS, colors);

	result.text = strip_dagger_and_any_superscript_from_end(&result.text);
	
	if result.text.contains("<b>") {
		result.text = strip_html_bold(&result.text);
	}

	result.text = extract_string(&result.text, ")", "(");
	
	return result;
}

pub fn generate_ife_element_on_caliber_line(arm: &Armament) -> std::string::String {
	let font_size = if 1 == arm.ife.text.len() { arm.ife.fonts.size() } else { arm.ife.fonts.size() - 1.2 };
	
	return format!("<tspan style=\"font-size:{0:.2}px\">({1})</tspan></text>", font_size, &arm.ife.text.to_string());
}

pub fn generate_ife_element_alternate_location(mut counter_file: &std::fs::File, arm: &Armament, y_position: f64) -> f64 {
	let mut result: f64 = 0.0;
	let ife = &arm.ife;
	let mut x_pos = GUN_COLUMN_X_POSITION;
	let mut y_pos = y_position;
	let mut anchor = "start".to_string();
	let mut x_percentage = 0.0;
	
	if MOD_LOCATION_GS == ife.alternate_location {
		result = IFE_HEIGHT;
	} else if MOD_LOCATION_ABOVE_MGS == ife.alternate_location {
		x_pos = 21.0; // Magic! 57 - 36;
		y_pos = MGS_LINE_2_Y_POSITION;
		x_percentage = 100.0;
		anchor = "end".to_string();
	} else if MOD_LOCATION_BEFORE_TOWING == ife.alternate_location {
		x_pos = BEFORE_TOWING_X_POSITION;
		y_pos = TOWING_Y_POSITION;
		anchor = "end".to_string();
	}

	let font_size = if 1 == ife.text.len() { ife.fonts.size() } else { ife.fonts.size() - 1.2 };

	generate_svg_start_element(counter_file, 1, x_pos, y_pos - ife.fonts.height(), 36.0, ife.fonts.height(), "IFE", "white"); // Magic!
	write!(counter_file, 	"\t\t<text x=\"{0}%\" y=\"80%\" dominant-baseline=\"auto\" text-anchor=\"{1}\"><tspan style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">({5})</tspan></text>\n", x_percentage, anchor, font_size, FONT_MAIN, ife.color, ife.text).unwrap();
	write!(counter_file, "\t</svg>\n").unwrap();
	
	if MOD_LOCATION_GS == ife.alternate_location {
		result += gun_column_y_gap(&counter_file, x_pos, y_pos - ife.fonts.height(), "lightblue");
	}
	
	return result;
}

pub fn generate_gun_element(arm: &Armament, x_position: f64, y_position: f64) -> std::string::String {
	let mut result = format!("\t\t<text x=\"{0:.2}\" y=\"{1:.2}%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">", x_position, y_position, arm.fonts.size(), &FONT_MAIN.to_string(), &arm.color);
	
	if !arm.location.is_empty() {
		result.push_str(&arm.location.to_string());
	}
	
	if arm.underline {
		result.push_str("<tspan style=\"text-decoration:underline\">");
	}
	
	if arm.overline {
		result.push_str("<tspan style=\"text-decoration:overline\">");
	}
	
	result.push_str(&arm.caliber.to_string());

	if arm.underline {
		result.push_str("</tspan>");
	}
	
	if arm.overline {
		result.push_str("</tspan>");
	}

	if !arm.velocity.is_empty() {
		if arm.velocity.contains(&SIX_LOBED_ASTERISK_UC.to_string()) {
			result.push_str(&generate_six_lobed_asterisk_svg(&arm.fonts));
		} else {
			result.push_str(&arm.velocity.to_string());
		}
	}

	if !arm.caliber_note.text.is_empty() && NoteAction::Postfix == arm.caliber_note.action {
		//
		// TODO: Keep separate (for now?) in case we need to treat the different asterisks differently.
		//
		result.push_str("<tspan>");
		
		if arm.caliber_note.text.contains(SIX_LOBED_ASTERISK_UC) {
			result.push_str(&arm.caliber_note.text);
		} else if arm.caliber_note.text.contains(FIVE_LOBED_ASTERISK_UC) {
			result.push_str(&arm.caliber_note.text);
		}

		result.push_str("</tspan>");
	}
	
	result.push_str("</text>");

	return result;
}

pub fn generate_gun_elements(mut counter_file: &std::fs::File, ma: &Armament, overrides: &Overrides, y_position: f64) -> f64 {
	let mut x_position = 0.0;
	let mut baseline: f64 = ma.fonts.y_percentage();

	generate_svg_start_element(counter_file, 1, GUN_COLUMN_X_POSITION, y_position - ma.fonts.height(), 54.0, ma.fonts.height(), "MA, text position adjusts to account for any overlined/underlined text.", "white");
	
	if !ma.caliber_note.text.is_empty() && NoteAction::Prefix == ma.caliber_note.action {
		if ma.caliber_note.text.contains(SIX_LOBED_ASTERISK_UC) {
			write!(counter_file, "\t\t<text x=\"{0:.2}\" y=\"{1:.2}%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\" baseline-shift=\"super\">{5}</text>\n", x_position, baseline, ma.fonts.sup_size(), FONT_ALT, ma.color, ma.caliber_note.text).unwrap();
			x_position = 5.4; // Magic!
		} else {
			write!(counter_file, "\t\t<text x=\"{0:.2}\" y=\"{1:.2}%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">{5}</text>\n", x_position, baseline, ma.fonts.size(), FONT_ALT, ma.color, ma.caliber_note.text).unwrap();
			x_position = 4.8; // Magic!
		}
		
		if ma.fonts.selected_font != FONT_NORMAL {
			x_position -= 0.6; // Magic!
		}
	} else if NoteAction::Prefix == ma.underline_note.action {
		x_position = 4.8;
	}
	
	let mut gun = generate_gun_element(ma, x_position, baseline);
	
	if overrides.ma.moving_target_penalty { // TODO: Add to Armament struct and copy in Sanitize() method?
		let mut temp_gun: String = gun.to_string();
		let foo: String = generate_six_lobed_asterisk_svg(&ma.fonts);

		temp_gun = convert_text(&temp_gun, "fill:black", "fill:none");

		if temp_gun.contains(&foo) {
			temp_gun = convert_text(&temp_gun, &foo, "");
		}		

		temp_gun = convert_text(&temp_gun, "</text>", MA_MOVT_WHITE_CIRCLE);
		write!(counter_file, "{0}\n", temp_gun).unwrap();
	}

	if !ma.is_secondary && !ma.ife.text.is_empty() && ma.ife.alternate_location.is_empty() {
		let ife = generate_ife_element_on_caliber_line(&ma);
		
		gun = convert_text(&gun, "</text>", &ife);
	}
	
	write!(counter_file, "{0}\n", gun).unwrap();
	//
	// TODO: for now, sophisticate it later?
	//
	if NoteAction::Prefix == ma.underline_note.action {
		baseline = 112.00; // Magic!
		x_position = 0.0;

		write!(counter_file, "\t\t<text x=\"{0:.2}\" y=\"{1:.2}%\" dominant-baseline=\"auto\" text-anchor=\"start\"><tspan style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">*</tspan></text> <!-- Note on underline -->\n", x_position, baseline, ma.fonts.size(), FONT_MAIN, ma.color).unwrap();
	}

	write!(counter_file, "\t</svg>\n").unwrap();
	
	return ma.fonts.height();
}
//
// For Vehicle counters and Ordnance counters with a second range value.
//
pub fn generate_range_element(mut counter_file: &std::fs::File, range: &TextField, no_mgs: bool, y_position: f64, comment: &str) -> f64 {
	let mut result = 0.0; 

	if !range.text.is_empty() {
		//
		// Our default position is the same as where the machine guns would be displayed. If there's a conflict
		// then the "alt_range" override will have to be invoked to move us into the "gun stack" or above the MGs.
		//
		let mut x_pos: f64 = MGS_LINE_X_POSITION;
		let mut y_pos: f64 = RANGE_LINE_Y_POSITION;
		let mut anchor: String = "end".to_string();
		let mut x_percentage = 100;

		if MOD_LOCATION_ABOVE_MGS == range.alternate_location {
			y_pos = MGS_LINE_2_Y_POSITION;
		} else if MOD_LOCATION_GS == range.alternate_location || !no_mgs { // TODO Double negative, yuck!
			x_pos = GUN_COLUMN_X_POSITION;
			result = range.fonts.height();
			y_pos = y_position;
			anchor = "start".to_string();
			x_percentage = 0;
		}

		generate_svg_start_element(counter_file, 1, x_pos, y_pos - range.fonts.height(), 36.0, range.fonts.height(), comment, "white"); // Magic!
		write!(counter_file, "\t\t<text x=\"{0}%\" y=\"80%\" dominant-baseline=\"auto\" text-anchor=\"{1}\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">{5}</text>\n", x_percentage, anchor, range.fonts.size(), FONT_MAIN, range.color, range.text).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
		
		if MOD_LOCATION_GS == range.alternate_location || !no_mgs { // TODO Double negative, yuck!
			result += gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, y_pos - range.fonts.height(), "cyan");
		}
	}

	return result;
}
//
// For Ordnance counters.
//
pub fn generate_range_and_special_ammunition_elements(mut counter_file: &std::fs::File, ma: &Armament) {
	//
	// Our default position is the same as where the machine guns would be displayed on a vehicle counter.
	// If there's a conflict then the "alt_range" override will have to be invoked to move us above the default position.
	//
	let x_pos: f64 = 57.0;
	let mut y_pos: f64 = 55.5;
	let mut prefix_note: String = Default::default();
	let mut postfix_note: String = Default::default();

	if NoteAction::Prefix == ma.range.note.action {
		prefix_note = ma.range.note.text.to_string();
	} else if NoteAction::Postfix == ma.range.note.action {
		postfix_note = ma.range.note.text.to_string();
	}

	if !ma.range.text.is_empty() {
		if MOD_LOCATION_ABOVE_MGS == ma.range.alternate_location {
			y_pos = 46.5;
		}
	
		write!(counter_file, "\t<!-- Range -->\n").unwrap();
	
		if ma.range.text.contains('[') {
			write!(counter_file, "\t<text x=\"{0:.2}\" y=\"{1:.2}\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">{5}</text>\n", x_pos, y_pos, ma.range.fonts.size(), FONT_MAIN, ma.range.color, ma.range.text).unwrap();
		} else {
			write!(counter_file, "\t<text x=\"{0:.2}\" y=\"{1:.2}\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">{5}[{6}]{7}</text>\n", x_pos, y_pos, ma.range.fonts.size(), FONT_MAIN, ma.range.color, prefix_note, ma.range.text, postfix_note).unwrap();
		}
	}

	if !ma.special_ammo.text.is_empty() {
		if MOD_LOCATION_ABOVE_MGS == ma.special_ammo.alternate_location {
			y_pos = 46.5;
		}

		write!(counter_file, "\t<!-- Special Ammunition -->\n").unwrap();
		write!(counter_file, "\t<text x=\"{0:.2}\" y=\"{1:.2}\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">{5}</text>\n", x_pos, y_pos, ma.special_ammo.fonts.size(), FONT_MAIN, ma.special_ammo.color, ma.special_ammo.text).unwrap();
	}
}

/* Do not need get_text_width() at this time, keep code around in case we need it later.
pub fn calculate_gun_field_length(arm: &Armament, ctc: &mut CosmicTextContext) -> f64 {
	let mut result: f64 = 0;

	if !arm.caliber.is_empty() {
		let mut gun_spec: String = Default::default();
		
		if !arm.caliber_note.is_empty() {
			gun_spec.push_str(&FIVE_LOBED_ASTERISK.to_string()); // Not really, but we just need a reasonable width.
		}
		
		gun_spec.push_str(&arm.caliber);
		gun_spec.push_str(&arm.velocity);
		
		let temp = get_text_width(ctc, &gun_spec, arm.fonts.size(), Some(&FONT_MAIN.to_string())).expect("REASON");
		result = temp as f64;
	}

	return result;
}
Do not need get_text_width() at this time, keep code around in case we need it later. */

pub fn generate_gun_caliber_line(counter_file: &std::fs::File, record: &CommonRecord) -> f64 {
	let mut y_position: f64 = GUN_CALIBER_BASELINE;
	
	if !record.ma.caliber.is_empty() {
		y_position -= generate_gun_elements(&counter_file, &record.ma, &record.overrides, y_position);
		y_position -= gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, y_position, "red");
	}
	
	return y_position;
}

pub fn generate_rof_element(mut counter_file: &std::fs::File, rof: &TextField, y_position: f64, color: &String, multiple_hits: bool) -> f64 {
	let mut result = ROF_HEIGHT;
	
	generate_svg_start_element(counter_file, 1, GUN_COLUMN_X_POSITION, y_position - ROF_HEIGHT, ROF_HEIGHT, ROF_HEIGHT, "ROF", "white");

	if multiple_hits {
		write!(counter_file, "\t\t<circle cx=\"50%\" cy=\"50%\" r=\"4.8\" style=\"display:inline;fill:white;fill-opacity:1;stroke:none;stroke-width:0.00px;stroke-dasharray:none;stroke-opacity:1\"></circle>\n").unwrap();
	}

	if NoteAction::Prefix == rof.note.action {
		write!(counter_file, "\t\t<text x=\"50%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">{5}{6}</tspan></text>\n", rof.fonts.y_percentage(), rof.fonts.size(), FONT_MAIN, FONT_MAIN, color, rof.note.text, rof.text).unwrap();
	} else if NoteAction::Postfix == rof.note.action {
		write!(counter_file, "\t\t<text x=\"50%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">{5}{6}</tspan></text>\n", rof.fonts.y_percentage(), rof.fonts.size(), FONT_MAIN, FONT_MAIN, color, rof.text, rof.note.text).unwrap();
	} else {
		write!(counter_file, "\t\t<text x=\"50%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">{5}</tspan></text>\n", rof.fonts.y_percentage(), rof.fonts.size(), FONT_MAIN, FONT_MAIN, color, rof.text).unwrap();
	}

	write!(counter_file, "\t\t<rect x=\"{0:.2}\" y=\"{0:.2}\" width=\"{1:.2}\" height=\"{1:.2}\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{2};stroke-width:{3:.2}px;stroke-dasharray:none;stroke-opacity:1\"/>\n", ROF_OFFSET, ROF_BOX_SIZE, color, ROF_STROKE_WIDTH).unwrap();
	write!(counter_file, "\t</svg>\n").unwrap();
	
	result += gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, y_position - result, "blue");
	
	return result;
}

pub fn generate_sa_elements(mut counter_file: &std::fs::File, sa: &Armament, overrides: &Overrides, y_position: f64) -> f64 {
	let mut result = sa.fonts.height();
	let baseline: f64 = sa.fonts.y_percentage();
	let mut x_pos = 0.0;

	generate_svg_start_element(counter_file, 1, GUN_COLUMN_X_POSITION, y_position - sa.fonts.height(), 36.0, sa.fonts.height(), "SA, text position adjusts to account for any overlined/underlined text.", "white");

	if !sa.caliber_note.text.is_empty() {
		if sa.caliber_note.text.contains(SIX_LOBED_ASTERISK_UC) {
			write!(counter_file, "\t\t<text x=\"{0:.2}\" y=\"{1:.2}%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\" baseline-shift=\"super\">{5}</text>\n", x_pos, baseline, sa.fonts.sup_size(), FONT_ALT, sa.color, sa.caliber_note.text).unwrap();
			x_pos = 3.6; // Magic!
		} else {
			write!(counter_file, "\t\t<text x=\"{0:.2}\" y=\"{1:.2}%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">{5}</text>\n", x_pos, baseline, sa.fonts.size(), FONT_ALT, sa.color, sa.caliber_note.text).unwrap();
			x_pos = 3.0; // Magic!
		}
	}
	
	let mut gun = generate_gun_element(sa, x_pos, baseline);
	
	if gun.contains("<sup>") {
		let super_font_size = sa.fonts.size() - 2.4;
		gun = convert_superscripts(&gun, super_font_size);
	}

	if overrides.sa.moving_target_penalty {
		let mut temp_gun: String = gun.to_string();
		let foo: String = generate_six_lobed_asterisk_svg(&sa.fonts);

		temp_gun = convert_text(&temp_gun, "fill:black", "fill:none");

		if temp_gun.contains(&foo) {
			temp_gun = convert_text(&temp_gun, &foo, "");
		}		

		temp_gun = convert_text(&temp_gun, "</text>", SA_MOVT_WHITE_CIRCLE);
		write!(counter_file, "{0}\n", temp_gun).unwrap();
	}

	write!(counter_file, "{0}\n", gun).unwrap();
	write!(counter_file, "\t</svg>\n").unwrap();

	result += gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, y_position - result, "orange");
	
	return result;
}

pub fn sanitize_gun_type(source: &String, overrides: &Overrides, colors: &Colors) -> TextField {
	let mut result: TextField = Default::default();

	if overrides.ma.ignore {
		result.text = "".to_string();
	} else {
		result.text = source.to_string();
	}

	result.color = colors.text.to_string();

	return result;
}

pub fn sanitize_mount(source: &String, overrides: &Overrides, colors: &Colors) -> Turret {
	let mut result: Turret = Default::default();

	if !overrides.ma.ignore {
		result.speed = extract_turret_type(&source, &overrides);
		result.color = colors.turret_type.to_string();
	}

	return result;
}
