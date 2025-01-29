use std::write;
use std::io::prelude::*;
// Local files.
//
use crate::colors::*;
use crate::defines::*;
use crate::overrides::*;
use crate::text_field::*;
use crate::utils::*;

pub const MGS_FONTS: [[f64; 4]; 8] = [
	[   8.0,   6.0,  80.0,   6.6 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const MGS_FONT_WEIGHT: &str =	"bold";

pub const MGS_SIX_LOBED_ASTERISK_FONT_SIZE: f64 =		5.0;
pub const MGS_SIX_LOBED_ASTERISK_FONT_SIZE_STR: &str =	"5.0";

pub const FIXED_BMG_WHITE_CIRCLE: &str = "<tspan style=\"fill:white;{FONT_WEIGHT_MAIN}\">&#x2B24;</tspan>";

#[derive(PartialEq)]
#[derive(Default)]
pub struct MachineGuns {
	pub field: TextField,
	pub fixed_bmg: bool,
}

impl MachineGuns {
	pub fn sanitize(&mut self, bmg: &String, cmg: &String, aamg: &String, overrides: &Overrides, colors: &Colors) {
		self.field.color = colors.text.to_string();
		self.field.fonts.initialize(MGS_FONTS);
		
		if overrides.machine_guns_set {
			let entry = overrides.machine_guns.clone();
			let entries: Vec<std::string::String> = extract_vector(&entry, MOD_DELIMITER2);
			let mut mgs_value_read: bool = false;
			
			for entry in &entries {
				if !mgs_value_read {
					self.field.text = entry.to_string(); // COPY_FIELD not needed!
					mgs_value_read = true;
				} else if entry.contains(MOD_INC_SIZE) {
					let temp: String = extract_from(&entry, MOD_INC_SIZE);
				
					self.field.fonts.adjust_size(temp.parse::<f64>().unwrap_or(0.0));
				} else if entry.contains(MOD_DEC_SIZE) {
					let temp: String = extract_from(&entry, MOD_DEC_SIZE);
					
					self.field.fonts.adjust_size(temp.parse::<f64>().unwrap_or(0.0) * -1.0);
				} else if is_alternate_location(entry.to_string()) {
					self.field.alternate_location = entry.to_string();
				} else {
					panic!("sanitize MGs@{0}: unimplemented! entry '{1}'", line!(), entry);
				}
			}			
		} else {
			let mut my_bmg: String = strip_opt(&bmg);
			let mut my_cmg: String = strip_opt(&cmg);
			let mut my_aamg: String = strip_opt(&aamg);

			if my_bmg.contains(DAGGER) {
				my_bmg = strip_dagger_and_any_superscript_from_end(&my_bmg);

				if !overrides.fixed_bmg {
					my_bmg.push_str(&FIVE_LOBED_ASTERISK_UC.to_string());
				}
			}

			if my_cmg.contains(DAGGER) {
				my_cmg = strip_dagger_and_any_superscript_from_end(&my_cmg);
				my_cmg.push_str(&FIVE_LOBED_ASTERISK_UC.to_string());
			}

			if my_aamg.contains(DAGGER) {
				my_aamg = strip_dagger_and_any_superscript_from_end(&my_aamg);
				my_aamg.push_str(&FIVE_LOBED_ASTERISK_UC.to_string());
			}

			if !my_bmg.is_empty() || !my_cmg.is_empty() || !my_aamg.is_empty() {
				if !my_bmg.is_empty() {
					self.field.text.push_str(&my_bmg);
				} else {
					self.field.text.push_str("-");
				}

				self.field.text.push_str("/");
				if !my_cmg.is_empty() {
					self.field.text.push_str(&my_cmg);
				} else {
					self.field.text.push_str("-");
				}

				if !my_aamg.is_empty() {
					self.field.text.push_str("/");
					self.field.text.push_str(&my_aamg);
				}
			}
		}

		self.field.text = convert_text(&self.field.text, FIVE_LOBED_ASTERISK_TAG, FIVE_LOBED_ASTERISK_UC);
		self.field.text = convert_text(&self.field.text, SIX_LOBED_ASTERISK_TAG, SIX_LOBED_ASTERISK_MG_SVG);

		if self.field.text.contains("<sup>") {
			self.field.text = convert_superscripts(&self.field.text, MGS_SUPERSCRIPT_FONT_SIZE);
		}
			
		self.fixed_bmg = overrides.fixed_bmg;
	}	

	pub fn generate_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		if !self.field.text.is_empty() {
			let x_position = MGS_LINE_X_POSITION;
			let y_position = MGS_LINE_Y_POSITION - self.field.fonts.height() - 1.00; // "3.0" to line the bottom of the MG text with the bottom of the MA text and still allow for descenders.
		
			generate_svg_start_element(counter_file, 1, x_position, y_position, MGS_WIDTH, self.field.fonts.height() + 3.0 /* magic! */, "Machine guns", &"lightgreen".to_string());

			if self.fixed_bmg {
				let mut temp = self.field.text.clone();
				
				temp.remove(0);
				
				let mut fixed_bmg = FIXED_BMG_WHITE_CIRCLE.to_string();
				fixed_bmg.push_str(&temp);
				
				write!(counter_file, "\t\t<text x=\"100%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"end\"><tspan style=\"font-size:{1:.2}px;font-weight:{2};font-family:{3};fill:none;fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", self.field.fonts.y_percentage(), self.field.fonts.size(), MGS_FONT_WEIGHT, FONT_MAIN, fixed_bmg).unwrap();
			}

			write!(counter_file, "\t\t<text x=\"100%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"end\"><tspan style=\"font-size:{1:.2}px;font-weight:{2};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">{5}</tspan></text>\n", self.field.fonts.y_percentage(), self.field.fonts.size(), MGS_FONT_WEIGHT, FONT_MAIN, self.field.color, self.field.text).unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();
		}
	}
}
