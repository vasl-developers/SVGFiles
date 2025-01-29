use std::fmt;
use std::io::prelude::*;
//
// Local defines.
//
use crate::colors::*;
use crate::overrides::*;
use crate::text_field::*;
use crate::utils::*;

pub const ARM_FONT_WEIGHT: &str = "bold";

pub const ARM_SVG_HEIGHT: f64 =		12.0;
pub const ARM_SVG_WIDTH: f64 =		18.0;
pub const ARM_BOX_X: f64 =			 7.0;
pub const ARM_BOX_Y: f64 =			 1.0;
pub const ARM_BOX_SIZE: f64 =		10.0;
pub const ARM_CIRCLE_RADIUS: f64 =	 5.0;
pub const ARM_STROKE_WIDTH: f64 =	 0.50;

pub const ARM_X_POSITION: f64 = 40.0;
pub const FAR_Y_POSITION: f64 = 15.0;
pub const SAR_Y_POSITION: f64 = FAR_Y_POSITION + ARM_SVG_HEIGHT - 1.0;
pub const RAR_Y_POSITION: f64 = SAR_Y_POSITION + ARM_SVG_HEIGHT - 1.0;

pub const ARMOR_VALUE_FONT_SIZE: f64 =				8.0;
pub const ARMOR_VALUE_ALTERNATE_FONT_SIZE: f64 =	7.0;
pub const ARMOR_VALUE_NOTE_FONT_SIZE: f64 =			5.0;

#[derive(PartialEq)]
#[derive(Default)]
pub enum ArmorModifier {
	#[default]
	None,
	Superior,
	Inferior,
	ExtraInferior,
}

impl fmt::Display for ArmorModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArmorModifier::None => write!(f, "None"),
            ArmorModifier::Superior => write!(f, "Superior"),
            ArmorModifier::Inferior => write!(f, "Inferior"),
            ArmorModifier::ExtraInferior => write!(f, "ExtraInferior"),
        }
    }
}

#[derive(PartialEq)]
#[derive(Default)]
pub struct ArmorValue {
	pub value: String,
	pub modifier: ArmorModifier,
	pub stroke_color: String,
	pub fill_color: String,
	pub modifier_color: String,
	pub small_target_circle_color: String,
	pub small_target_circle: bool,
	pub large_target_circle: bool,	// Red dot for -3 sized targets (cough) Maus (cough).
	pub note: Note,					// Armor note asterisk.
	pub y_position: f64,
	pub comment: String,
}

impl ArmorValue {
	pub fn initialize(&mut self, y_position: f64, comment: String, colors: &Colors) {
		self.y_position = y_position;
		self.comment = comment;
		
		self.fill_color = colors.normal_target.to_string();		
		self.stroke_color = "none".to_string();
		self.modifier_color = colors.armor_modifier.to_string();
	}

	fn note_initialize(&mut self, tag: &str) {
		if self.value.contains(tag) {
			if self.value.starts_with(tag) {
				self.note.initialize(tag, NoteAction::Prefix);
			} else {
				self.note.initialize(tag, NoteAction::Postfix);
			}

			self.value = convert_text(&self.value, tag, "");			
		}		
	}
	
	pub fn generate_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		let armor_stroke_color: String = self.stroke_color.to_string();
		let mut armor_fill_color: String = self.fill_color.to_string();
		let armor_modifier: String = self.modifier_color.to_string();
	
		generate_svg_start_element(counter_file, 1, ARM_X_POSITION, self.y_position, ARM_SVG_WIDTH, ARM_SVG_HEIGHT, &self.comment, "white");
		
		if self.small_target_circle {
			write!(counter_file, "\t\t<circle cx=\"66%\" cy=\"50%\" r=\"{0:.2}\" style=\"display:inline;fill:{1};fill-opacity:1;stroke:none;stroke-width:1.00px;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", ARM_CIRCLE_RADIUS, self.small_target_circle_color).unwrap();
			armor_fill_color = BLACK.to_string(); // Force black text when we have a small target cicle to display on.
		}
	
		let mut font_size = ARMOR_VALUE_FONT_SIZE;
	
		if self.value.contains(&STAR.to_string()) {
			let mut prefix_note: String = Default::default();
			let mut postfix_note: String = Default::default();
			
			font_size = ARMOR_VALUE_ALTERNATE_FONT_SIZE;
	
			if NoteAction::Prefix == self.note.action {
				prefix_note = self.note.text.to_string();
				
				if prefix_note.contains(&SIX_LOBED_ASTERISK_UC.to_string()) {
					font_size = ARMOR_VALUE_NOTE_FONT_SIZE;
				}
			} else if NoteAction::Postfix == self.note.action {
				postfix_note = self.note.text.to_string();
				
				if prefix_note.contains(&SIX_LOBED_ASTERISK_UC.to_string()) {
					font_size = ARMOR_VALUE_NOTE_FONT_SIZE;
				}
			}
	
			write!(counter_file, "\t\t<text x=\"66%\" y=\"70%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{0}px;font-weight:{1};font-family:{2};fill:{3};fill-opacity:1;stroke:{4};stroke-width:0.33\">{5}{6}{7}</tspan></text>\n", font_size, ARM_FONT_WEIGHT, FONT_MAIN, armor_fill_color, armor_stroke_color, prefix_note, self.value, postfix_note).unwrap();
		} else {
			let mut y_pos: &str = "75";
			
			if ArmorModifier::None != self.modifier {
				let value: f64 = self.value.parse::<f64>().unwrap_or(0.0);
				
				if value > 10.0 {
					font_size = ARMOR_VALUE_ALTERNATE_FONT_SIZE;
					y_pos = "70";
				}
			}
			
			write!(counter_file, "\t\t<text x=\"66%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{1:.2}px;font-weight:{2};font-family:{3};fill:{4};fill-opacity:1;stroke:none;stroke-width:0.2\">{5}</tspan></text>\n", y_pos, font_size, ARM_FONT_WEIGHT, FONT_MAIN, armor_fill_color, self.value).unwrap();
		}
	
		if self.large_target_circle {
			write!(counter_file, "\t\t<circle cx=\"66%\" cy=\"50%\" r=\"3.00\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", armor_fill_color).unwrap(); // Red dot
		}
	
		if !self.value.contains(&STAR.to_string()) {
			if ArmorModifier::Superior == self.modifier {
				write!(counter_file, "\t\t<rect x=\"{0}\" y=\"{1}\" width=\"{2}\" height=\"{2}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{3};stroke-width:{4};stroke-dasharray:none;stroke-opacity:1\"/> <!-- Superior Turret Armor -->\n", ARM_BOX_X, ARM_BOX_Y, ARM_BOX_SIZE, armor_modifier, ARM_STROKE_WIDTH).unwrap();
			} else if ArmorModifier::Inferior == self.modifier {
				write!(counter_file, "\t\t<circle cx=\"66%\" cy=\"50%\" r=\"{0}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{1};stroke-width:{2};stroke-dasharray:none;stroke-opacity:1\"></circle> <!-- Inferior Turret Armor -->\n", ARM_CIRCLE_RADIUS, armor_modifier, ARM_STROKE_WIDTH).unwrap();
			} else if ArmorModifier::ExtraInferior == self.modifier {
				write!(counter_file, "\t\t<!-- (extra) Inferior Turret Armor -->\n").unwrap();
				write!(counter_file, "\t\t<circle cx=\"66%\" cy=\"50%\" r=\"5\" style=\"display:inline;fill:none;fill-opacity:1;stroke:black;stroke-width:2;stroke-dasharray:none;stroke-opacity:1\"></circle>\n").unwrap();
				write!(counter_file, "\t\t<circle cx=\"66%\" cy=\"50%\" r=\"5\" style=\"display:inline;fill:none;fill-opacity:1;stroke:white;stroke-width:0.5;stroke-dasharray:none;stroke-opacity:1\"></circle>\n").unwrap();
			}
		}
	
		if !self.value.contains(&STAR.to_string()) && (NoteAction::Prefix == self.note.action || NoteAction::Postfix == self.note.action) {
			let note_y_position = ARM_SVG_HEIGHT / 2.0;
				
			if self.note.text.contains(&SIX_LOBED_ASTERISK_UC.to_string()) {
				font_size = ARMOR_VALUE_NOTE_FONT_SIZE;
			}
			
			if ArmorModifier::None == self.modifier {
				write!(counter_file, "\t\t<text x=\"40%\" y=\"{0:.2}\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{1:.2}px;font-weight:{2};fill:black;fill-opacity:1;stroke-width:0.2\">{3}</tspan></text> <!-- {4} note -->\n", note_y_position, font_size, ARM_FONT_WEIGHT, self.note.text, self.comment).unwrap();
			} else {
				write!(counter_file, "\t\t<text x=\"20%\" y=\"{0:.2}\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{1:.2}px;font-weight:{2};fill:black;fill-opacity:1;stroke-width:0.2\">{3}</tspan></text> <!-- {4} note -->\n", note_y_position, font_size, ARM_FONT_WEIGHT, self.note.text, self.comment).unwrap();
			}
		}

		write!(counter_file, "\t</svg>\n").unwrap();
	}	
}

#[derive(PartialEq)]
#[derive(Default)]
pub struct ArmorValues {
	pub front: ArmorValue,
	pub side: ArmorValue,
	pub rear: ArmorValue,
	pub count: usize,
	pub target_size: i64,
}

impl ArmorValues {
	pub fn initialize(&mut self, af: &String, ta: &String, size: &String, overrides: &Overrides, colors: &Colors) {
		let temp: String = af.to_string();
		let armor_values: Vec<String>;

		self.front.initialize(FAR_Y_POSITION, "Front Armor".to_string(), colors);
		self.side.initialize(SAR_Y_POSITION, "Side/Rear Armor".to_string(), colors);
		self.rear.initialize(RAR_Y_POSITION, "Rear Armor".to_string(), colors);
			
		armor_values = extract_armor_values(&temp.to_string());

		let armor_values_count = armor_values.len();

		match armor_values_count {
			1 => {
				self.front.value = armor_values[0].to_string();
				self.side.value = armor_values[0].to_string();
				self.count = 2;
			}
			2 => {
				self.front.value = armor_values[0].to_string();
				self.side.value = armor_values[1].to_string();
				self.count = 2;
			}
			3 => {
				self.front.value = armor_values[0].to_string();
				self.side.value = armor_values[1].to_string();
				self.rear.value = armor_values[2].to_string();
				self.count = 3;
			}
			0_usize | 4_usize.. => {
				println!("ArmorValue::initialize(): Illegal value '{0}'!", armor_values_count);
			}
		}

		if !overrides.armor_front.is_empty() {
			if !overrides.armor_front.contains(COPY_FIELD) {
				self.front.value = overrides.armor_front.to_string();
			} else {
				self.front.value = convert_text(&overrides.armor_front, COPY_FIELD, &self.front.value);
			}
		}
		
		if !overrides.armor_side.is_empty() {
			if !overrides.armor_side.contains(COPY_FIELD) {
				self.side.value = overrides.armor_side.to_string();
			} else {
				self.side.value = convert_text(&overrides.armor_side, COPY_FIELD, &self.side.value);
			}
		}
		
		if !overrides.armor_rear.is_empty() {
			if !overrides.armor_rear.contains(COPY_FIELD) {
				self.rear.value = overrides.armor_rear.to_string();
			} else {
				self.rear.value = convert_text(&overrides.armor_rear, COPY_FIELD, &self.rear.value);
			}
		}

		self.note_helper(BLACK_ASTERISK_TAG);
		self.note_helper(SIX_LOBED_ASTERISK_TAG);
		self.note_helper(SIX_LOBED_BLACK_ASTERISK_TAG);
		self.note_helper(FIVE_LOBED_ASTERISK_TAG);

		let turret_armor = if overrides.turret_armor_modifiers.is_empty() { ta.to_string() } else { overrides.turret_armor_modifiers.to_string() };
		
		if 0 != turret_armor.len() {
			let modifiers = extract_armor_modifier_values(&turret_armor);

			for n in 0..modifiers.len() {
				if "--F" == modifiers[n] {
					self.front.modifier = ArmorModifier::ExtraInferior;
				} else if "-FSR" == modifiers[n] {
					self.front.modifier = ArmorModifier::Inferior;
					self.side.modifier = ArmorModifier::Inferior;
					self.rear.modifier = ArmorModifier::Inferior;
				} else if "+FSR" == modifiers[n] {
					self.front.modifier = ArmorModifier::Superior;
					self.side.modifier = ArmorModifier::Superior;
					self.rear.modifier = ArmorModifier::Superior;
				} else if "-SR" == modifiers[n] {
					self.side.modifier = ArmorModifier::Inferior;
					self.rear.modifier = ArmorModifier::Inferior;
				} else if "+SR" == modifiers[n] {
					self.side.modifier = ArmorModifier::Superior;
					self.rear.modifier = ArmorModifier::Superior;
				} else if "-F" == modifiers[n] {
					self.front.modifier = ArmorModifier::Inferior;
				} else if "+F" == modifiers[n] {
					self.front.modifier = ArmorModifier::Superior;
				}
			}
		}

		if !overrides.target_size.is_empty() {
			self.target_size = overrides.target_size.parse::<i64>().unwrap_or(0);
		} else {
			self.target_size = strip_all_occurances(&size, DAGGER).parse::<i64>().unwrap_or(0);
		}

		match self.target_size {
			-3 => {
				self.front.fill_color = colors.large_target.to_string();
				self.side.fill_color = colors.large_target.to_string();
				self.rear.fill_color = colors.large_target.to_string();
				self.rear.large_target_circle = true;
				self.front.modifier_color = colors.large_target.to_string();
				self.side.modifier_color = colors.large_target.to_string();
				self.rear.modifier_color = colors.large_target.to_string();
			}
			-2 => {
				self.front.fill_color = colors.large_target.to_string();
				self.side.fill_color = colors.large_target.to_string();
				self.front.modifier_color = colors.large_target.to_string();
				self.side.modifier_color = colors.large_target.to_string();				
			}
			-1 => {
				self.front.fill_color = colors.large_target.to_string();
				if self.side.value.contains(&STAR.to_string()) {
					self.side.fill_color = colors.normal_unarmored_target_fill.to_string();
					self.side.stroke_color = colors.normal_unarmored_target_stroke.to_string();
				}
				self.front.modifier_color = colors.large_target.to_string();
			}
			0 => {
				if self.front.value.contains(&STAR.to_string()) {
					self.front.fill_color = colors.normal_unarmored_target_fill.to_string();
					self.front.stroke_color = colors.normal_unarmored_target_stroke.to_string();
				}
				if self.side.value.contains(&STAR.to_string()) {
					self.side.fill_color = colors.normal_unarmored_target_fill.to_string();
					self.side.stroke_color = colors.normal_unarmored_target_stroke.to_string();
				}
			}
			1 => {
				if self.front.value.contains(&STAR.to_string()) {
					self.front.stroke_color = colors.small_unarmored_target_stroke.to_string();
					self.front.fill_color = colors.small_unarmored_target_fill.to_string();
					self.front.small_target_circle = false;
				} else {
					self.front.small_target_circle = true;
					self.front.small_target_circle_color = colors.small_target_circle.to_string();
				}
				if self.side.value.contains(&STAR.to_string()) {
					self.side.stroke_color = colors.normal_unarmored_target_stroke.to_string();
					self.side.fill_color = colors.normal_unarmored_target_fill.to_string();
					self.side.small_target_circle = false;
				} else {
					self.side.small_target_circle = false;
				}
			}
			2 => {
				if self.front.value.contains(&STAR.to_string()) {
					self.front.stroke_color = colors.small_unarmored_target_stroke.to_string();
					self.front.fill_color = colors.small_unarmored_target_fill.to_string();
					self.front.small_target_circle = false;
				} else {
					self.front.small_target_circle = true;
					self.front.small_target_circle_color = colors.small_target_circle.to_string();
				}
				if self.side.value.contains(&STAR.to_string()) {
					self.side.stroke_color = colors.small_unarmored_target_stroke.to_string();
					self.side.fill_color = colors.small_unarmored_target_fill.to_string();
					self.side.small_target_circle = false;
				} else {
					self.side.small_target_circle = true;
					self.side.small_target_circle_color = colors.small_target_circle.to_string();
				}
			}
			i64::MIN..=-4_i64 | 3_i64..=i64::MAX => {
			}
		}
	}

	pub fn generate_svg_elements(&mut self, counter_file: &std::fs::File) {
		self.front.generate_svg_elements(counter_file);
	
		self.side.generate_svg_elements(counter_file);
	
		if 3 == self.count {
			self.rear.generate_svg_elements(counter_file);
		}
	}
	
	fn note_helper(&mut self, tag: &str) {
		self.front.note_initialize(tag);
		self.side.note_initialize(tag);
		self.rear.note_initialize(tag);	
	}	
}

fn extract_armor_values(armor_factors: &String) -> Vec<String> {
	let mut result: Vec<std::string::String> = Default::default();
	let mut my_armor_factors: String = armor_factors.to_string();

	if armor_factors.contains("</") {	// A closing sup tag, e.g., "</sup>"
		my_armor_factors = mask_closing_html_tags(armor_factors).clone();
	}

	let factors = my_armor_factors.split('/');

	for factor in factors {
		result.push(strip_dagger_and_any_superscript_from_end(&factor.to_string()));
	}

	if 1 == result.len() {
		result.push(result[0].to_string());
	}

	return result;
}

fn extract_armor_modifier_values(ta: &String) -> Vec<String> {
	let mut result: Vec<std::string::String> = Default::default();
	let modifiers = ta.split('/');

	for modifier in modifiers {
		result.push(strip_all_occurances(modifier, DAGGER));
	}

	return result;
}
