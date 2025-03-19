use std::io::prelude::*;
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
use common_functions::armament::*;
use common_functions::colors::*;
use common_functions::common_record::*;
use common_functions::debugging::*;
use common_functions::defines::*;
use common_functions::malfunction::*;
use common_functions::movement::*;
use common_functions::overrides::*;
// TODO: CREATE_MALF_SIDE NOT YET? use common_functions::special::*;
use common_functions::text_field::*;
use common_functions::turret::*;
use common_functions::utils::*;

pub const GT_FONT_SIZE: f64 =		11.0;
pub const LIMBERED_FONT_SIZE: f64 =	 7.0;

#[derive(Default)]
struct Record {
	args: Arguments,
	common: CommonRecord,
	gun_type: TextField,
	movement: OrdnanceMovementValues,
	// TODO: NOT YET? special: Special, // Details for malfunction side.
	repair_or_disable: RepairValues,
	limbered: bool,
	limbered_data: String,
}

impl Record {
	fn reinitialize_limbered_data(&mut self) {
		self.limbered = true;

		if self.limbered_data.contains(LIMBERED_NO_FIRE) {
			self.common.turret.speed = TurretType::NonTurreted;
			self.gun_type.text = "".to_string();
			self.common.overrides.ma.ignore = true;
			self.common.overrides.ma.malf.ignore = true;
		}
		
		let mut entries: Vec<std::string::String> = Default::default();
		
		if self.limbered_data.contains(OVERRIDE_DELIMITER) {
			entries = extract_vector(&self.limbered_data, OVERRIDE_DELIMITER);
		} else if !self.limbered_data.is_empty() {
			entries.push(self.limbered_data.to_string());
		}
		
		for entry in entries {
			let mut temp: String;
		
			if entry.contains(NOVR_MA) {
				self.common.overrides.ma.text = extract_from(&entry, NOVR_MA);
				self.common.ma.sanitize_caliber(&self.common.ma.raw_caliber.to_string(), &self.common.overrides);
			} else if entry.contains(NOVR_MB) {
				self.common.overrides.ma.malf.text = extract_from(&entry, NOVR_MB);
				self.common.malfunction.sanitize(&"".to_string(), &self.common.overrides.ma, &self.common.colors);
			} else if entry.contains(NOVR_GT) {
				temp = extract_from(&entry, NOVR_GT);
		
				if !temp.is_empty() {
					self.gun_type.text = temp;
				}
			} else if entry.contains(NOVR_SIZE) {
				temp = extract_from(&entry, NOVR_SIZE);
		
				if !temp.is_empty() {
					self.movement.target_size = temp.parse::<i64>().unwrap_or(0);
		
					if 0 < self.movement.target_size {
						self.movement.manhandling_number.color = BLACK.to_string();
						self.movement.unhooking_penalty_color = self.common.colors.unhooking_penalty_color.to_string();
					} else if 0 == self.movement.target_size {
						self.movement.manhandling_number.color = self.common.colors.manhandling_fill.to_string();
						self.movement.unhooking_penalty_color = self.common.colors.unhooking_penalty_color.to_string();
					} else {
						self.movement.manhandling_number.color = RED.to_string();
						self.movement.unhooking_penalty_color = RED.to_string();
					}
				}
			} else if entry.contains(NOVR_MOUNT) {
				temp = extract_from(&entry, NOVR_MOUNT);
		
				if !temp.is_empty() {
					self.common.turret.speed = string_to_turret_type(&temp);
				}
			} else if entry.contains(NOVR_RANGE) {
				self.common.overrides.range_values = extract_from(&entry, NOVR_RANGE);
				self.common.ma.range.sanitize(&"".to_string(), &self.common.overrides.range_values, RANGE_FONTS, &self.common.colors);
			} else if entry.contains(NOVR_ROF) {
				self.common.ma.rof = sanitize_rof(&self.common.ma.rof.text, &entry, &self.common.colors); // PASS whole entry so we can recognize limbered override to set ROF to "".
			} else if entry.contains(NOVR_IFE) {
				self.common.ma.ife = sanitize_ife(&self.common.ma.ife.text, &entry, &self.common.colors); // PASS whole entry so we can recognize limbered override to set IFE to "".
			} else if entry.contains(NOVR_MANHANDLING) {
				temp = extract_from(&entry, NOVR_MANHANDLING);
		
				if !temp.is_empty() {
					if temp.contains(FIVE_LOBED_ASTERISK) {
						self.movement.manhandling_number.note.action = NoteAction::Infix;
						self.movement.manhandling_number.note.text = FIVE_LOBED_ASTERISK_UC.to_string();
						temp = strip_all_occurances(&temp.to_string(), FIVE_LOBED_ASTERISK);
					}
		
					self.movement.manhandling_number.text = temp;
		
					if self.movement.manhandling_number.text.contains("<b>") {
						self.movement.manhandling_number.text = extract_string(&self.movement.manhandling_number.text, &String::from("</b>"), &String::from("<b>"));
						self.movement.unhooking_penalty = true;
					} else {
						self.movement.unhooking_penalty = false;
					}						
				}
			}
		}
	}
}

fn generate_gun_type(mut output: &std::fs::File, gun_type: &TextField, overrides: &Overrides) {
	let mut gt: String = gun_type.text.clone();

	if !overrides.gt.is_empty() {
		gt = overrides.gt.to_string();
	}

	if !gt.is_empty() {
		generate_svg_start_element(output, 1, 27.00, 0.00, 30.00, 11.00, "Gun Type", &"cyan".to_string());
		write!(output, "\t\t<text x=\"100%\" y=\"100%\" dominant-baseline=\"auto\" text-anchor=\"end\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_BOLD};font-family:{1};fill:{2}\">{3}</text>\n", GT_FONT_SIZE, FONT_MAIN, gun_type.color, gt).unwrap();
		write!(output, "\t</svg>\n").unwrap();
	}
}

fn generate_counter_front(mut output: &std::fs::File, path: &String, unit_depiction: &String, record: &Record) {
	generate_counter_background_svg(output, 60, &record.common.colors, &record.common.overrides);
	
	if !record.common.overrides.ma.ignore {
		record.common.turret.generate_svg_elements(&output);
	}
	
	generate_unit_depiction_svg(output, &path, &unit_depiction, &record.common.note, &record.common.svg_image_transform, true, &record.common.name, record.common.display_name, &record.common.colors, &record.args);
	generate_debug_working_area_svg(&output);

	if !record.common.overrides.ma.ignore {
		generate_debug_gun_line_svg(output);
	
		let mut y_position = generate_gun_caliber_line(&output, &record.common);
		
		y_position -= generate_malfunction_elements(&output, &record.common.malfunction, y_position);

		if !record.common.ma.rof.text.is_empty() {
			generate_rof_element(&output, &record.common.ma.rof, y_position, &record.common.ma.color, false);
		}

		generate_range_and_special_ammunition_elements(&output, &record.common.ma);

		if !record.common.ma.range2.text.is_empty() {
			generate_range_element(&output, &record.common.ma.range2, true, y_position, "Range2");
		}

		generate_gun_type(&output, &record.gun_type, &record.common.overrides);
	}

	if 0 != record.movement.manhandling_number.text.len() {
		generate_manhandling_number_for_counter_front(&output, &record.movement);
	}

	if record.limbered {
		write!(output, "\t<text x=\"{0}\" y=\"8.00\" text-anchor=\"start\" style=\"font-size:{1:.2}px;{FONT_WEIGHT_NORM};font-family:{2};fill:{3}\">Limbered</text>\n", GUN_COLUMN_X_POSITION, LIMBERED_FONT_SIZE, FONT_MAIN, record.common.ma.color).unwrap();
	}
}

fn generate_counter_back(mut output: &std::fs::File, path: &String, unit_depiction: &String, record: &Record) {
	generate_counter_background_svg(output, 60, &record.common.colors, &record.common.overrides);

	write!(output, "\t<line x1=\"10\" y1=\"10\" x2=\"50\" y2=\"50\" style=\"stroke:{0}; stroke-width:3.00\"/>\n", record.common.colors.malfunction_x).unwrap();
	write!(output, "\t<line x1=\"10\" y1=\"50\" x2=\"50\" y2=\"10\" style=\"stroke:{0}; stroke-width:3.00\"/>\n", record.common.colors.malfunction_x).unwrap();
		
	if !record.repair_or_disable.repair.text.is_empty() {
		write!(output, "\t<text x=\"3.00\" y=\"11.00\" text-anchor=\"start\" style=\"font-size:8pt;font-family:{0};fill:{1}\">R{2}</text>\n", FONT_MAIN, record.repair_or_disable.repair.color, record.repair_or_disable.repair.text).unwrap();
	}

	if !record.repair_or_disable.disable.text.is_empty() {
		write!(output, "\t<text x=\"57.00\" y=\"57.00\" text-anchor=\"end\" style=\"font-size:8pt;font-family:{0};fill:{1}\">X{2}</text>\n", FONT_MAIN, record.repair_or_disable.disable.color, record.repair_or_disable.disable.text).unwrap();
	}

	// TODO not yet? record.special.generate_svg(output);	// Handle all the "special" text including "Limbered".

	if 0 != record.movement.manhandling_number.text.len() {
		generate_manhandling_number_for_counter_back(&output, &record.movement);
	}
	
	if record.limbered {
		write!(output, "\t<text x=\"57.00\" y=\"8.00\" text-anchor=\"end\" style=\"font-size:{0:.2}px;{FONT_WEIGHT_NORM};font-family:{1};fill:{2}\">Limbered</text>\n", LIMBERED_FONT_SIZE, FONT_MAIN, record.common.ma.color).unwrap();
	}
	
	generate_unit_depiction_svg(output, &path, &unit_depiction, &record.common.note, &record.common.svg_image_transform, false, &record.common.name, record.common.display_name, &record.common.colors, &record.args);
}

fn extract_nationality(source: &String, nationality: &String) -> std::string::String {
	let mut result = (&source[0..2]).to_string();

	if !nationality.is_empty() {
		result = nationality.to_string();
	}

	return result;
}

fn generate_counters(record: &Record) {
	let path = construct_path(&record.common.nationality, "gun", &record.args.destination);
	let mut piece: String = record.common.piece_front.clone();
	let mut name = record.common.name.clone();

	if record.limbered {
		piece.push_str("-l");
		name.push_str(" (Limbered)");
	}

	let unit_depiction: String = piece.clone();

	if !record.args.quiet {
		print!("Generating '{0}.svg' ({1}) ...", piece, record.common.note);
	} else {
		println!("{0}", piece);
	}
	
	//
	// Create the front counter file.
	//
	let mut output = match open_counter_file(&path, &piece) {
		Err(why) => panic!("couldn't create file: {0} {1}", piece, why),
		Ok(output) => output,
	};

	generate_counter_header_svg_elements("vasl_ordnance_counters", &output, 60, &name, &record.common.note, &record.common.comments, &record.common.version);
	generate_counter_front(&output, &path, &unit_depiction, &record);
	generate_footer_svg(&output);

	drop(output);

	if CREATE_MALF_SIDE /* TODO: NOT YET? && !record.common.overrides.ignore_rev*/ {
		//
		// Create the back counter file.
		//
		piece = record.common.piece_front.clone();

		if record.limbered {
			piece.push_str("-l");
			name.push_str(" (Limbered)");
		}

		piece.push_str("b");
		
		name = record.common.name.clone();
		name.push_str(" (Malfunctioned)");

		if !record.args.quiet {
			print!("Generating '{0}.svg' ({1}) ...", piece, record.common.note);
		} else {
			println!("{0}", piece);
		}
	
		output = match open_counter_file(&path, &piece) {
			Err(why) => panic!("couldn't create file: {0} {1}", piece, why),
			Ok(output) => output,
		};

		generate_counter_header_svg_elements("vasl_gun_counters", &output, 60, &name, &record.common.note, &record.common.comments, &record.common.version);
		generate_counter_back(&output, &path, &unit_depiction, &record);
		generate_footer_svg(&output);

		drop(output);
	}

	if !record.args.quiet {
		println!(" done.");
	}
}

pub fn sanitize_repair_numbers(source: &String, /* TODO: CREATE_MALF_SIDE NOT YET? special_repair: &String,*/ colors: &Colors) -> RepairValues {
	let mut result: RepairValues = Default::default();
	let mut repair: &str = "1";
	let mut disable: &str = "6";

	if source.contains('/') {
		(repair, disable) = source.split_once('/').unwrap();
	}

	// TODO: NOT YET? if !special_repair.is_empty() {
	// TODO: NOT YET? 	result.repair.text = special_repair.to_string();
	// TODO: NOT YET? } else {
		result.repair.text = repair.to_string();
	// TODO: NOT YET? }

	result.repair.color = colors.text.to_string();

	result.disable.text = disable.to_string();
	result.disable.color = colors.text.to_string();

	return result;
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
	count: String,
	name: String,
	gun_type: String,
	caliber: String,
	rof_ife: String,
	breakdown: String,
	range: String,
	manhandling: String,
	target_size: String,
	dates: String,
	special: String,
	bpv: String,
	rf: String,
	notes: String,
	version: String,
	piece: String,
	r_x: String,
	overrides: String, // Overrides various items from the original CSV entry. '|'-separated list of overrides of the form "XXX=YYY" or just "ZZZ". See const declarations in overrides.rs for complete list.
	limbered: String,
	svg_image_transform: String,
	reverse: String,
	comments: String,
}

impl SpreadsheetRecord {
	fn sanitize(&mut self, nat: &String, args: &Arguments) -> Record {
		let mut result: Record = Default::default();
		let mut nationality = if nat.is_empty() { extract_nationality(&self.piece, &result.common.overrides.nationality) } else { nat.to_string() };
		
		if self.overrides.contains(NOVR_NATIONALITY) {
			nationality = extract_from(&self.overrides, NOVR_NATIONALITY);
		}
		
		result.args = args.clone();
		
		result.common.overrides.sanitize(&self.overrides);
		
		result.common.initialize(&nationality, &self.notes, &self.name, &self.caliber, &self.range, &self.rof_ife, &self.breakdown, &self.version, &self.piece, &self.svg_image_transform, &self.comments);
		
		result.common.turret = sanitize_mount(&self.special, &result.common.overrides, &result.common.colors);
		
		if !result.common.overrides.special_ammo.is_empty() {
			result.common.ma.special_ammo.sanitize(&"".to_string(), &result.common.overrides.special_ammo, RANGE_FONTS, &result.common.colors);
		}
		
		result.gun_type = sanitize_gun_type(&self.gun_type, &result.common.overrides, &result.common.colors);
		
		result.movement.sanitize(&self.manhandling, strip_all_occurances(&self.target_size, DAGGER).parse::<i64>().unwrap_or(0), &self.special, &result.common.overrides, &result.common.colors);
		
		// TODO: CREATE_MALF_SIDE NOT YET? result.special.initialize(&self.special, &result.common.overrides, result.limbered, &result.common.colors);
	
		result.repair_or_disable = sanitize_repair_numbers(&self.r_x, /* TODO: CREATE_MALF_SIDE NOT YET? &result.special.repair,*/ &result.common.colors);
	
		result.limbered_data = self.limbered.clone();
	
		return result;
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let mut args = Arguments::parse();
	
	args.sanitize_destination();
		
	let mut rdr = csv::Reader::from_reader(io::stdin());

	for result in rdr.deserialize() {
		let mut spreadsheet_record: SpreadsheetRecord = result?;

		if NOVR_ANNOUNCE == spreadsheet_record.overrides {
			if !args.quiet {
				println!("{}", strip_html_bold(&spreadsheet_record.count));
			}
		} else if !spreadsheet_record.overrides.contains(NOVR_IGNORE) {
			let mut record: Record = spreadsheet_record.sanitize(&"".to_string(), &args);
			let mut limbered_record: Record = spreadsheet_record.sanitize(&"".to_string(), &args);
			let pieces = record.common.pieces.clone();

			for piece in pieces {
				if !piece.contains('@') {
					record.common.piece_front = piece.to_string();
					generate_counters(&record);
					
					if !record.limbered_data.is_empty() {
						limbered_record.common.piece_front = piece.to_string();
						limbered_record.reinitialize_limbered_data();
					
						generate_counters(&limbered_record);
					}
				} else {
					let (piece_name, nationality) = piece.split_once("@").unwrap();
					let mut alt_record: Record = spreadsheet_record.sanitize(&nationality.to_string(), &args);
					let mut alt_limbered_record: Record = spreadsheet_record.sanitize(&nationality.to_string(), &args);
					
					alt_record.common.nationality = nationality.to_string();
					alt_record.common.colors = nationality_to_colors(&nationality.to_string());
					alt_record.common.piece_front = piece_name.to_string();
					
					generate_counters(&alt_record);
					
					if !alt_record.limbered_data.is_empty() {
						alt_limbered_record.common.nationality = nationality.to_string();
						alt_limbered_record.common.colors = nationality_to_colors(&nationality.to_string());
						alt_limbered_record.common.piece_front = piece_name.to_string();
					
						alt_limbered_record.reinitialize_limbered_data();
					
						generate_counters(&alt_limbered_record);
					}					
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
