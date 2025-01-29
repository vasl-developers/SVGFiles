use std::write;
use std::io::prelude::*;
// Local files.
//
use crate::colors::*;
use crate::defines::*;
use crate::overrides::*;
use crate::text_field::*;
use crate::utils::*;

pub const TRANSPORT_FONTS: [[f64; 4]; 8] = [
	[   8.0,   4.0,  98.00,   6.0 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   7.5,   4.0,  73.00,   8.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  98.00,   8.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  80.00,  10.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,    0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,    0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,    0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,    0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

#[derive(PartialEq)]
#[derive(Default)]
pub struct TransportValues {
	pub pp: TextField,
	pub towing: TextField,
	pub manhandling_number: TextField,
}

impl TransportValues {
	pub fn sanitize(&mut self, source: &String, overrides: &Overrides, colors: &Colors) {
		let mut temp: String = source.to_string();
	
		temp = strip_daggered_note(&temp);
		temp = convert_superscripts(&temp, TRANSPORT_SUPERSCRIPT_FONT_SIZE);
		temp = mask_closing_html_tags(&temp);
	
		let factors = temp.split('/');
	
		for factor in factors {
			if !factor.is_empty() {
				let value = unmask_closing_html_tags(&factor.to_string());
				
				if factor.contains("PP") {
					self.pp.is_set = true;
					self.pp.text = value;
					self.pp.color = colors.text.to_string();
					self.pp.fonts.initialize(TRANSPORT_FONTS);
				} else if factor.contains("T")  {
					self.towing.is_set = true;
					self.towing.text = value;
					self.towing.color = colors.text.to_string();
					self.towing.fonts.initialize(TRANSPORT_FONTS);
				} else if factor.contains("M") { // Thanks for nothing Nimbus!
					self.manhandling_number.is_set = true;
					self.manhandling_number.text = value[1..].to_string();
					self.manhandling_number.color = colors.text.to_string();
					self.manhandling_number.fonts.initialize(TRANSPORT_FONTS);
				}
			}
		}
		//
		// Now handle overrides (if any).
		//
		if overrides.pp_number_ignore {
			self.pp.is_set = false;
			self.pp.text = "".to_string();
		} else if !overrides.pp_number.is_empty() {
			self.pp.process_overrides(&overrides.pp_number, PROCESS_ASTERISK_TAGS);
			self.pp.color = colors.text.to_string();
			self.pp.fonts.initialize(TRANSPORT_FONTS);
		}
	
		if !overrides.towing_number.is_empty() {
			self.towing.process_overrides(&overrides.towing_number, PROCESS_ASTERISK_TAGS);
			self.towing.color = colors.text.to_string();
			self.towing.fonts.initialize(TRANSPORT_FONTS);
		}
		
		if !overrides.manhandling.is_empty() {
			self.manhandling_number.process_overrides(&overrides.manhandling, PROCESS_ASTERISK_TAGS);
			self.manhandling_number.color = colors.text.to_string();
			self.manhandling_number.fonts.initialize(TRANSPORT_FONTS);
		}
	}
	
	pub fn lc_sanitize(&mut self, source: &String, ramp: bool, overrides: &Overrides, colors: &Colors) {
		let mut temp: String = source.to_string();
	
		if temp.contains(DAGGER) {
			self.pp.note.action = NoteAction::Postfix;
			self.pp.note.text = FIVE_LOBED_ASTERISK_SVG.to_string();
		}
		
		temp = strip_daggered_note(&temp);
		temp = convert_superscripts(&temp, TRANSPORT_SUPERSCRIPT_FONT_SIZE);
		temp = mask_closing_html_tags(&temp);
	
		if !temp.is_empty() {
			self.pp.is_set = true;
			self.pp.text = temp;		
			self.pp.color = colors.text.to_string();
			self.pp.fonts.initialize(TRANSPORT_FONTS);
			
			if ramp {
				self.pp.fonts.selected_font = FONT_UNDERLINED;
			}
		}
		//
		// Now handle overrides (if any).
		//
		if overrides.pp_number_ignore {
			self.pp.is_set = false;
			self.pp.text = "".to_string();
		} else if !overrides.pp_number.is_empty() {
			temp = overrides.pp_number.clone();
			self.pp.color = colors.text.to_string();
			self.pp.fonts.initialize(TRANSPORT_FONTS);
			self.pp.note.action = NoteAction::None;

			if overrides.pp_number.contains(FIVE_LOBED_ASTERISK_TAG) {
				self.pp.note.action = NoteAction::Postfix;
				self.pp.note.text = FIVE_LOBED_ASTERISK_SVG.to_string();			
				temp = temp.replace(FIVE_LOBED_ASTERISK_TAG, "");
			}
			
			self.pp.process_overrides(&temp, PROCESS_ASTERISK_TAGS);
		}
	}

	pub fn gl_sanitize(&mut self, source: &String, overrides: &Overrides, colors: &Colors) {
		let mut temp: String = source.to_string();
	
		if temp.contains(DAGGER) {
			self.pp.note.action = NoteAction::Postfix;
			self.pp.note.text = FIVE_LOBED_ASTERISK_SVG.to_string();
		}
		
		temp = strip_daggered_note(&temp);
		temp = convert_superscripts(&temp, TRANSPORT_SUPERSCRIPT_FONT_SIZE);
		temp = mask_closing_html_tags(&temp);
	
		if !temp.is_empty() {
			self.pp.is_set = true;
			self.pp.text = temp;		
			self.pp.color = colors.text.to_string();
			self.pp.fonts.initialize(TRANSPORT_FONTS);
		}
		//
		// Now handle overrides (if any).
		//
		if overrides.pp_number_ignore {
			self.pp.is_set = false;
			self.pp.text = "".to_string();
		} else if !overrides.pp_number.is_empty() {
			temp = overrides.pp_number.clone();
			self.pp.color = colors.text.to_string();
			self.pp.fonts.initialize(TRANSPORT_FONTS);
			self.pp.note.action = NoteAction::None;

			if overrides.pp_number.contains(FIVE_LOBED_ASTERISK_TAG) {
				self.pp.note.action = NoteAction::Postfix;
				self.pp.note.text = FIVE_LOBED_ASTERISK_SVG.to_string();			
				temp = temp.replace(FIVE_LOBED_ASTERISK_TAG, "");
			}
			
			self.pp.process_overrides(&temp, PROCESS_ASTERISK_TAGS);
		}
	}	
}

pub fn generate_pp_number_element(mut counter_file: &std::fs::File, pp: &TextField, x_position: f64, y_position: f64, anchor: &String) -> f64
{
	let mut result: f64 = 0.0;
	let x_pos = if "end" == anchor { "100" } else { "0" };

	if !pp.text.is_empty() {
		let mut prefix_note: String = Default::default();
		let mut postfix_note: String = Default::default();

		if NoteAction::Prefix == pp.note.action {
			prefix_note = pp.note.text.to_string();
		} else if NoteAction::Postfix == pp.note.action {
			postfix_note = pp.note.text.to_string();
		}

		generate_svg_start_element(counter_file, 1, x_position, y_position - pp.fonts.height(), 36.0, pp.fonts.height(), "PP #", &"white".to_string()); // Magic!
		write!(counter_file, "\t\t<text x=\"{0}%\" y=\"{1}%\" dominant-baseline=\"auto\" text-anchor=\"{2}\"><tspan style=\"font-size:{3:.2}px;{FONT_WEIGHT_BOLD};font-family:{4};fill:{5};fill-opacity:1;stroke-width:0.2\">{6}{7}{8}</tspan></text>\n", x_pos, pp.fonts.y_percentage(), anchor, pp.fonts.size(), FONT_MAIN, pp.color, prefix_note, pp.text, postfix_note).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();

		if pp.alternate_location.is_empty() {
			result = pp.fonts.height() + gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, y_position - pp.fonts.height(), "yellow");
		}
	}

	return result;
}

pub fn lc_generate_pp_svg_elements(mut counter_file: &std::fs::File, pp: &TextField, x_position: f64, y_position: f64, anchor: &String, ramp: bool) -> f64
{
	let mut result: f64 = 0.0;

	if !pp.text.is_empty() {
		let mut prefix_note: String = Default::default();
		let mut postfix_note: String = Default::default();

		if NoteAction::Prefix == pp.note.action {
			prefix_note = pp.note.text.to_string();
		} else if NoteAction::Postfix == pp.note.action {
			postfix_note = pp.note.text.to_string();
		}

		generate_svg_start_element(counter_file, 1, x_position, y_position - pp.fonts.height(), 36.0, pp.fonts.height(), "PP #", &"white".to_string()); // Magic!
		
		if ramp {
			write!(counter_file, "\t\t<text x=\"0.00\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"{1}\"><tspan style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">{5}{6}<tspan style=\"text-decoration:underline\">PP</tspan>{7}</tspan></text>\n", pp.fonts.y_percentage(), anchor, pp.fonts.size(), FONT_MAIN, pp.color, prefix_note, pp.text, postfix_note).unwrap();
		} else {
			write!(counter_file, "\t\t<text x=\"0.00\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"{1}\"><tspan style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">{5}{6}PP{7}</tspan></text>\n", pp.fonts.y_percentage(), anchor, pp.fonts.size(), FONT_MAIN, pp.color, prefix_note, pp.text, postfix_note).unwrap();
		}
		
		write!(counter_file, "\t</svg>\n").unwrap();

		if pp.alternate_location.is_empty() {
			result = pp.fonts.height() + gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, y_position - pp.fonts.height(), "yellow");
		}
	}

	return result;
}

pub fn gl_generate_pp_svg_elements(mut counter_file: &std::fs::File, pp: &TextField, x_position: f64, y_position: f64, anchor: &String)
{
	if !pp.text.is_empty() {
		let mut prefix_note: String = Default::default();
		let mut postfix_note: String = Default::default();

		if NoteAction::Prefix == pp.note.action {
			prefix_note = pp.note.text.to_string();
		} else if NoteAction::Postfix == pp.note.action {
			postfix_note = pp.note.text.to_string();
		}

		generate_svg_start_element(counter_file, 1, x_position, y_position - pp.fonts.height(), 36.0, pp.fonts.height(), "PP #", &"white".to_string()); // Magic!
		
		write!(counter_file, "\t\t<text x=\"0.00\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"{1}\"><tspan style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">{5}{6}PP{7}</tspan></text>\n", pp.fonts.y_percentage(), anchor, pp.fonts.size(), FONT_MAIN, pp.color, prefix_note, pp.text, postfix_note).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}
}

pub fn generate_towing_number_element(mut counter_file: &std::fs::File, towing: &TextField, x_position: f64, y_position: f64, anchor: &String)
{
	if !towing.text.is_empty() {
		let mut x_pos = 0.0;

		if "end" == anchor {
			x_pos = 24.0;
		}

		let mut prefix_note: String = Default::default();
		let mut postfix_note: String = Default::default();

		if NoteAction::Prefix == towing.note.action {
			prefix_note = towing.note.text.to_string();
		} else if NoteAction::Postfix == towing.note.action {
			postfix_note = towing.note.text.to_string();
		}

		generate_svg_start_element(counter_file, 1, x_position, y_position - towing.fonts.height(), 24.0, towing.fonts.height(), "Towing #", &"white".to_string()); // Magic!
		write!(counter_file, "\t\t<text x=\"{0:.2}\" y=\"{1}%\" dominant-baseline=\"auto\" text-anchor=\"{2}\"><tspan style=\"font-size:{3:.2}px;{FONT_WEIGHT_BOLD};font-family:{4};fill:{5};fill-opacity:1;stroke-width:0.2\">{6}{7}{8}</tspan></text>\n", x_pos, towing.fonts.y_percentage(), anchor, towing.fonts.size(), FONT_MAIN, towing.color, prefix_note, towing.text, postfix_note).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}
}
