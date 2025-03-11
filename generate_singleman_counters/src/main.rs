use std::io::prelude::*;
use std::path::Path;
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

pub const COL_X_POSITION: f64 =	 	 	 2.0;
pub const COL_X_POSITION_SNIPER: f64 =	10.0;
pub const COL_Y_POSITION: f64 =		 	 2.0;
pub const COL_HEIGHT: f64 =				12.0;
pub const COL_WIDTH: f64 =			 	 8.0;
//
// Sanitized and parsed aircraft-specific record fields.
//
#[derive(Default)]
struct Record {
	args: Arguments,
	nationality: String,
	values: String,
	armor_leader: bool,
	assault_engineer: bool,
	broken: bool,
	wounded: bool,
	pieces: Vec<std::string::String>,	// Possibly multiple pieces per entry.
	piece: String,						// Each individual piece from "pieces" above.
	version: String,
	overrides: Overrides,
	colors: Colors,
	svg_image_transform: String,
	comments: String,
}

impl Record {
	fn generate_unit_depiction_svg_elements(&mut self, mut output: &std::fs::File, root_path: &String, size: u32) {
		if INCLUDE_IMAGES {
			let path_prefix = "svg/";
			let file_type_svg = ".svg";
			let file_type_png = ".png";
	
			let paths: Vec<std::string::String> = [
				format!("{}{}{}", path_prefix, &self.piece, file_type_svg),
				format!("{}{}{}", path_prefix, &self.piece, file_type_png)
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
	
					write!(output, "\t<image id=\"Silhouettes\" x=\"0\" y=\"0\" width=\"{size}\" height=\"{size}\" preserveAspectRatio=\"xMidYMid meet\" transform=\"{transform}\" style=\"opacity:{0:.2}\" href=\"{path}\" xlink:href=\"{path}\"/>\n", self.overrides.opacity).unwrap();
					break;	// Our work here is done.
				}
			}
		}
	}

	fn generate_crew_pass_counter_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		
		write!(output, "\t<text x=\"3\" y=\"14\" style=\"font-size:13px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-condensed;text-anchor:start;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">CE</text>\n").unwrap();
		write!(output, "\t<text x=\"3\" y=\"21.5\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-condensed;text-anchor:start;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">crew</text>\n").unwrap();
		write!(output, "\t<line x1=\"3\" y1=\"23.5\" x2=\"45\" y2=\"23.5\" id=\"dash2\" style=\"stroke:{color}; stroke-width:1.5\"/>\n").unwrap();
		write!(output, "\t<text x=\"3\" y=\"31.5\" style=\"font-size:10px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-condensed;text-anchor:start;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">pass</text>\n").unwrap();
		write!(output, "\t<text x=\"3\" y=\"44\" style=\"font-size:13px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-condensed;text-anchor:start;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">BU</text>\n").unwrap();
	}
	
	fn generate_nt_bu_counter_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		
		write!(output, "\t<text x=\"3\" y=\"14\" style=\"font-size:13px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-condensed;text-anchor:start;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">BU</text>\n").unwrap();
	}

	fn generate_nt_ce_counter_svg_elements(&mut self, mut output: &std::fs::File) {
		let color = &self.colors.text;
		
		write!(output, "\t<text x=\"3\" y=\"14\" style=\"font-size:13px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-condensed;text-anchor:start;fill:{color};fill-opacity:1;font-family:{FONT_MAIN}\">CE</text>\n").unwrap();
	}
	
	fn generate_sniper_svg_elements(&mut self, mut output: &std::fs::File, size: u32) {
		let mut column1 = 6;
		let mut column2 = 30;
		let mut column3 = 54;
		let mut row1 = 17;
		let mut row2 = 50;
		let mut font_size = 12;
		
		if 60 == size {
			write!(output, "\t<path id=\"Hexagon\" style=\"fill:none;stroke:{0};stroke-width:1.5;stroke-opacity:{1:.2}\" d=\"M 43.045125,7.4246216 56.100305,30.06722 43.01883,52.694636 16.882175,52.679455 3.8269945,30.036857 16.90847,7.4094401 Z\"/>\n", self.colors.text, self.overrides.opacity).unwrap();
		} else {
			column1 = 6;
			column2 = 24;
			column3 = 42;
			row1 = 13;
			row2 = 41;
			font_size = 9;			

			write!(output, "\t<path id=\"Hexagon\" style=\"fill:none;stroke:{0};stroke-width:1.5;stroke-opacity:{1:.2}\" transform=\"scale(0.80)\" d=\"M 43.045125,7.4246216 56.100305,30.06722 43.01883,52.694636 16.882175,52.679455 3.8269945,30.036857 16.90847,7.4094401 Z\"/>\n", self.colors.text, self.overrides.opacity).unwrap();
		}

		write!(output, "\t<text x=\"{column2}\" y=\"{row1}\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:{1};{FONT_MAIN}\">1</text>\n", self.colors.text, self.overrides.opacity).unwrap();
		write!(output, "\t<text x=\"{column3}\" y=\"{row1}\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:{1};{FONT_MAIN}\">2</text>\n", self.colors.text, self.overrides.opacity).unwrap();
		write!(output, "\t<text x=\"{column3}\" y=\"{row2}\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:{1};{FONT_MAIN}\">3</text>\n", self.colors.text, self.overrides.opacity).unwrap();
		write!(output, "\t<text x=\"{column2}\" y=\"{row2}\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:{1};{FONT_MAIN}\">4</text>\n", self.colors.text, self.overrides.opacity).unwrap();
		write!(output, "\t<text x=\"{column1}\" y=\"{row2}\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:{1};{FONT_MAIN}\">5</text>\n", self.colors.text, self.overrides.opacity).unwrap();
		write!(output, "\t<text x=\"{column1}\" y=\"{row1}\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:{1};{FONT_MAIN}\">6</text>\n", self.colors.text, self.overrides.opacity).unwrap();
		
		if "ff" == self.nationality {
			generate_svg_start_element(output, 1, COL_X_POSITION_SNIPER, COL_Y_POSITION, COL_WIDTH, COL_HEIGHT, "Cross of Lorraine", &"yellow".to_string());
			write!(output, "\t\t<image id=\"Cross of Lorraine\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" style=\"opacity:{1}\" href=\"{0}\" xlink:href=\"{0}\"/>\n", "./svg/CoL.svg", self.overrides.opacity).unwrap();
			write!(output, "\t</svg>\n").unwrap();			
		}		
	}
	
	fn generate_armor_leader_back_svg_elements(&mut self, mut output: &std::fs::File) {
		write!(output, "\t<text x=\"50%\" y=\"12.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1.0;font-family:{FONT_MAIN}\">TH DR, ML</text>\n", self.colors.text).unwrap();
		write!(output, "\t<text x=\"50%\" y=\"22.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1.0;font-family:{FONT_MAIN}\">OVR, CC</text>\n", self.colors.text).unwrap();
		write!(output, "\t<text x=\"50%\" y=\"32.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1.0;font-family:{FONT_MAIN}\">HD Mnvr</text>\n", self.colors.text).unwrap();
		write!(output, "\t<text x=\"50%\" y=\"42.00\" style=\"font-size:8.5px;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1.0;font-family:{FONT_MAIN}\">Bog Rmvl</text>\n", self.colors.text).unwrap();
	}
	
	fn generate_armor_leader_counter_svg_elements(&mut self, mut output: &std::fs::File) {
		write!(output, "\t<text id=\"Values\" x=\"50%\" y=\"44\" style=\"font-size:12px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.values).unwrap();
	}

	fn generate_sniper_back_svg_elements(&mut self, mut output: &std::fs::File) {
		write!(output, "\t<text x=\"30\" y=\"19\" style=\"font-size:13px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:red;fill-opacity:1;{FONT_MAIN}\">Pin: 3 DR</text>\n").unwrap();
		write!(output, "\t<text x=\"30\" y=\"34\" style=\"font-size:13px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:red;fill-opacity:1;{FONT_MAIN}\">K: &#8804;2 DR</text>\n").unwrap();
		write!(output, "\t<text x=\"30\" y=\"49\" style=\"font-size:13px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:semi-expanded;text-anchor:middle;fill:red;fill-opacity:1;{FONT_MAIN}\">No Attack</text>\n").unwrap();		
	}
	
	fn generate_broken_leader_counter_svg_elements(&mut self, mut output: &std::fs::File) {
		let size = BROKEN_MORALE_SIZE - (2.0 * BROKEN_MORALE_STROKE_WIDTH);
		let mut font_size = BROKEN_MORALE_FONT_SIZE;
		let mut font_stretch = "font-stretch:semi-expanded";
		
		if "10" == self.values {
			font_size -= 1.0;
			font_stretch = "font-stretch:condensed"
		}
		
		generate_svg_start_element(output, 1, BROKEN_MORALE_X_POSITION, BROKEN_MORALE_Y_POSITION, BROKEN_MORALE_SIZE, BROKEN_MORALE_SIZE, "Morale", &"yellow".to_string());
		
		write!(output, "\t\t<rect id=\"Self rally\" x=\"1\" y=\"1\" width=\"{size}\" height=\"{size}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{0};stroke-width:{1};stroke-dasharray:none;stroke-opacity:1\"/>\n", self.colors.text, BROKEN_MORALE_STROKE_WIDTH).unwrap();
		write!(output, "\t<text id=\"Morale\" x=\"50%\" y=\"80%\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};{font_stretch};text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.values).unwrap();
		
		write!(output, "\t</svg>\n").unwrap();
	}
	
	fn generate_leader_counter_svg_elements(&mut self, mut output: &std::fs::File) {
		let mut font_size = 12.0;
		let mut y_position: f64 = 24.0;
		
		if self.assault_engineer || self.wounded {
			y_position = 28.0;
		}
		
		if "(1)-0-8" == self.values && self.wounded {
			font_size = 9.0;
		}

		if "ff" == self.nationality {
			generate_svg_start_element(output, 1, COL_X_POSITION, COL_Y_POSITION, COL_WIDTH, COL_HEIGHT, "Cross of Lorraine", &"yellow".to_string());
			write!(output, "\t\t<image id=\"Cross of Lorraine\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{0}\" xlink:href=\"{0}\"/>\n", "./svg/CoL.svg").unwrap();
			write!(output, "\t</svg>\n").unwrap();			
		}
		
		if self.assault_engineer {
			generate_svg_start_element(output, 1, AE_X_POSITION, AE_Y_POSITION, AE_WIDTH, AE_HEIGHT, "Assault Engineer", &"red".to_string());
			write!(output, "\t\t<image id=\"Assault Engineer\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{0}\" xlink:href=\"{0}\"/>\n", "./svg/dc.svg").unwrap();
			write!(output, "\t</svg>\n").unwrap();
		}

		if self.piece.contains("PO") { // Special case for Communist Chinese Political Officers.
			write!(output, "\t<text id=\"Values\" transform=\"translate(44,{y_position}) rotate(-90)\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:red;fill-opacity:1;stroke:black;stroke-width:0.5;stroke-opacity:1;{FONT_MAIN}\">{0}</text>\n", self.values).unwrap();
		} else {
			write!(output, "\t<text id=\"Values\" transform=\"translate(44,{y_position}) rotate(-90)\" style=\"font-size:{font_size}px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.values).unwrap();
		}
		
		if self.wounded {
			write!(output, "\t<text id=\"Wounded\" x=\"46\" y=\"9\" style=\"font-size:8px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};text-anchor:end;fill:{0};fill-opacity:1;{FONT_MAIN}\">3MF</text>\n", self.colors.text).unwrap();
		}
	}
}

fn generate_svg_counter(record: &mut Record) {
	let path = format!("{0}{1}/", record.args.destination, record.nationality);
	
	if record.overrides.copy {
		let _ = copy_counter("", &record.nationality, &record.piece, &"".to_string(), &record.args);
	} else {
		let size: u32 = if 0 != record.overrides.counter_size { record.overrides.counter_size } else { 48 };
		
		if !record.args.quiet {
			print!("Generating '{0}.svg' ...", record.piece);
		} else {
			println!("{0}", record.piece);
		}
		//
		// Create the counter file.
		//
		let output = match open_counter_file(&path, &record.piece) {
			Err(why) => panic!("couldn't create file: {0} {1}", record.piece, why),
			Ok(output) => output,
		};

		generate_counter_header_svg_elements("vasl_singleman_counters", &output, size, &record.piece, &"".to_string(), &record.comments, &record.version);
		generate_counter_background_svg(&output, size, &record.colors, &record.overrides);
		generate_debug_working_area_svg(&output);

		record.generate_unit_depiction_svg_elements(&output, &path, size);

		match record.values.as_str() {
			"CrewPass" => {
				record.generate_crew_pass_counter_svg_elements(&output);
			}
			"NtBu" => {
				record.generate_nt_bu_counter_svg_elements(&output);
			}
			"NtCe" => {
				record.generate_nt_ce_counter_svg_elements(&output);
			}
			"Sniper" => {
				record.generate_sniper_svg_elements(&output, size);
			}
			"_aleader" => {
				record.generate_armor_leader_back_svg_elements(&output);
			}
			"_sniper" => {
				record.generate_sniper_back_svg_elements(&output);
			}
			&_ => {
				if record.armor_leader {
					record.generate_armor_leader_counter_svg_elements(&output);
				} else if record.broken {
					record.generate_broken_leader_counter_svg_elements(&output);
				} else {
					record.generate_leader_counter_svg_elements(&output);
				}
			}
		}

		generate_footer_svg(&output);

		drop(output);
		
		if !record.args.quiet {
			println!(" done.");
		}
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
	nationality: String,
	values: String,
	armor: String,
	assault_engineer: String,
	broken: String,
	wounded: String,
	piece: String,
	version: String,
	overrides: String,
	svg_image_transform: String,
	comments: String,
}

impl SpreadsheetRecord {
	fn sanitize(&mut self, nat: &String, args: &Arguments) -> Record {
		let mut result: Record = Default::default();
		
		result.nationality = if nat.is_empty() { self.nationality.to_string() } else { nat.to_string() };
		
		result.args = args.clone();
		
		result.overrides.sanitize(&self.overrides);
		
		result.values = self.values.to_string();
		
		result.armor_leader = "yes" == self.armor;
		
		result.assault_engineer = "yes" == self.assault_engineer;
		
		result.broken = "yes" == self.broken;
		
		result.wounded = "yes" == self.wounded;
		
		result.pieces = extract_vector(&self.piece, OVERRIDE_DELIMITER);
		
		result.version = self.version.to_string();
		
		result.colors = nationality_to_colors(&result.nationality);
		
		result.svg_image_transform = self.svg_image_transform.to_string();
		
		result.comments = self.comments.to_string();
		
		return result;
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let mut args = Arguments::parse();
	
	args.sanitize_destination();
	
	let mut rdr = csv::Reader::from_reader(io::stdin());

	for result in rdr.deserialize() {
		let mut spreadsheet_record: SpreadsheetRecord = result?;

		if !spreadsheet_record.nationality.is_empty() && !spreadsheet_record.overrides.contains(NOVR_IGNORE) {
			let mut record: Record = spreadsheet_record.sanitize(&"".to_string(), &args);
			let pieces = record.pieces.clone();
			
			for piece in pieces {
				if !piece.contains('@') {
					record.piece = piece.to_string();
					generate_svg_counter(&mut record);
				} else {
					let (piece, nationality) = piece.split_once("@").unwrap();
					let mut alt_record: Record = spreadsheet_record.sanitize(&nationality.to_string(), &args);
					
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
