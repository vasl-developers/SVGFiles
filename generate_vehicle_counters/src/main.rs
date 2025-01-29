use std::io::prelude::*;
use std::{error::Error, io, process};
// This lets us write `#[derive(Deserialize)]`.
use serde::Deserialize;
//
// Local files.
//
use common_functions::*;
use common_functions::armament::*;
use common_functions::armor::*;
use common_functions::colors::*;
use common_functions::common_record::*;
use common_functions::debugging::*;
use common_functions::defines::*;
use common_functions::machine_guns::*;
use common_functions::malfunction::*;
use common_functions::movement::*;
use common_functions::overrides::*;
use common_functions::text_field::*;
use common_functions::transport::*;
use common_functions::utils::*;
//
// Sanitized and parsed vehicle-specific record fields.
//
#[derive(Default)]
struct Record {
	common: CommonRecord,
	sa: Armament,
	sa_malfunction: Malfunction,
	mgs: MachineGuns,
	armor: ArmorValues,
	movement_values: VehicleMovementValues,
	transport_values: TransportValues,
}

fn generate_armament_elements(counter_file: &std::fs::File, record: &Record) {
	let mut y_position: f64;
	let mut pp_generated = false;

	generate_debug_gun_line_svg(counter_file);
	
	if !record.common.overrides.ma.ignore {
		y_position = generate_gun_caliber_line(&counter_file, &record.common);

		if !record.common.overrides.sa.ignore && !record.sa.caliber.is_empty() {
			y_position -= generate_sa_elements(&counter_file, &record.sa, &record.common.overrides, y_position);
		}

		if !record.common.ma.range.text.is_empty() {
			y_position -= generate_range_element(&counter_file, &record.common.ma.range, record.mgs.field.text.is_empty(), y_position, "Range");
		}

		if !record.common.ma.range2.text.is_empty() {
			y_position -= generate_range_element(&counter_file, &record.common.ma.range2, record.mgs.field.text.is_empty(), y_position, "Range 2");
		}

		if !record.common.ma.raw_caliber.is_empty() && record.transport_values.pp.is_set && record.transport_values.pp.alternate_location.is_empty() {
			y_position -= generate_pp_number_element(&counter_file, &record.transport_values.pp, GUN_COLUMN_X_POSITION, y_position, &"start".to_string());
			pp_generated = true;
		}

		y_position -= generate_malfunction_elements(&counter_file, &record.common.malfunction, y_position);
		y_position -= generate_malfunction_elements(&counter_file, &record.sa_malfunction, y_position);

		if !record.common.ma.rof.text.is_empty() {
			y_position -= generate_rof_element(&counter_file, &record.common.ma.rof, y_position, &record.common.ma.color, record.common.overrides.ma.multiple_hits);
		}	

		if !record.common.ma.ife.text.is_empty() && !record.common.ma.ife.alternate_location.is_empty() {
			generate_ife_element_alternate_location(counter_file, &record.common.ma, y_position);
		}		
	}

	if !record.transport_values.pp.alternate_location.is_empty() {
		generate_pp_number_element(&counter_file, &record.transport_values.pp, TOWING_PP_X_POSITION, TOWING_Y_POSITION, &"end".to_string());
	} else if pp_generated {
		generate_towing_number_element(&counter_file, &record.transport_values.towing, TOWING_X_POSITION, TOWING_Y_POSITION, &"end".to_string());
	} else { // for unarmored vehicles
		y_position = GUN_COLUMN_Y_POSITION;
		y_position -= generate_pp_number_element(&counter_file, &record.transport_values.pp, GUN_COLUMN_X_POSITION, y_position, &"start".to_string());
		generate_towing_number_element(&counter_file, &record.transport_values.towing, GUN_COLUMN_X_POSITION, y_position, &"start".to_string());
	}
}

fn generate_counter_front(mut counter_file: &std::fs::File, path: &String, record: &mut Record) {
	write!(counter_file, "\t<svg width=\"60.00\" height=\"60.00\" viewBox=\"0 0 1000 1000\">\n").unwrap();

	generate_counter_background_svg(counter_file, &record.common.colors, &record.common.overrides);

	write!(counter_file, "\t</svg>\n").unwrap();

	generate_debug_working_area_svg(&counter_file);
	
	record.common.turret.generate_svg_elements(&counter_file);
	
	generate_unit_depiction_svg(counter_file, &path, &record.common.piece_front, &record.common.note, &record.common.svg_image_transform, true, &record.common.name, &record.common.colors);

	generate_armament_elements(&counter_file, &record); // Construct the whole "gun stack" of information, containing (potentially) the gun caliber, ROF, breakdown number, IFE, PP #, etc.).

	record.armor.generate_svg_elements(&counter_file);
	
	record.mgs.generate_svg_elements(&counter_file);

	record.movement_values.generate_svg_elements(&counter_file, &record.common.colors);	

	if record.transport_values.manhandling_number.is_set {
		generate_nimbus_manhandling_number_element(&counter_file, &record.transport_values.manhandling_number, &record.common.colors.text);
	}
}

/* TODO: CREATE_WRECKS NOT YET?
fn generate_counter_back(mut counter_file: &std::fs::File, nationality: &String, background_color: &String, record: &Record) {
	write!(counter_file, "\t\t<image x=\"0\" y=\"0\" height=\"1000\" width=\"1000\" href=\"{0}/{0}blank58.svg\"/>\n", background_color).unwrap();
	write!(counter_file, "\t\t<text x=\"50%\" y=\"50%\" dominant-baseline=\"Central\" text-anchor=\"middle\"style=\"font-size:1200px;{FONT_WEIGHT_BOLD};font-family:{0};fill:#ffffff\">✕</text>\n", FONT_MAIN).unwrap(); // Malfunctioned
	write!(counter_file, "\t\t<text x=\"50\" y=\"158\" text-anchor=\"start\" style=\"font-size:160.0px;{FONT_WEIGHT_BOLD};font-family:{0};fill:#000000\">R1</text>\n", FONT_MAIN).unwrap();
	write!(counter_file, "\t\t<text x=\"954\" y=\"950\" text-anchor=\"end\" style=\"font-size:160.0px;{FONT_WEIGHT_BOLD};font-family:{0};fill:#000000\">X6</text>\n", FONT_MAIN).unwrap();

	if INCLUDE_IMAGES {
		write!(counter_file, "\t\t<image x=\"33\" y=\"33\" width=\"934\" height=\"934\" transform=\"rotate(-90)\" href=\"{0}/veh/svg/{1}.svg\"></image> <!-- For now ... -->\n", nationality, record.piece).unwrap();
	}
}
TODO: CREATE_WRECKS NOT YET? */

// TODO: NOT YET fn process_notes(record: &mut Record, field: &String, action_string: &String, note: &String) {
// TODO: NOT YET 	let action: NoteAction;
// TODO: NOT YET 
// TODO: NOT YET 	if action_string.contains(&OVR_NOTES_POSTFIX) {
// TODO: NOT YET 		action = NoteAction::Postfix;
// TODO: NOT YET 	} else if action_string.contains(&OVR_NOTES_PREFIX) {
// TODO: NOT YET 		action = NoteAction::Prefix;
// TODO: NOT YET 	} else if action_string.contains(&OVR_NOTES_INFIX) {
// TODO: NOT YET 		action = NoteAction::Infix;
// TODO: NOT YET 	} else if action_string.contains(&OVR_NOTES_DELETE) {
// TODO: NOT YET 		action = NoteAction::Delete;
// TODO: NOT YET 	} else {
// TODO: NOT YET 		action = NoteAction::None;
// TODO: NOT YET 	}
// TODO: NOT YET 
// TODO: NOT YET 	if OVR_FIELD_MA == field {
// TODO: NOT YET 		record.common.ma.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_SA == field {
// TODO: NOT YET 		record.sa.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_BRK_SA == field {
// TODO: NOT YET 		record.sa_malfunction.breakdown.value.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_BRK_MA == field {
// TODO: NOT YET 		record.common.malfunction.breakdown.value.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_ROF == field {
// TODO: NOT YET 		record.common.ma.rof.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_MP == field {
// TODO: NOT YET 		record.movement_values.points.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_PP == field {
// TODO: NOT YET 		record.transport_values.pp.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_TOW == field {
// TODO: NOT YET 		record.transport_values.towing.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_MAU == field {
// TODO: NOT YET 		record.common.ma.underline_note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_FAR == field {
// TODO: NOT YET 		record.armor.front.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_SAR == field {
// TODO: NOT YET 		record.armor.side.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_RAR == field {
// TODO: NOT YET 		record.armor.rear.note.initialize(note, action);
// TODO: NOT YET 	}
// TODO: NOT YET }
// TODO: NOT YET 
// TODO: NOT YET fn add_notes(record: &mut Record, notes: &String) {
// TODO: NOT YET 	if notes.contains(OVR_NOTES_DELIMITER) {
// TODO: NOT YET 		let entries: Vec<std::string::String> = extract_vector(notes, OVR_NOTES_DELIMITER);
// TODO: NOT YET 
// TODO: NOT YET 		for entry in entries {
// TODO: NOT YET 			let fields: Vec<std::string::String> = extract_vector(&entry, OVR_NOTES_SEPARATOR);
// TODO: NOT YET 
// TODO: NOT YET 			process_notes(record, &fields[0], &fields[1], &fields[2]);
// TODO: NOT YET 		}
// TODO: NOT YET 	} else if !notes.is_empty() && notes.contains(OVR_NOTES_SEPARATOR) {
// TODO: NOT YET 		let fields: Vec<std::string::String> = extract_vector(notes, OVR_NOTES_SEPARATOR);
// TODO: NOT YET 
// TODO: NOT YET 		process_notes(record, &fields[0], &fields[1], &fields[2]);
// TODO: NOT YET 	}
// TODO: NOT YET }
// TODO: NOT YET 
// TODO: NOT YET fn process_font_sizes(record: &mut Record, field: &String, font_size: f64) {
// TODO: NOT YET 	if OVR_FIELD_MA_RANGE == field {
// TODO: NOT YET 		record.common.ma.range.font_size = font_size;
// TODO: NOT YET 	} else if OVR_FIELD_MA == field {
// TODO: NOT YET 		record.common.ma.font_size = font_size;
// TODO: NOT YET 	}
// TODO: NOT YET }
// TODO: NOT YET 
// TODO: NOT YET fn add_font_sizes(record: &mut Record, font_sizes: &String) {
// TODO: NOT YET 	if font_sizes.contains(OVR_NOTES_DELIMITER) { // borrowing OVR_NOTES_DELIMITER
// TODO: NOT YET 		let entries: Vec<std::string::String> = extract_vector(font_sizes, OVR_NOTES_DELIMITER);
// TODO: NOT YET 
// TODO: NOT YET 		for entry in entries {
// TODO: NOT YET 			let fields: Vec<std::string::String> = extract_vector(&entry, OVR_NOTES_SEPARATOR);
// TODO: NOT YET 
// TODO: NOT YET 			process_font_sizes(record, &fields[0], fields[1].parse::<f64>().unwrap_or(0));
// TODO: NOT YET 		}
// TODO: NOT YET 	} else if !font_sizes.is_empty() && font_sizes.contains(OVR_NOTES_SEPARATOR) {
// TODO: NOT YET 		let fields: Vec<std::string::String> = extract_vector(font_sizes, OVR_NOTES_SEPARATOR);
// TODO: NOT YET 
// TODO: NOT YET 		process_font_sizes(record, &fields[0], fields[1].parse::<f64>().unwrap_or(0));
// TODO: NOT YET 	}
// TODO: NOT YET }

fn generate_counter(record: &mut Record, note_number: &String) {
	print!("Generating '{0}.svg' ({1}) ...", record.common.piece_front, note_number);

	let path = construct_path(&record.common.nationality, "veh", &record.common.destination);
	//
	// Create the front counter file.
	//
	let counter_file = match open_counter_file(&path, &record.common.piece_front) {
		Err(why) => panic!("couldn't create file: {0} {1}", record.common.piece_front, why),
		Ok(counter_file) => counter_file,
	};

	generate_header_svg("vasl_vehicle_counters", &counter_file, &note_number, &record.common.name, &record.common.comments, &record.common.version);
	generate_counter_front(&counter_file, &path, record);
	generate_footer_svg(&counter_file);

	drop(counter_file);

/* TODO: NOT YET?
	if CREATE_WRECKS && !ignore_element(&record.common.overrides, &IGNORE_REVERSE) {
		let mut piece: String = record.piece.clone();
		piece.push_str("b");

		counter_file = match open_counter_file(&nationality, "veh", &piece) {
			Err(why) => panic!("couldn't create file: {0} {1}", piece, why),
			Ok(counter_file) => counter_file,
		};

		generate_header("vasl_vehicle_counters", &counter_file, &note_number, &record.name, &record.comments, &record.version);
		generate_counter_back(&counter_file, &nationality, &background_nationality, &record);
		generate_footer(&counter_file);

		drop(counter_file);
	}
TODO: NOT YET? */
	println!(" done.");
}

fn generate_counters(record: &mut Record) {
	let note_number: String = record.common.note.clone();
	
	if record.common.overrides.copy {
		let _ = copy_counter("veh", &record.common.nationality, &record.common.piece_front, &note_number, &record.common.destination);
	} else if !record.common.nationality.is_empty() {
		generate_counter(record, &note_number);
	} else {
		println!("Missing nationality for piece '{0}'", record.common.piece_front);
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
	count: String,
	name: String,
	radioless: String,
	weight: String,
	bpv: String,
	rf: String,
	dates: String,
	size: String,
	af: String,
	ta: String,
	ot: String,
	cs: String,
	mp: String,
	gp: String,
	gt: String,
	ma: String,
	rof_ife: String,
	breakdown: String,
	intensive_fire: String,
	bmg: String,
	cmg: String,
	aamg: String,
	sa: String,
	ammunition: String,
	smoke_depletion: String,
	smoke_discharger: String,
	transport: String,
	notes: String,
	version: String,
	piece: String,
	overrides: String,		// Overrides various items from the original CSV entry. '|'-separated list of overrides of the form "XXX=YYY" or just "ZZZ". See const declarations in lib.rs for complete list.
	svg_image_transform: String,
	reverse: String,
	comments: String,
}

impl SpreadsheetRecord {
	fn sanitize(&mut self, destination: &String, nat: &String) -> Record {
		let mut result: Record = Default::default();
		let nationality = if nat.is_empty() { extract_from(&self.overrides, NOVR_NATIONALITY) } else { nat.to_string() };

		result.common.destination = destination.to_string();
		result.common.overrides.sanitize(&self.overrides);
		result.common.initialize(&nationality, &self.notes, &self.name, &self.ma, &"".to_string(), &self.rof_ife, &self.breakdown, &self.version, &self.piece, &self.svg_image_transform, &self.comments);
		result.common.turret = sanitize_mount(&self.gt, &result.common.overrides, &result.common.colors);
		result.common.ma.special_ammo.sanitize(&self.ammunition, &result.common.overrides.special_ammo, RANGE_FONTS, &result.common.colors);

		if !self.sa.is_empty() || self.overrides.contains("NOVR_SA") {
			result.sa.is_secondary = true;
			result.sa.sanitize(&self.sa, &"".to_string(), &"".to_string(), &result.common.overrides, &result.common.colors);
		}
		
		result.sa_malfunction.sanitize(&"".to_string(), &result.common.overrides.sa, &result.common.colors); // SA breakdown MUST be specified via Override.
		
		result.armor.initialize(&self.af, &self.ta, &self.size, &result.common.overrides, &result.common.colors);
		result.movement_values.sanitize(&self.name, &self.mp, &self.gp, &result.common.overrides, !self.ot.is_empty(), &result.common.colors);
		result.transport_values.sanitize(&self.transport, &result.common.overrides, &result.common.colors);

		result.mgs.sanitize(&self.bmg, &self.cmg, &self.aamg, &result.common.overrides, &result.common.colors);
		
		return result;
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let mut rdr = csv::Reader::from_reader(io::stdin());
	let destination = get_destination_arg();

	for result in rdr.deserialize() {
		let mut spreadsheet_record: SpreadsheetRecord = result?;

	if NOVR_ANNOUNCE == spreadsheet_record.overrides {
		let announcement = strip_html_bold(&spreadsheet_record.count);
		
		println!("{announcement}");
	} else if !spreadsheet_record.overrides.contains(NOVR_IGNORE) {
			// println!("{:?}", spreadsheet_record); // For debugging
			let mut record: Record = spreadsheet_record.sanitize(&destination, &"".to_string());
			let pieces = record.common.pieces.clone();
			
			for piece in pieces {
				if !piece.contains('@') {
					record.common.piece_front = piece.to_string();
					generate_counters(&mut record);
				} else {
					let (piece_name, nationality) = piece.split_once("@").unwrap();
					let mut alt_record: Record = spreadsheet_record.sanitize(&destination, &nationality.to_string());
					
					alt_record.common.colors = nationality_to_colors(&nationality.to_string());
					alt_record.common.piece_front = piece_name.to_string();
					
					generate_counters(&mut alt_record);
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
