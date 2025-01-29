/* TODO: CREATE_WRECKS/CREATE_MALF_SIDE NOT YET?
use std::fmt;
use std::io::Write;
//
// Local files.
//
use crate::defines::*;
use crate::colors::*;
use crate::overrides::*;
use crate::structs::*;
use crate::turret::*;
use crate::utils::*;

#[derive(PartialEq)]
#[derive(Default)]
pub enum SpecialPositions {
	#[default]
	None,
	LowerLeft,
	UpperRight,
	LowerMiddle,
	UpperMiddle,
}

impl fmt::Display for SpecialPositions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpecialPositions::None => write!(f, "None"),
            SpecialPositions::LowerLeft => write!(f, "LowerLeft"),
            SpecialPositions::UpperRight => write!(f, "UpperRight"),
            SpecialPositions::LowerMiddle => write!(f, "LowerMiddle"),
			SpecialPositions::UpperMiddle => write!(f, "UpperMiddle"),
        }
    }
}

#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct Special {
	pub turret_type: TurretType,
	pub ll_lines: Vec<TextField>,	// Lower left lines
	pub ll_count: usize,
	pub ur_lines: Vec<TextField>,	// Upper right lines
	pub ur_count: usize,
	pub lm_lines: Vec<TextField>,	// Lower middle lines
	pub lm_count: usize,
	pub um_lines: Vec<TextField>,	// Upper middle lines
	pub um_count: usize,
	pub repair: String,
}

impl Special {
	pub fn initialize(&mut self, source: &String, overrides: &Overrides, limbered: bool, colors: &Colors) {
		let mut entries: Vec<std::string::String> = Default::default();
		let _limber_fire: String;
		let mut my_source = source.to_string();
		
		if source.contains("LF") {
			_limber_fire = extract_string(source, "]", "[");
			my_source = remove_string(source, ", LF", "]");
		}
		
		if !overrides.special.is_empty() {
			my_source = overrides.special.clone();
		}
		
		if !overrides.special_additions.is_empty() {
			my_source.push_str(",");
			my_source.push_str(&overrides.special_additions);
		}
		
		if my_source.contains(COMMA) {
			entries = extract_vector(&my_source, COMMA);
		} else {
			entries.push(my_source.to_string());
		}
		
		if limbered {
			let mut tf: TextField = Default::default();
			
			tf.text = "Limbered".to_string();
			tf.font_size = 106;
			tf.color = colors.text.to_string();
			
			self.ur_lines.push(tf);
		}
		
		for (position, entry) in entries.iter().enumerate() {
			if !entry.is_empty() {
				if 0 == position {
					self.turret_type = string_to_turret_type(&entry);
				} else {
					self.process_entry(&entry, colors);
				}
			}
		}
	}
	
	fn process_entry(&mut self, entry: &String, colors: &Colors) {
		let mut tf: TextField = Default::default();
		let mut sp: SpecialPositions = Default::default();

		tf.text = strip_dagger_and_any_superscript_from_end(entry);
		tf.font_size = 80;

		match tf.text.as_str() { 
			"QSU" => {
				tf.font_size = 133;
				sp = SpecialPositions::LowerLeft;
			}
			"NM" => {
				tf.text = "*NM".to_string();
				tf.font_size = 133;
				sp = SpecialPositions::LowerLeft;
			}
			"RFNM" => {
				tf.font_size = 133;

				tf.text = " ".to_string();				// "*RFNM" will take up too much space, reserve the bottom-most "LowerMiddle" text area.
				sp = SpecialPositions::LowerMiddle;
				self.insert(&tf, sp);
				
				tf.text = "*RFNM".to_string();
				sp = SpecialPositions::LowerLeft;
			}			
			"h-d" | "no IF" | "Towed" => {
				tf.font_size = 106;
				sp = SpecialPositions::LowerMiddle;
			}
			"IFE=B10" => {
				tf.text = "*IFE=B10".to_string();
				sp = SpecialPositions::LowerMiddle;
			}	
			"Towing risk" => {
				sp = SpecialPositions::LowerMiddle;
			}			
			"no shield" | "no gunshield" | "no Gunshield" => {
				tf.text = "no shield".to_string();
				sp = SpecialPositions::UpperMiddle;
			}
			"Towing NA" => {
				tf.text = SIX_LOBED_ASTERISK_SVG_SUFFIX.to_string();
				tf.text.push_str("Tow NA");
				sp = SpecialPositions::LowerMiddle;
			}
			"Road MP = 1" => {
				tf.text = SIX_LOBED_ASTERISK_SVG_SUFFIX.to_string();
				tf.text.push_str("Road MP=1");
				sp = SpecialPositions::UpperMiddle;
			}
			"black_th" => {
				tf.text = "*Black TH#".to_string();
				sp = SpecialPositions::UpperMiddle;
			}
			"H=B12" => {
				tf.text = "*H=B12".to_string();
				sp = SpecialPositions::LowerMiddle;
			}
			"IR<sup>3+</sup>" => {
				tf.text = convert_superscripts(&tf.text, 60);
				sp = SpecialPositions::LowerLeft;
			}
			"[3-24]<sup>3+</sup>" => {
				tf.text = convert_superscripts(&tf.text, 60);
				sp = SpecialPositions::LowerMiddle;				
			}
			"s8<sup>N1+</sup>" => {
				tf.text = convert_superscripts(&"*s8<sup>N1+</sup>".to_string(), 60);
				sp = SpecialPositions::LowerMiddle;				
			}
			"AA Fire ROF 2" => {
				tf.text = convert_superscripts(&"*AA ROF 2".to_string(), 60);
				sp = SpecialPositions::UpperMiddle;				
			}
			"2 TK DR" => {
				tf.text = "*2 TK DR".to_string();
				sp = SpecialPositions::LowerMiddle;				
			}
			"R2" => {
				self.repair = "2".to_string();
			}
			_ => {
				// Don't recognize it, so skip it.
			}
		}

		if SpecialPositions::None != sp {
			tf.color = colors.text.to_string();
			self.insert(&tf, sp);
		}
	}

	pub fn insert(&mut self, tf: &TextField, sp: SpecialPositions) {
		let mut inserted = false;
		let mut my_sp = sp;
		
		while SpecialPositions::None != my_sp && !inserted {
			match my_sp {
				SpecialPositions::LowerLeft => {
					if 2 == self.ll_count {
						my_sp = SpecialPositions::UpperRight;
					} else {
						self.ll_lines.push(tf.clone());
						inserted = true;
					}
				}
				SpecialPositions::UpperRight => {
					if 2 == self.ur_count {
						my_sp = SpecialPositions::LowerMiddle;
					} else {
						self.ur_lines.push(tf.clone());
						inserted = true;
					}
				}
				SpecialPositions::LowerMiddle => {
					if 2 == self.lm_count {
						my_sp = SpecialPositions::UpperMiddle;
					} else {
						self.lm_lines.push(tf.clone());
						inserted = true;
					}					
				}
				SpecialPositions::UpperMiddle => {
					if 2 == self.um_count {
						my_sp = SpecialPositions::None;	// No room at the Inn, nothing we can do with this one, hopefully never happens.
						println!("Cannot add special text.");
					} else {
						self.um_lines.push(tf.clone());
						inserted = true;
					}					
				}
				_ => {
					// We shouldn't get here.
				}
			}
		}		
	}
	
	pub fn find(&self, source: &String) -> TextField  {
		let mut result: TextField = Default::default();
		let mut found = false;
		
		for line in &self.ll_lines {
			if source.to_string() == line.text.to_string() {
				result = line.clone();
				found = true;
			}
		}
		
		if !found {
			for line in &self.ur_lines {
				if source.to_string() == line.text.to_string() {
					result = line.clone();
				}
			}
		}
		
		return result;
	}
	
	pub fn generate_svg(&self, mut counter_file: &std::fs::File) {
		let mut x_pos: u32 = 50;
		let mut y_pos: u32 = 950;
		
		for line in &self.ll_lines {
			write!(counter_file, "\t\t<text x=\"{0}\" y=\"{1}\" text-anchor=\"start\" style=\"font-size:{2}px;font-weight:normal;font-family:{3};fill:{4}\">{5}</text>\n", x_pos, y_pos, line.font_size, FONT_MAIN, line.color, line.text).unwrap();				
			y_pos -= line.font_size;
		}
		
		x_pos = 950;
		y_pos = 130;
		
		for line in &self.ur_lines {
			write!(counter_file, "\t\t<text x=\"{0}\" y=\"{1}\" text-anchor=\"end\" style=\"font-size:{2}px;font-weight:normal;font-family:{3};fill:{4}\">{5}</text>\n", x_pos, y_pos, line.font_size, FONT_MAIN, line.color, line.text).unwrap();				
			y_pos += line.font_size;
		}
		
		x_pos = 500;
		y_pos = 130;
		
		for line in &self.um_lines {
			write!(counter_file, "\t\t<text x=\"{0}\" y=\"{1}\" text-anchor=\"middle\" style=\"font-size:{2}px;font-weight:normal;font-family:{3};fill:{4}\">{5}</text>\n", x_pos, y_pos, line.font_size, FONT_MAIN, line.color, line.text).unwrap();				
			y_pos += line.font_size;
			y_pos += 2 * GUN_COLUMN_Y_GAP; // TODO Superscript handling!?
		}
		
		y_pos = 950;
		
		for line in &self.lm_lines {
			write!(counter_file, "\t\t<text x=\"{0}\" y=\"{1}\" text-anchor=\"middle\" style=\"font-size:{2}px;font-weight:normal;font-family:{3};fill:{4}\">{5}</text>\n", x_pos, y_pos, line.font_size, FONT_MAIN, line.color, line.text).unwrap();				
			y_pos -= line.font_size;
			y_pos -= 2 * GUN_COLUMN_Y_GAP; // TODO Superscript handling!?
		}		
	}
}
TODO: CREATE_WRECKS/CREATE_MALF_SIDE NOT YET? */