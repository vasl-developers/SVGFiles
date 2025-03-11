use const_format::formatcp;
use std::fmt;
//
// Local defines.
//
use crate::colors::*;
use crate::overrides::*;
use crate::utils::*;
//
// Fonts.
//
pub const FONT_MAIN: &str =	"Nimbus Sans L";
pub const FONT_ALT: &str =	"Nimbus Sans L";

pub const FONT_WEIGHT_BOLD: &str =	"font-weight:bold";
pub const FONT_WEIGHT_NORM: &str =	"font-weight:normal";

pub const FONT_NORMAL: usize =			0;
pub const FONT_UNDERLINED: usize =		1;
pub const FONT_OVERLINED: usize =		2;
pub const FONT_BOTHLINED: usize = 		3;
pub const FONT_ALT_NORMAL: usize =		4;
pub const FONT_ALT_UNDERLINED: usize =	5;
pub const FONT_ALT_OVERLINED: usize =	6;
pub const FONT_ALT_BOTHLINED: usize = 	7;

pub const FA_SIZE: usize =			0;	// Font size in pixels.
pub const FA_SUP_SIZE: usize =		1;	// Superscript font size in pixels.
pub const FA_Y_PERCENTAGE: usize =	2;	// Y value of baseline within the SVG bounding box (as percentage).
pub const FA_HEIGHT: usize = 		3;	// Height used for calculating top of this element in the "gun stack."
//
// Font sizes in pixels.
//
pub const RANGE_FONTS: [[f64; 4]; 8] = [
	[   8.4,   6.0,   0.0,   7.8 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

#[derive(PartialEq, Default, Clone, Copy)]
pub struct FontsObj {
	pub selected_font: usize,
	pub fonts: [[f64; 4]; 8],
}

impl FontsObj {
	pub fn copy(&mut self, source: &FontsObj) {
		self.selected_font = source.selected_font;
		self.fonts = source.fonts;
	}
	
	pub fn initialize(&mut self, source: [[f64; 4]; 8]) {
		self.selected_font = FONT_NORMAL;
		self.fonts = source;
	}

	pub fn size(self) -> f64 {
		return self.fonts[self.selected_font][FA_SIZE];
	}
	
	pub fn sup_size(self) -> f64 {
		return self.fonts[self.selected_font][FA_SUP_SIZE];
	}
	
	pub fn y_percentage(self) -> f64 {
		return self.fonts[self.selected_font][FA_Y_PERCENTAGE];
	}
	
	pub fn height(self) -> f64 {
		return self.fonts[self.selected_font][FA_HEIGHT];
	}

	pub fn adjust_size(&mut self, delta: f64) {
		self.adjust(delta, FA_SIZE);
	}
	
	pub fn adjust_sup_size(&mut self, delta: f64) {
		self.adjust(delta, FA_SUP_SIZE);
	}
	
	pub fn adjust_y_percentage(&mut self, delta: f64) {
		self.adjust(delta, FA_Y_PERCENTAGE);
	}
	
	pub fn adjust_height(&mut self, delta: f64) {
		self.adjust(delta, FA_HEIGHT);
	}
	
	fn adjust(&mut self, delta: f64, col: usize) {
		let mut idx = FONT_NORMAL;
		
		while idx <= FONT_ALT_BOTHLINED {
			if delta.abs() < self.fonts[idx][col] || delta > 0.0 {
				self.fonts[idx][col] = self.fonts[idx][col] + delta;
			}
			
			idx += 1;
		}
	}
}
//
// Old busted font data.
//
pub const FIXED_BMG_FONT_SIZE: f64 =				 14.4;
pub const MOTORCYCLE_FONT_SIZE: f64 =				  9.0;
pub const MGS_SIX_LOBED_ASTERISK_FONT_SIZE: f64 =	  4.2;
pub const RANGE_FONT_SIZE: f64 =					  8.4;
pub const SPECIAL_AMMO_FONT_SIZE: f64 =				  9.0;
pub const SPECIAL_AMMO_SUPERSCRIPT_FONT_SIZE: f64 =	  6.0;
pub const BREAKDOWN_NOTE_FONT_POS_DELTA: f64 =		  1.2;
pub const LOW_AMMO_BREAKDOWN_FONT_NEG_DELTA: f64 =	  1.2;
pub const TRANSPORT_SUPERSCRIPT_FONT_SIZE: f64 =	  4.8;
pub const NAME_FONT_SIZE: f64 =						  6.0;
pub const SIX_LOBED_ASTERISK_FONT_SIZE: f64 =		  4.8;

pub const MGS_SIX_LOBED_ASTERISK_FONT_SIZE_STR: &str =	"4.2";	// formatcp!() doesn't handle floats.
pub const SIX_LOBED_ASTERISK_FONT_SIZE_STR: &str =		"4.8";	// formatcp!() doesn't handle floats.

pub const DAGGER: char = 					'†';
pub const STAR: char = 						'★';
pub const CIRCLE: char = 					'●';
pub const FIVE_LOBED_ASTERISK: char =		'*';
pub const FIVE_LOBED_ASTERISK_UC: &str =	"&#x002A;";
pub const SIX_LOBED_ASTERISK: char =		'✽';
pub const SIX_LOBED_ASTERISK_UC: &str =		"&#x273D;";
pub const COLON: char =						':';
pub const COMMA: char =						',';
pub const SPACE: char =						' ';
pub const SUPER: char =						'^';
pub const MINUS: char =						'-';
pub const EQUALS: char =					'=';
pub const UNDERLINE: char =					'_';
pub const RAMP_DOT: char =					'•';
//
// Copy the appropriate original data scrubbed of Daggers (and their superscripts if present).
// Used as part of the override/limber system.
//
pub const COPY_FIELD: &str =					"$$";
//
// Tag substitutions.
//
pub const BLACK_ASTERISK_TAG: &str =					"[b*]";	// Force the asterisk to be displayed in black, not the current color.
pub const BLACK_ASTERISK_SVG: &str =					formatcp!("<tspan style=\"font-family:{0};fill:black;stroke:none\">{1}</tspan>", FONT_MAIN, FIVE_LOBED_ASTERISK_UC);
		
pub const WHITE_ASTERISK_TAG: &str =					"[w*]";	// Force the asterisk to be displayed in white, not the current color.
pub const WHITE_ASTERISK_SVG: &str =					formatcp!("<tspan style=\"font-family:{0};fill:white;stroke:none\">{1}</tspan>", FONT_MAIN, FIVE_LOBED_ASTERISK_UC);
		
pub const FIVE_LOBED_ASTERISK_TAG: &str =				"[*]";
pub const FIVE_LOBED_ASTERISK_SVG: &str =				formatcp!("<tspan style=\"font-family:{0}\">{1}</tspan>", FONT_MAIN, FIVE_LOBED_ASTERISK_UC);
		
pub const UNDERLINED_CALIBER_NOTE_TAG: &str =			"[_*]";
		
pub const SIX_LOBED_ASTERISK_TAG: &str =				"<*>";
pub const SIX_LOBED_ASTERISK_SVG: &str = 				formatcp!("<tspan style=\"font-family:{0};{FONT_WEIGHT_BOLD};font-size:{1}px\">{2}</tspan>", FONT_ALT, SIX_LOBED_ASTERISK_FONT_SIZE_STR, SIX_LOBED_ASTERISK_UC);
pub const SIX_LOBED_ASTERISK_SUPER_SVG: &str = 			formatcp!("<tspan style=\"font-family:{0};{FONT_WEIGHT_BOLD};font-size:{1}px\" baseline-shift=\"super\">{2}</tspan>", FONT_ALT, SIX_LOBED_ASTERISK_FONT_SIZE_STR, SIX_LOBED_ASTERISK_UC);
		
pub const SIX_LOBED_ASTERISK_MG_SVG: &str =				formatcp!("<tspan style=\"font-family:{0};{FONT_WEIGHT_BOLD};font-size:{1}px\" baseline-shift=\"super\">{2}</tspan>", FONT_ALT, MGS_SIX_LOBED_ASTERISK_FONT_SIZE_STR, SIX_LOBED_ASTERISK_UC);
		
pub const SIX_LOBED_BLACK_ASTERISK_TAG: &str =			"<b*>";	// Force the asterisk to be displayed in black, not the current color.
pub const SIX_LOBED_BLACK_ASTERISK_SVG: &str =			formatcp!("<tspan style=\"font-family:{0};{FONT_WEIGHT_BOLD};fill:black;stroke:none\">{1}</tspan>", FONT_ALT, SIX_LOBED_ASTERISK_UC);

pub const SIX_LOBED_WHITE_ASTERISK_TAG: &str =			"<w*>";	// Force the asterisk to be displayed in white, not the current color.
pub const SIX_LOBED_WHITE_ASTERISK_SVG: &str =			formatcp!("<tspan style=\"font-family:{0};{FONT_WEIGHT_BOLD};fill:white;stroke:none\">{1}</tspan>", FONT_ALT, SIX_LOBED_ASTERISK_UC);

pub const SIX_LOBED_BLACK_ASTERISK_SUPER_SVG: &str =	formatcp!("<tspan style=\"font-family:{0};{FONT_WEIGHT_BOLD};fill:black;stroke:none\" baseline-shift=\"super\">{1}</tspan>", FONT_ALT, SIX_LOBED_ASTERISK_UC);

pub const CLOSE_TSPAN_SVG: &str =				"</tspan>";

pub const CALIBER_FONT_FAMILY_SVG: &str =		formatcp!("<tspan style=\"font-family:{0}\">", FONT_MAIN);

pub const UNARMORED_TAG: &str =					"<^>"; // Replace "<^>" with STAR character defined above.

#[derive(PartialEq, Default, Clone)]
pub enum NoteAction {
	#[default]
	None,
	Postfix,
	Prefix,
	Infix,		// For "Notes" that will be inserted, by the element generation code whererver desired.
	Delete,
}

impl fmt::Display for NoteAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoteAction::None => write!(f, "None"),
            NoteAction::Postfix => write!(f, "Postfix"),
            NoteAction::Prefix => write!(f, "Prefix"),
            NoteAction::Infix => write!(f, "Infix"),
			NoteAction::Delete => write!(f, "Delete"),
        }
    }
}

pub fn string_to_action(value: &String) -> NoteAction {
	let mut action: NoteAction = NoteAction::None;

	if value.contains(&MOD_NOTES_POSTFIX) {
		action = NoteAction::Postfix;
	} else if value.contains(&MOD_NOTES_PREFIX) {
		action = NoteAction::Prefix;
	} else if value.contains(&MOD_NOTES_INFIX) {
		action = NoteAction::Infix;
	// TODO DEPRECATED? } else if value.contains(&MOD_NOTES_DELETE) {
	// TODO DEPRECATED? 	action = NoteAction::Delete;
	}
	
	return action;
}

#[derive(PartialEq, Default, Clone)]
pub struct Note {
	pub action: NoteAction,
	pub text: String,
}

impl Note {
	pub fn initialize(&mut self, note: &str, act: NoteAction) {
		self.action = act;

		if note.contains(FIVE_LOBED_ASTERISK_TAG) {
			self.text = FIVE_LOBED_ASTERISK_SVG.to_string();
		} else if note.contains(SIX_LOBED_ASTERISK_TAG) {
			self.text = SIX_LOBED_ASTERISK_SVG.to_string();
		} else if note.contains(SIX_LOBED_BLACK_ASTERISK_TAG) {
			self.text = SIX_LOBED_BLACK_ASTERISK_SVG.to_string();
		} else if note.contains(BLACK_ASTERISK_TAG) {
			self.text = BLACK_ASTERISK_SVG.to_string();
		} else if note.contains(WHITE_ASTERISK_TAG) {
			self.text = WHITE_ASTERISK_SVG.to_string();
		} else {
			if NoteAction::Delete == self.action {
				self.text = SIX_LOBED_ASTERISK_UC.to_string();
			} else {
				self.text = FIVE_LOBED_ASTERISK_SVG.to_string();
			}
		}
	}
}

pub const PROCESS_ASTERISK_TAGS: bool =	true;
pub const LEAVE_ASTERISK_TAGS: bool =	false;

pub trait TextFieldTraits {
	fn initialize(&mut self, _source: &TextField) {
		panic!("TextFieldTraits::initialize() - No default implementation!");
	}

	fn process_overrides(&mut self, _overrides: &String, _convert_asterisk_tags: bool) {		
		panic!("TextFieldTraits::process_overrides() - No default implementation!");
	}
	
	fn sanitize(&mut self, _source: &String, _overrides: &String, _fonts: [[f64; 4]; 8], _colors: &Colors) {
		panic!("TextFieldTraits::sanitize() - No default implementation!");
	}		
}

#[derive(PartialEq, Default, Clone)]
pub struct TextField {
	pub is_set: bool,
	pub text: String,
	pub color: String,
	pub note: Note,
	pub alternate_location: String,
	pub fonts: FontsObj,
}

impl TextFieldTraits for TextField {
	fn initialize(&mut self, source: &TextField) {
		self.is_set = source.is_set;
		self.text = source.text.to_string();
		self.color = source.color.to_string();
		self.note = source.note.clone();
		self.fonts = source.fonts.clone();
		self.alternate_location = source.alternate_location.to_string();
	}

	fn process_overrides(&mut self, overrides: &String, convert_asterisk_tags: bool) {
		let entries: Vec<std::string::String> = extract_vector(&overrides, MOD_DELIMITER2);
		let mut value_read = false;
		
		for entry in &entries {
			if !value_read {
				self.text = convert_text(&entry, COPY_FIELD, &self.text.to_string());
				value_read = true;
			} else if entry.contains(MOD_INC_SIZE) {
				let temp: String = extract_from(&entry, MOD_INC_SIZE);
			
				self.fonts.adjust_size(temp.parse::<f64>().unwrap_or(0.0));
			} else if entry.contains(MOD_DEC_SIZE) {
				let temp: String = extract_from(&entry, MOD_DEC_SIZE);
			
				self.fonts.adjust_size(temp.parse::<f64>().unwrap_or(0.0) * -1.0);
			} else {
				self.alternate_location = entry.to_string();
			}

			if PROCESS_ASTERISK_TAGS == convert_asterisk_tags {
				if self.text.contains(FIVE_LOBED_ASTERISK_TAG) {
					self.text = convert_text(&self.text, FIVE_LOBED_ASTERISK_TAG, &FIVE_LOBED_ASTERISK_UC.to_string());
				}
				
				if self.text.contains(SIX_LOBED_ASTERISK_TAG) {
					self.text = convert_text(&self.text, SIX_LOBED_ASTERISK_TAG, SIX_LOBED_ASTERISK_SUPER_SVG);
				}
			}
			
			self.is_set = true;
		}		
	}
	
	fn sanitize(&mut self, source: &String, overrides: &String, fonts: [[f64; 4]; 8], colors: &Colors) {
		self.color = colors.text.to_string();
		self.fonts.initialize(fonts);
	
		if !overrides.is_empty() {
			let entries: Vec<std::string::String> = extract_vector(overrides, MOD_DELIMITER2);
			let mut value_read: bool = false;
			
			for entry in &entries {
				if !value_read {
					self.text = convert_text(&entry, COPY_FIELD, &strip_dagger_and_any_superscript_from_end(&source));
					value_read = true;
				} else if entry.contains(MOD_INC_SIZE) {
					let temp: String = extract_from(&entry, MOD_INC_SIZE);
				
					self.fonts.adjust_size(temp.parse::<f64>().unwrap_or(0.0));
				} else if entry.contains(MOD_DEC_SIZE) {
					let temp: String = extract_from(&entry, MOD_DEC_SIZE);
				
					self.fonts.adjust_size(temp.parse::<f64>().unwrap_or(0.0) * -1.0);
				} else {
					self.alternate_location = entry.to_string();
				}
			}
	
			if self.text.contains(FIVE_LOBED_ASTERISK_TAG) {
				self.text = convert_text(&self.text, FIVE_LOBED_ASTERISK_TAG, FIVE_LOBED_ASTERISK_UC);
			}
			
			if self.text.contains(SIX_LOBED_ASTERISK_TAG) {
				self.text = convert_text(&self.text, SIX_LOBED_ASTERISK_TAG, SIX_LOBED_ASTERISK_SUPER_SVG);
			}
			
			if self.text.contains("<sup>") {
				let temp_size: f64 = if 2.0 < self.fonts.size() { self.fonts.size() - 2.0 } else { self.fonts.size() };
				
				self.text = convert_superscripts(&self.text, temp_size);
			}
		}
	}	
}
