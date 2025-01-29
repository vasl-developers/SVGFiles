use std::io::prelude::*;
use std::path::Path;
use std::{error::Error, io, process};
// This lets us write `#[derive(Deserialize)]`.
use serde::Deserialize;
//
// Local files.
//
use common_functions::*;
use common_functions::colors::*;
use common_functions::debugging::*;
use common_functions::defines::*;
use common_functions::overrides::*;
use common_functions::text_field::*;
use common_functions::utils::*;

pub const BROKEN_MORALE_X_POSITION: f64 =	 	30.0;
pub const BROKEN_MORALE_Y_POSITION: f64 =		30.0;
pub const BROKEN_MORALE_SIZE: f64 =				16.0;
pub const BROKEN_MORALE_FONT_SIZE: f64 =		14.0;
pub const BROKEN_MORALE_STROKE_WIDTH: f64 =		 0.75;

pub const AE_X_POSITION: f64 =	 	36.0;
pub const AE_Y_POSITION: f64 =		 2.0;
pub const AE_HEIGHT: f64 =			15.0;
pub const AE_WIDTH: f64 =			10.0;
//
// Sanitized and parsed aircraft-specific record fields.
//
#[derive(Default)]
struct Record {
	destination: String,
	nationality: String,
	values: String,
	armor_leader: bool,
	assault_engineer: bool,
	broken: bool,
	wounded: bool,
	pieces: Vec<std::string::String>,	// Possibly multiple pieces per entry.
	piece: String,						// Each individual piece from "pieces" above.
	image: String,
	version: String,
	overrides: Overrides,	// Not needed here, but required for some interfaces.
	colors: Colors,
	svg_image_transform: String,
	comments: String,
}

impl Record {
	fn generate_unit_depiction_svg_elements(&mut self, mut counter_file: &std::fs::File, root_path: &String) {
		if INCLUDE_IMAGES {
			let path_prefix = "svg/";
			let file_type_svg = ".svg";
			let file_type_png = ".png";
	
			let paths: Vec<std::string::String> = [
				format!("{}{}{}", path_prefix, &self.image, file_type_svg),
				format!("{}{}{}", path_prefix, &self.image, file_type_png)
			].to_vec();
	
			for mut path in paths {
				let mut pathname: String = root_path.to_string();
				pathname.push_str(&path.to_string());
	
				if Path::new(&pathname).exists() {
					let mut transform: String = "scale(1.00)".to_string();
	
					if !self.svg_image_transform.is_empty() {
						if self.svg_image_transform.contains("scale") {
							transform = self.svg_image_transform.to_string();
						} else {
							transform = format!("{transform} {0}", self.svg_image_transform);
						}
					}
	
					if path.contains(SPACE) {
						path = path.replace(SPACE, "%20");
					}
	
					write!(counter_file, "\t<image id=\"Silhouettes\" x=\"0\" y=\"0\" width=\"48\" height=\"48\" preserveAspectRatio=\"xMidYMid meet\" transform=\"{transform}\" href=\"{path}\" xlink:href=\"{path}\"/>\n").unwrap();
					break;	// Our work here is done.
				}
			}
		}
	}

	fn generate_armor_leader_counter_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		write!(counter_file, "\t<text id=\"Values\" x=\"50%\" y=\"44\" style=\"font-size:12px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.values).unwrap();
	}
	
	fn generate_broken_leader_counter_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		let size = BROKEN_MORALE_SIZE - (2.0 * BROKEN_MORALE_STROKE_WIDTH);
		let mut font_size = BROKEN_MORALE_FONT_SIZE;
		let mut font_stretch = "font-stretch:semi-expanded";
		
		if "10" == self.values {
			font_size -= 1.0;
			font_stretch = "font-stretch:condensed"
		}
		
		generate_svg_start_element(counter_file, 1, BROKEN_MORALE_X_POSITION, BROKEN_MORALE_Y_POSITION, BROKEN_MORALE_SIZE, BROKEN_MORALE_SIZE, "Morale", &"yellow".to_string());
		
		write!(counter_file, "\t\t<rect id=\"Self rally\" x=\"1\" y=\"1\" width=\"{size}\" height=\"{size}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{0};stroke-width:{1};stroke-dasharray:none;stroke-opacity:1\"/>\n", self.colors.text, BROKEN_MORALE_STROKE_WIDTH).unwrap();
		write!(counter_file, "\t<text id=\"Morale\" x=\"50%\" y=\"80%\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};{font_stretch};text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.values).unwrap();
		
		write!(counter_file, "\t</svg>\n").unwrap();		
	}
	
	fn generate_leader_counter_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		let mut font_size = 12.0;
		let mut y_position: f64 = 24.0;
		
		if self.assault_engineer || self.wounded {
			y_position = 28.0;
		}
		
		if "(1)-0-8" == self.values && self.wounded {
			font_size = 9.0;
		}
		
		if self.assault_engineer {
			generate_svg_start_element(counter_file, 1, AE_X_POSITION, AE_Y_POSITION, AE_WIDTH, AE_HEIGHT, "Assault Engineer", &"red".to_string());
			write!(counter_file, "\t\t<image id=\"Assault Engineer\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{0}\" xlink:href=\"{0}\"/>\n", "./svg/dc.svg").unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();
		}

		write!(counter_file, "\t<text id=\"Values\" transform=\"translate(44,{y_position}) rotate(-90)\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.values).unwrap();
		
		if self.wounded {
			write!(counter_file, "\t<text id=\"Wounded\" x=\"46\" y=\"9\" style=\"font-size:8px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};text-anchor:end;fill:{0};fill-opacity:1;{FONT_MAIN}\">3MF</text>\n", self.colors.text).unwrap();
		}
	}
}

fn generate_svg_counter(record: &mut Record) {
	let path = &record.destination.to_string();
	let size = 48;
	
	print!("Generating '{0}.svg' ...", record.piece);
	//
	// Create the counter file.
	//
	let mut counter_file = match open_counter_file(&path, &record.piece) {
		Err(why) => panic!("couldn't create file: {0} {1}", record.piece, why),
		Ok(counter_file) => counter_file,
	};

	generate_counter_header_svg_elements("vasl_singleman_counters", &counter_file, size, &record.piece, &record.comments, &record.version);

	write!(counter_file, "\t<svg width=\"{0:.2}\" height=\"{0:.2}\" viewBox=\"0 0 1000 1000\">\n", size).unwrap();

	generate_counter_background_svg(&counter_file, &record.colors, &record.overrides);

	write!(counter_file, "\t</svg>\n").unwrap();

	generate_debug_working_area_svg(&counter_file);
	
	record.generate_unit_depiction_svg_elements(&counter_file, &path);

	if record.armor_leader {
		record.generate_armor_leader_counter_svg_elements(&counter_file);
	} else if record.broken {
		record.generate_broken_leader_counter_svg_elements(&counter_file);
	} else {
		record.generate_leader_counter_svg_elements(&counter_file);
	}

	generate_footer_svg(&counter_file);

	drop(counter_file);
	println!(" done.");
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
	nationality: String,
	values: String,
	armor: String,
	assault_engineer: String,
	broken: String,
	wounded: String,
	piece: String,
	image: String,
	version: String,
	svg_image_transform: String,
	comments: String,
}

impl SpreadsheetRecord {
	fn sanitize(&mut self, nat: &String) -> Record {
		let mut result: Record = Default::default();
		
		result.nationality = if nat.is_empty() { self.nationality.to_string() } else { nat.to_string() };
		result.destination = format!("{0}{1}/", get_destination_arg(), result.nationality); // get_destination() ensures a trailing '/'
		result.values = self.values.to_string();
		result.armor_leader = "yes" == self.armor;
		result.assault_engineer = "yes" == self.assault_engineer;
		result.broken = "yes" == self.broken;
		result.wounded = "yes" == self.wounded;

		result.pieces = extract_vector(&self.piece, OVERRIDE_DELIMITER);

		result.image = self.image.to_string();
		result.version = self.version.to_string();
		result.colors = nationality_to_colors(&result.nationality);
		result.svg_image_transform = self.svg_image_transform.to_string();
		result.comments = self.comments.to_string();
		
		return result;
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let mut rdr = csv::Reader::from_reader(io::stdin());

	for result in rdr.deserialize() {
		let mut spreadsheet_record: SpreadsheetRecord = result?;

		if !spreadsheet_record.nationality.is_empty() {
			// println!("{:?}", spreadsheet_record); // For debugging
			
			let mut record: Record = spreadsheet_record.sanitize(&"".to_string());
			let pieces = record.pieces.clone();
			
			for piece in pieces {
				if !piece.contains('@') {
					record.piece = piece.to_string();
					generate_svg_counter(&mut record);
				} else {
					let (piece, nationality) = piece.split_once("@").unwrap();
					let mut alt_record: Record = spreadsheet_record.sanitize(&nationality.to_string());
					
					alt_record.colors = nationality_to_colors(&nationality.to_string());
					alt_record.piece = piece.to_string();
					
					generate_svg_counter(&mut alt_record);
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
