use std::io::prelude::*;
use std::path::Path;
use std::{error::Error, io, process};
use std::fmt;
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

pub const STRENGTH_FACTORS_X_POSITION: f64 =	 3.0;
pub const STRENGTH_FACTORS_Y_POSITION: f64 =	32.0;
pub const STRENGTH_FACTORS_Y_BASELINE: f64 =	11.0;
pub const STRENGTH_FACTORS_HEIGHT: f64 =		16.0;
pub const STRENGTH_FACTORS_FONT_SIZE: f64 =		12.0;
pub const STRENGTH_FACTORS_SUP_FONT_SIZE: f64 =	 8.0;

pub const CLASS_X_POSITION: f64 =	 	35.0;
pub const CLASS_Y_POSITION: f64 =		 2.0;
pub const CLASS_SIZE: f64 =				11.0;
pub const CLASS_FONT_SIZE: f64 =		 9.0;
pub const CLASS_STROKE_WIDTH: f64 =		 0.75;
pub const CLASS_CIRCLE_RADIUS: f64 =	 4.5;

pub const AE_X_POSITION: f64 =	 	 2.0;
pub const AE_Y_POSITION: f64 =		 2.0;
pub const AE_HEIGHT: f64 =			15.0;
pub const AE_WIDTH: f64 =			10.0;

pub const BROKEN_MORALE_X_POSITION: f64 =	 	30.0;
pub const BROKEN_MORALE_Y_POSITION: f64 =		30.0;
pub const BROKEN_MORALE_SIZE: f64 =				16.0;
pub const BROKEN_MORALE_FONT_SIZE: f64 =		14.0;
pub const BROKEN_MORALE_STROKE_WIDTH: f64 =		 0.75;

#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub enum ClassIdentifier {
	#[default]
	None,
	Shutzstaffel,
	AssaultEngineer1,	// "AE"
	AssaultEngineer2,	// "A"
	Engineer,
	BoxedEngineer,
	CircledEngineer,
	Firstline,
	BoxedFirstline,
	CircledFirstline,
	Secondline,
	BoxedSecondline,
	Green,
	Conscript,
	Crew,
	Paratroop,
	Nkvd,
	Marines,
	BoxedMarines,
	CircledMarines,
	ParaMarines,
	Pegasus,
	BoxedPegasus,
	FirstAirborne,
	BoxedFirstAirborne,
	Broken,
	Cloaking,
	Concealment,
	LargeConcealment,
	MiniConcealment,
	Roi,
}

impl fmt::Display for ClassIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClassIdentifier::None => write!(f, "None"),
            ClassIdentifier::Shutzstaffel => write!(f, "Shutzstaffel"),
            ClassIdentifier::AssaultEngineer1 => write!(f, "AssaultEngineer - AE"),
            ClassIdentifier::AssaultEngineer2 => write!(f, "AssaultEngineer - A"),
            ClassIdentifier::Engineer => write!(f, "Engineer"),
			ClassIdentifier::BoxedEngineer => write!(f, "BoxedEngineer"),
			ClassIdentifier::CircledEngineer => write!(f, "CircledEngineer"),
			ClassIdentifier::Firstline => write!(f, "Firstline"),
			ClassIdentifier::BoxedFirstline => write!(f, "BoxedFirstline"),
			ClassIdentifier::CircledFirstline => write!(f, "CircledFirstline"),
			ClassIdentifier::Secondline => write!(f, "Secondline"),
			ClassIdentifier::BoxedSecondline => write!(f, "BoxedSecondline"),
			ClassIdentifier::Green => write!(f, "Green"),
			ClassIdentifier::Conscript => write!(f, "Conscript"),
			ClassIdentifier::Crew => write!(f, "Crew"),
			ClassIdentifier::Paratroop => write!(f, "Paratroop"),
			ClassIdentifier::Nkvd => write!(f, "Nkvd"),
			ClassIdentifier::Marines => write!(f, "Marines"),
			ClassIdentifier::BoxedMarines => write!(f, "BoxedMarines"),
			ClassIdentifier::CircledMarines => write!(f, "CircledMarines"),
			ClassIdentifier::ParaMarines => write!(f, "ParaMarines"),
			ClassIdentifier::Pegasus => write!(f, "Pegasus"),
			ClassIdentifier::BoxedPegasus => write!(f, "BoxedPegasus"),
			ClassIdentifier::FirstAirborne => write!(f, "FirstAirborne"),
			ClassIdentifier::BoxedFirstAirborne => write!(f, "BoxedFirstAirborne"),
			ClassIdentifier::Broken => write!(f, "Broken"),
			ClassIdentifier::Cloaking => write!(f, "Cloaking"),
			ClassIdentifier::Concealment => write!(f, "Concealment"),
			ClassIdentifier::LargeConcealment => write!(f, "LargeConcealment"),
			ClassIdentifier::MiniConcealment => write!(f, "MiniConcealment"),
			ClassIdentifier::Roi => write!(f, "Roi"),
        }
    }
}

#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub enum UnitType {
	#[default]
	None,
	Squad,
	StripedSquad,
	Halfsquad,
	Crew,
	StripedCrew,
	ShoreFireControlParty,
}

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitType::None => write!(f, "No unit type"),
            UnitType::Squad => write!(f, "Squad"),
            UnitType::StripedSquad => write!(f, "Striped Squad"),
            UnitType::Halfsquad => write!(f, "Halfsquad"),
            UnitType::Crew => write!(f, "Crew"),
            UnitType::StripedCrew => write!(f, "Striped Crew"),
            UnitType::ShoreFireControlParty => write!(f, "Shore Fire Control Party"),
        }
    }
}
//
// Sanitized and parsed aircraft-specific record fields.
//
#[derive(Default)]
struct Record {
	destination: String,
	nationality: String,
	class: ClassIdentifier,
	firepower: usize,
	range: usize,
	morale: usize,
	broken_morale: usize,
	assault_fire: bool,
	spraying_fire: bool,
	elr: bool,
	smoke: usize,
	self_rally: bool,
	assault_engineer: bool,
	unit: UnitType,
	bpv: usize,
	pieces: Vec<std::string::String>,	// Possibly multiple pieces per entry.
	piece: String,						// Each individual piece from "pieces" above.
	image: String,
	version: String,
	overrides: Overrides,
	colors: Colors,
	svg_image_transform: String,
	comments: String,
}

impl Record {
	fn sanitize_class_identifier(&mut self, class: &str) {
		match class {
			"ss" => {
				self.class = ClassIdentifier::Shutzstaffel;
			}
			"AE" => {
				self.class = ClassIdentifier::AssaultEngineer1;
			}
			"A"  => {
				self.class = ClassIdentifier::AssaultEngineer2;
			}		
			"E" => {			
				self.class = ClassIdentifier::Engineer;
			}
			"[E]" => {			
				self.class = ClassIdentifier::BoxedEngineer;
			}
			"(E)" => {			
				self.class = ClassIdentifier::CircledEngineer;
			}
			"1" => {			
				self.class = ClassIdentifier::Firstline;
			}
			"[1]" => {			
				self.class = ClassIdentifier::BoxedFirstline;
			}
			"(1)" => {			
				self.class = ClassIdentifier::CircledFirstline; 
			}
			"2" => {			
				self.class = ClassIdentifier::Secondline;
			}
			"[2]" => {			
				self.class = ClassIdentifier::BoxedSecondline;
			}
			"G" => {			
				self.class = ClassIdentifier::Green;
			}
			"C" => {			
				self.class = ClassIdentifier::Conscript;
			}
			"crew" => {			
				self.class = ClassIdentifier::Crew;
			}			
			"P" => {			
				self.class = ClassIdentifier::Paratroop;
			}
			"NKVD" => {			
				self.class = ClassIdentifier::Nkvd;
			}
			"M" => {			
				self.class = ClassIdentifier::Marines;
			}
			"[M]" => {			
				self.class = ClassIdentifier::BoxedMarines;
			}
			"(M)" => {			
				self.class = ClassIdentifier::CircledMarines;
			}
			"(M)P" => {			
				self.class = ClassIdentifier::ParaMarines;
			}
			"pegasus" => {			
				self.class = ClassIdentifier::Pegasus;
			}
			"[pegasus]" => {			
				self.class = ClassIdentifier::BoxedPegasus;
			}
			"first_ab" => {			
				self.class = ClassIdentifier::FirstAirborne;
			}
			"[first_ab]" => {			
				self.class = ClassIdentifier::BoxedFirstAirborne;
			}			
			"broken" => {
				self.class = ClassIdentifier::Broken;
				
			}
			"?C" => {
				self.class = ClassIdentifier::Cloaking;
				
			}
			"?" => {
				self.class = ClassIdentifier::Concealment;
				
			}
			"?58" => {
				self.class = ClassIdentifier::LargeConcealment;
				
			}
			"?me" => {
				self.class = ClassIdentifier::MiniConcealment;
				
			}
			"?roi" => {
				self.class = ClassIdentifier::Roi;
				
			}
			&_ => {
				panic!("sanitize_class_identifier() - '{class}' unrecognized value!");
			}
		}
	}	
	
	fn sanitize_morale(&mut self, morale: &str) {
		if morale.contains("/") {
			let (left, right) = morale.split_once('/').unwrap();

			self.morale = left.parse::<usize>().unwrap_or(0);
			self.broken_morale = right.parse::<usize>().unwrap_or(0);
		} else {
			self.morale = morale.parse::<usize>().unwrap_or(0);
			self.broken_morale = self.morale;
		}
	}

	fn sanitize_unit_type(&mut self, unit: &str) {
		match unit {
			"squad" => {
				self.unit = UnitType::Squad;
			}
			"stripedsquad" => {
				self.unit = UnitType::StripedSquad;
			}
			"half" => {
				self.unit = UnitType::Halfsquad;
			}
			"crew" => {
				self.unit = UnitType::Crew;
			}
			"stripedcrew" => {
				self.unit = UnitType::StripedCrew;
			}
			"sfcparty" => {
				self.unit = UnitType::ShoreFireControlParty;
			}
			"na" => {
				self.unit = UnitType::None;
			}
			&_ => {
				panic!("sanitize_unit_type() - '{unit}' unrecognized value!");
			}
		}
	}

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
	
					write!(counter_file, "\t<!-- Silhouettes -->\n").unwrap();
					write!(counter_file, "\t<image x=\"0\" y=\"0\" width=\"48\" height=\"48\" preserveAspectRatio=\"xMidYMid meet\" transform=\"{transform}\" href=\"{path}\" xlink:href=\"{path}\"/>\n").unwrap();
					break;	// Our work here is done.
				}
			}
		}
	}

	fn generate_broken_unit_depiction_svg_elements(&mut self, mut counter_file: &std::fs::File, root_path: &String) {
		if INCLUDE_IMAGES {
			let path_prefix = "svg/";
			let file_type_svg = ".svg";
			let file_type_png = ".png";
			let filename: String = self.image.to_string();
			let paths: Vec<std::string::String> = [
				format!("{}{}{}", path_prefix, &filename, file_type_svg),
				format!("{}{}{}", path_prefix, &filename, file_type_png)
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
	
	fn generate_strength_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		write!(counter_file, "\t<text x=\"11\" y=\"44\" id=\"Firepower\" style=\"font-size:12px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.firepower).unwrap();
		
		if self.assault_fire {
			write!(counter_file, "\t<line y2=\"46\" id=\"AssaultFire\" style=\"stroke:{0}; stroke-width:1.25\" x1=\"7.75\" x2=\"14.25\" y1=\"46\"/>\n", self.colors.text).unwrap();
		}

		write!(counter_file, "\t<line y2=\"41\" id=\"Dash1\" style=\"stroke:{0}; stroke-width:1.5\" x1=\"15.5\" x2=\"19.5\" y1=\"41\"/>\n", self.colors.text).unwrap();

		if 0 != self.smoke {
			write!(counter_file, "\t<text x=\"17.25\" y=\"39.5\" id=\"SmokeExponent\" style=\"font-size:8px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.smoke).unwrap();
		}
		
		write!(counter_file, "\t<text x=\"24\" y=\"44\" id=\"Range\" style=\"font-size:12px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.range).unwrap();
		
		if self.spraying_fire {
			write!(counter_file, "\t<line y2=\"46\" id=\"SprayingFire\" style=\"stroke:{0}; stroke-width:1.25\" x1=\"20.75\" x2=\"27.25\" y1=\"46\"/>\n", self.colors.text).unwrap();
		}
		
		write!(counter_file, "\t<line y2=\"41\" id=\"dash2\" style=\"stroke:{0}; stroke-width:1.5\" x1=\"28.5\" x2=\"32.5\" y1=\"41\"/>\n", self.colors.text).unwrap();
		
		write!(counter_file, "\t<text x=\"37\" y=\"44\" id=\"Morale\" style=\"font-size:12px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.morale).unwrap();
		
		if self.elr {
			write!(counter_file, "\t<line y2=\"46\" id=\"ELR\" style=\"stroke:{0}; stroke-width:1.25\" x1=\"33.75\" x2=\"40.25\" y1=\"46\"/>\n", self.colors.text).unwrap(); 
		}
	}

	fn generate_class_box_svg(&mut self, mut counter_file: &std::fs::File, size: f64) {
		write!(counter_file, "\t\t<rect x=\"1\" y=\"1\" width=\"{size}\" height=\"{size}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{0};stroke-width:{1};stroke-dasharray:none;stroke-opacity:1\"/>\n", self.colors.text, CLASS_STROKE_WIDTH).unwrap();
	}
	
	fn generate_class_circle_svg(&mut self, mut counter_file: &std::fs::File) {
		write!(counter_file, "\t\t<circle cx=\"50%\" cy=\"50%\" r=\"{0}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{1};stroke-width:{2};stroke-dasharray:none;stroke-opacity:1\"></circle>\n", CLASS_CIRCLE_RADIUS, self.colors.text, CLASS_STROKE_WIDTH).unwrap();
	}

	fn generate_class_text_svg(&mut self, mut counter_file: &std::fs::File, text: &str) {
		write!(counter_file, "\t\t<text x=\"50%\" y=\"83%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{0:.2}px;font-style:normal;font-variant:normal;font-stretch:normal;font-weight:{1};font-family:{2};fill:{3};fill-opacity:1;stroke:none;stroke-width:0.2\">{4}</tspan></text>\n", CLASS_FONT_SIZE, FONT_WEIGHT_BOLD, FONT_MAIN, self.colors.text, text).unwrap();
	}

	fn generate_class_image_svg(&mut self, mut counter_file: &std::fs::File, source: &str) {
		write!(counter_file, "\t\t<image x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{source}\" xlink:href=\"{source}\"/>\n").unwrap();
	}
	
	fn generate_modified_class_image_svg(&mut self, mut counter_file: &std::fs::File, source: &str, x_pos: f64, y_pos: f64, width: f64, height: f64) {
	write!(counter_file, "\t\t<image x=\"{x_pos}\" y=\"{y_pos}\" width=\"{width}%\" height=\"{height}%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{source}\" xlink:href=\"{source}\"/>\n").unwrap();
	}	
	
	fn generate_paramarines_image_svg(&mut self, mut counter_file: &std::fs::File) {
		let source = "./svg/class_para.svg";
		
		write!(counter_file, "\t\t<image x=\"35.00\" y=\"23.00\" width=\"11\" height=\"11\" preserveAspectRatio=\"xMidYMid meet\" href=\"{source}\" xlink:href=\"{source}\"/>\n").unwrap();
	}
	
	fn generate_class_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		let size = CLASS_SIZE - 1.75;
		
		generate_svg_start_element(counter_file, 1, CLASS_X_POSITION, CLASS_Y_POSITION, CLASS_SIZE, CLASS_SIZE, "Class", &"red".to_string());
		
		match self.class {
			ClassIdentifier::None => {
			}
			ClassIdentifier::Shutzstaffel => {
				self.generate_class_image_svg(&counter_file, "./svg/class_ss.svg");
			}
			ClassIdentifier::AssaultEngineer1 => {
				self.generate_class_text_svg(&counter_file, "AE");
			}
			ClassIdentifier::AssaultEngineer2 => {
				self.generate_class_text_svg(&counter_file, "A");
			}
			ClassIdentifier::Engineer => {
				self.generate_class_text_svg(&counter_file, "E");
			}
			ClassIdentifier::BoxedEngineer => {
				self.generate_class_box_svg(&counter_file, size);
				self.generate_class_text_svg(&counter_file, "E");
			}
			ClassIdentifier::CircledEngineer => {
				self.generate_class_circle_svg(&counter_file);
				self.generate_class_text_svg(&counter_file, "E");
			}
			ClassIdentifier::Firstline => {
				self.generate_class_text_svg(&counter_file, "1");
			}
			ClassIdentifier::BoxedFirstline => {
				self.generate_class_box_svg(&counter_file, size);
				self.generate_class_text_svg(&counter_file, "1");
			}
			ClassIdentifier::CircledFirstline => {
				self.generate_class_circle_svg(&counter_file);
				self.generate_class_text_svg(&counter_file, "1");
			}
			ClassIdentifier::Secondline => {
				self.generate_class_text_svg(&counter_file, "2");
			}
			ClassIdentifier::BoxedSecondline => {
				self.generate_class_box_svg(&counter_file, size);
				self.generate_class_text_svg(&counter_file, "2");
			}
			ClassIdentifier::Green => {
				self.generate_class_text_svg(&counter_file, "G");
			}
			ClassIdentifier::Conscript => {
				self.generate_class_text_svg(&counter_file, "C");
			}
			ClassIdentifier::Paratroop => {
				self.generate_class_image_svg(&counter_file, "./svg/class_para.svg");
			}
			ClassIdentifier::Nkvd => {
				self.generate_class_image_svg(&counter_file, "./svg/class_nkvd.svg");
			}
			ClassIdentifier::Marines => {
				self.generate_class_image_svg(&counter_file, "./svg/class_marines.svg");
			}
			ClassIdentifier::BoxedMarines => {
				self.generate_class_box_svg(&counter_file, size);
				self.generate_modified_class_image_svg(&counter_file, "./svg/class_marines.svg", 1.75, 1.75, 70.00, 70.00);
			}
			ClassIdentifier::CircledMarines | ClassIdentifier::ParaMarines => {
				self.generate_class_circle_svg(&counter_file);
				self.generate_modified_class_image_svg(&counter_file, "./svg/class_marines.svg", 1.75, 1.75, 70.00, 70.00);
			}
			
			ClassIdentifier::Pegasus => {
				self.generate_class_image_svg(&counter_file, "./svg/class_pegasus.svg");
			}
			ClassIdentifier::BoxedPegasus => {
				self.generate_class_box_svg(&counter_file, size);
				self.generate_modified_class_image_svg(&counter_file, "./svg/class_pegasus.svg", 1.75, 1.75, 70.00, 70.00);
			}
			ClassIdentifier::FirstAirborne => {
				self.generate_class_image_svg(&counter_file, "./svg/class_first_airborne.svg");
			}
			ClassIdentifier::BoxedFirstAirborne => {
				self.generate_class_box_svg(&counter_file, size);
				self.generate_modified_class_image_svg(&counter_file, "./svg/class_class_first_airborne", 1.75, 1.75, 70.00, 70.00);
			}
			
			_ => {
			}			
		}

		write!(counter_file, "\t</svg>\n").unwrap();
		
		if self.assault_engineer {
			generate_svg_start_element(counter_file, 1, AE_X_POSITION, AE_Y_POSITION, AE_WIDTH, AE_HEIGHT, "Assault Engineer", &"red".to_string());
			write!(counter_file, "\t\t<image x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{0}\" xlink:href=\"{0}\"/>\n", "./svg/dc.svg").unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();
		}

		if ClassIdentifier::ParaMarines == self.class {
			self.generate_paramarines_image_svg(&counter_file);
		}
	}

	fn generate_broken_morale_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		let size = BROKEN_MORALE_SIZE - (2.0 * BROKEN_MORALE_STROKE_WIDTH);
		
		generate_svg_start_element(counter_file, 1, BROKEN_MORALE_X_POSITION, BROKEN_MORALE_Y_POSITION, BROKEN_MORALE_SIZE, BROKEN_MORALE_SIZE, "Morale", &"yellow".to_string());

		if self.self_rally {
			write!(counter_file, "\t\t<rect id=\"Self rally\" x=\"1\" y=\"1\" width=\"{size}\" height=\"{size}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{0};stroke-width:{1};stroke-dasharray:none;stroke-opacity:1\"/>\n", self.colors.text, BROKEN_MORALE_STROKE_WIDTH).unwrap();
		}
		
		write!(counter_file, "\t<text id=\"Morale\" x=\"50%\" y=\"80%\" style=\"font-size:{BROKEN_MORALE_FONT_SIZE}px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.broken_morale).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}
}

fn generate_svg_counter(record: &mut Record) {
	let path = &record.destination.to_string();
	let size = if ClassIdentifier::LargeConcealment == record.class { 58 } else { 48 };
	//
	// Create the counter file.
	//
	let mut counter_file = match open_counter_file(&path, &record.piece) {
		Err(why) => panic!("couldn't create file: {0} {1}", record.piece, why),
		Ok(counter_file) => counter_file,
	};

	generate_counter_header_svg_elements("vasl_multiman_counters", &counter_file, size, &record.piece, &record.comments, &record.version);

	write!(counter_file, "\t<svg width=\"{0:.2}\" height=\"{0:.2}\" viewBox=\"0 0 1000 1000\">\n", size).unwrap();

	generate_counter_background_svg(&counter_file, &record.colors, &record.overrides);

	write!(counter_file, "\t</svg>\n").unwrap();

	generate_debug_working_area_svg(&counter_file);
	
	match record.class {
		ClassIdentifier::Broken => {
			generate_broken_counter_svg_elements(&counter_file, record, &path);
		}
		ClassIdentifier::Cloaking => {
			generate_cloaking_counter_svg_elements(&counter_file, record);
		}
		ClassIdentifier::Concealment => {
			generate_concealment_counter_svg_elements(&counter_file, record);
		}
		ClassIdentifier::LargeConcealment => {
			generate_large_concealment_counter_svg_elements(&counter_file, record);
		}
		ClassIdentifier::MiniConcealment => {
			generate_mini_concealment_counter_svg_elements(&counter_file, record);
		}
		ClassIdentifier::Roi => {
			generate_roi_counter_svg_elements(&counter_file, record);
		}
		_ => {
			generate_multiman_counter_svg_elements(&counter_file, record, &path);
		}
	}

	generate_footer_svg(&counter_file);

	drop(counter_file);
}

fn generate_mini_concealment_svg_counter(record: &mut Record) {
	let path = &record.destination.to_string();
	let size = 48;
	let mini_size = 24;
	//
	// Create the counter file.
	//
	let mut counter_file = match open_counter_file(&path, &record.piece) {
		Err(why) => panic!("couldn't create file: {0} {1}", record.piece, why),
		Ok(counter_file) => counter_file,
	};

	generate_counter_header_svg_elements("vasl_multiman_counters", &counter_file, size, &record.piece, &record.comments, &record.version);

	write!(counter_file, "\t<svg width=\"{0:.2}\" height=\"{0:.2}\" viewBox=\"0 0 1000 1000\">\n", mini_size).unwrap();

	generate_counter_background_svg(&counter_file, &record.colors, &record.overrides);

	write!(counter_file, "\t</svg>\n").unwrap();

	generate_debug_working_area_svg(&counter_file);
	generate_mini_concealment_counter_svg_elements(&counter_file, record);
	generate_footer_svg(&counter_file);

	drop(counter_file);
}

fn generate_multiman_counter_svg_elements(mut counter_file: &std::fs::File, record: &mut Record, path: &String) {
	record.generate_unit_depiction_svg_elements(&counter_file, &path);
	
	if record.unit == UnitType::StripedSquad || record.unit == UnitType::StripedCrew {
		write!(counter_file, "\t<rect x=\"0\" y=\"34\" width=\"48\" height=\"11\" style=\"display:inline;fill:red;fill-opacity:1.0;stroke:none;stroke-width:0;stroke-dasharray:none;stroke-opacity:1\"/>\n").unwrap();
	}
	
	record.generate_strength_svg_elements(&counter_file);
	record.generate_class_svg_elements(&counter_file);	
}

fn generate_broken_counter_svg_elements(counter_file: &std::fs::File, record: &mut Record, path: &String) {
	record.generate_broken_unit_depiction_svg_elements(&counter_file, &path);	
	record.generate_broken_morale_svg_elements(&counter_file);
}

fn generate_cloaking_counter_svg_elements(mut counter_file: &std::fs::File, record: &mut Record) {
	write!(counter_file, "\t<text x=\"50.00%\" y=\"65.00%\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:25.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">?</text>\n", record.colors.text).unwrap();
	write!(counter_file, "\t<text x=\"50.00%\" y=\"44.00\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:8.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">Cloak</text>\n", record.colors.text).unwrap();
}

fn generate_concealment_counter_svg_elements(mut counter_file: &std::fs::File, record: &mut Record) {
	write!(counter_file, "\t<text x=\"50.00%\" y=\"65.00%\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:25.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">?</text>\n", record.colors.text).unwrap();
	write!(counter_file, "\t<text x=\"50.00%\" y=\"44.00\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:8.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">7 morale</text>\n", record.colors.text).unwrap();
}

fn generate_large_concealment_counter_svg_elements(mut counter_file: &std::fs::File, record: &mut Record) {
	write!(counter_file, "\t<text x=\"50.00%\" y=\"60.00%\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:30.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">?</text>\n", record.colors.text).unwrap();
	write!(counter_file, "\t<text x=\"50.00%\" y=\"55.00\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:12.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">7 morale</text>\n", record.colors.text).unwrap();
}

fn generate_mini_concealment_counter_svg_elements(mut counter_file: &std::fs::File, record: &mut Record) {
	write!(counter_file, "\t<text x=\"25.00%\" y=\"35.00%\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:12.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">?</text>\n", record.colors.text).unwrap();
}

fn generate_roi_counter_svg_elements(mut counter_file: &std::fs::File, record: &mut Record) {
	write!(counter_file, "\t<text x=\"50.00%\" y=\"65.00%\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:25.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">?</text>\n", record.colors.text).unwrap();
	write!(counter_file, "\t<text x=\"50.00%\" y=\"44.00\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:8.00px;font-weight:bold;font-family:Nimbus Sans L;fill:{0}\">No ROI</text>\n", record.colors.text).unwrap();
}

fn generate_svg_counter_announcer(record: &mut Record) {
	print!("Generating '{0}.svg' ...", record.piece);
	
	if ClassIdentifier::MiniConcealment == record.class {
		generate_mini_concealment_svg_counter(record);
	} else {
		generate_svg_counter(record);
	}
	
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
	class: String,
	firepower: String,
	range: String,
	morale: String,
	assault_fire: String,
	smoke: String,
	spraying_fire: String,
	elr: String,
	self_rally: String,
	assault_engineer: String,
	size: String,
	bpv: String,
	piece: String,
	image: String,
	version: String,
	overrides: String,
	svg_image_transform: String,
	comments: String,
}
	
impl SpreadsheetRecord {
	fn sanitize(&mut self, nat: &String) -> Record {
		let mut result: Record = Default::default();
		
		result.nationality = if nat.is_empty() { self.nationality.to_string() } else { nat.to_string() };
		result.destination = format!("{0}{1}/", get_destination_arg(), result.nationality); // get_destination() ensures a trailing '/'
		result.overrides.sanitize(&self.overrides);
		result.sanitize_class_identifier(&self.class);
		result.firepower = self.firepower.parse::<usize>().unwrap_or(0);
		result.range = self.range.parse::<usize>().unwrap_or(0);
		result.sanitize_morale(&self.morale);
		result.assault_fire = "yes" == self.assault_fire;
		result.smoke = self.smoke.parse::<usize>().unwrap_or(0);
		result.spraying_fire = "yes" == self.spraying_fire;
		result.elr = "yes" == self.elr;
		result.self_rally = "yes" == self.self_rally;
		result.assault_engineer = "yes" == self.assault_engineer;
		result.sanitize_unit_type(&self.size);
		result.bpv = self.bpv.parse::<usize>().unwrap_or(0);
		
		result.pieces = extract_vector(&self.piece, OVERRIDE_DELIMITER);
		
		result.image = self.image.to_string();
		result.version = self.version.to_string();
		
		if !result.overrides.background_color.is_empty() {
			result.colors = nationality_to_colors(&result.overrides.background_color);
		} else {
			result.colors = nationality_to_colors(&result.nationality);
		}

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
					generate_svg_counter_announcer(&mut record);
				} else {
					let (piece, nationality) = piece.split_once("@").unwrap();
					let mut alt_record: Record = spreadsheet_record.sanitize(&nationality.to_string());
					
					alt_record.colors = nationality_to_colors(&nationality.to_string());
					alt_record.piece = piece.to_string();
					
					generate_svg_counter_announcer(&mut alt_record);
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
