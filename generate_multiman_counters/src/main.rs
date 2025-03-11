use std::io::prelude::*;
use std::path::Path;
use std::{error::Error, io, process};
use std::fmt;
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

pub const COL_X_POSITION: f64 =	 	 2.0;
pub const COL_Y_POSITION: f64 =		 2.0;
pub const COL_HEIGHT: f64 =			12.0;
pub const COL_WIDTH: f64 =			 8.0;

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
	CircledSecondline,
	Green,
	Conscript,
	BoxedConscript,
	CircledConscript,
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
	RoyalMarines,
	KoreanMarines,
	Fn16,
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
			ClassIdentifier::CircledSecondline => write!(f, "CircledSecondline"),
			ClassIdentifier::Green => write!(f, "Green"),
			ClassIdentifier::Conscript => write!(f, "Conscript"),
			ClassIdentifier::BoxedConscript => write!(f, "BoxedConscript"),
			ClassIdentifier::CircledConscript => write!(f, "CircledConscript"),
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
			ClassIdentifier::RoyalMarines => write!(f, "RoyalMarines"),
			ClassIdentifier::KoreanMarines => write!(f, "KoreanMarines"),
			ClassIdentifier::Fn16 => write!(f, "Fn16"),
			ClassIdentifier::Broken => write!(f, "Broken"),
			ClassIdentifier::Cloaking => write!(f, "Cloaking"),
			ClassIdentifier::Concealment => write!(f, "Concealment"),
			ClassIdentifier::LargeConcealment => write!(f, "LargeConcealment"),
			ClassIdentifier::MiniConcealment => write!(f, "MiniConcealment"),
			ClassIdentifier::Roi => write!(f, "Roi"),
        }
    }
}
//
// Sanitized and parsed multi-man counter-specific record fields.
//
#[derive(Default)]
struct Record {
	args: Arguments,
	nationality: String,
	class: ClassIdentifier,
	firepower: usize,
	range: String, // Thanks Communist Chinese Grenadier squads ...
	morale: usize,
	broken_morale: usize,
	assault_fire: bool,
	spraying_fire: bool,
	elr: bool,
	smoke: usize,
	self_rally: bool,
	assault_engineer: bool,
	bpv: usize,
	pieces: Vec<std::string::String>,	// Possibly multiple pieces per entry.
	piece: String,						// Each individual piece from "pieces" above.
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
			"(2)" => {			
				self.class = ClassIdentifier::CircledSecondline;
			}			
			"G" => {			
				self.class = ClassIdentifier::Green;
			}
			"C" => {			
				self.class = ClassIdentifier::Conscript;
			}
			"[C]" => {			
				self.class = ClassIdentifier::BoxedConscript;
			}
			"(C)" => {			
				self.class = ClassIdentifier::CircledConscript;
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
			"RM" => {
				self.class = ClassIdentifier::RoyalMarines;
			}
			"KMC" => {
				self.class = ClassIdentifier::KoreanMarines;
			}
			"Fn16" => {
				self.class = ClassIdentifier::Fn16;
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
			"none" => {
				self.class = ClassIdentifier::None;
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

	fn generate_unit_depiction_svg_elements(&mut self, mut counter_file: &std::fs::File, root_path: &String) {
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
			let filename: String = self.piece.to_string();
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
		let mut firepower_x = 11.00;
		let mut assault_fire_x = 7.75;
		let mut dash1_x = 15.50;
		let mut smoke_x = 17.25;
		let range_x = 24.00;
		let spraying_fire_x = 20.75;
		let mut dash2_x = 28.50;
		let mut morale_x = 37.00;
		let mut elr_x = 33.75;

		if self.range.contains('(') && self.range.contains(')') {
			let delta = 4.00;
			
			firepower_x -= delta;
			assault_fire_x -= delta;
			dash1_x -= delta;
			smoke_x -= delta;
			dash2_x += delta;
			morale_x += delta;
			elr_x += delta;			
		}
		
		if 0 != self.firepower && 0 != self.morale {
			write!(counter_file, "\t<text id=\"Firepower\" x=\"{firepower_x}\" y=\"43.00\" style=\"font-size:12px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.firepower).unwrap();
			
			if self.assault_fire {
				write!(counter_file, "\t<line id=\"AssaultFire\" x1=\"{0:.2}\" y1=\"45.00\" x2=\"{1:.2}\" y2=\"45.00\" style=\"stroke:{2}; stroke-width:1.25\"/>\n", assault_fire_x, assault_fire_x + 6.50, self.colors.text).unwrap();
			}

			write!(counter_file, "\t<line id=\"Dash1\" x1=\"{0:.2}\" y1=\"40.00\" x2=\"{1:.2}\" y2=\"40.00\" style=\"stroke:{2}; stroke-width:1.5\"/>\n", dash1_x, dash1_x + 4.00, self.colors.text).unwrap();

			if 0 != self.smoke {
				write!(counter_file, "\t<text id=\"SmokeExponent\" x=\"{smoke_x}\" y=\"38.50\" style=\"font-size:8px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:normal;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.smoke).unwrap();
			}
			
			write!(counter_file, "\t<text id=\"Range\" x=\"{range_x}\" y=\"43.00\" style=\"font-size:12px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.range).unwrap();
			
			if self.spraying_fire {
				write!(counter_file, "\t<line id=\"SprayingFire\" x1=\"{0:.2}\" y1=\"45.00\" x2=\"{1:.2}\" y2=\"45.00\" style=\"stroke:{2}; stroke-width:1.25\"/>\n", spraying_fire_x, spraying_fire_x + 6.50, self.colors.text).unwrap();
			}
			
			write!(counter_file, "\t<line id=\"dash2\" x1=\"{0:.2}\" y1=\"40.00\" x2=\"{1:.2}\" y2=\"40.00\" style=\"stroke:{2}; stroke-width:1.5\"/>\n", dash2_x, dash2_x + 4.00, self.colors.text).unwrap();
			
			write!(counter_file, "\t<text id=\"Morale\" x=\"{morale_x}\" y=\"43.00\" style=\"font-size:12px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.morale).unwrap();
			
			if self.elr {
				write!(counter_file, "\t<line id=\"ELR\" x1=\"{0:.2}\" y1=\"45.00\" x2=\"{1:.2}\" y2=\"45.00\" style=\"stroke:{2}; stroke-width:1.25\"/>\n", elr_x, elr_x + 6.50, self.colors.text).unwrap(); 
			}
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
	write!(counter_file, "\t\t<image x=\"{x_pos:.2}\" y=\"{y_pos:.2}\" width=\"{width}%\" height=\"{height}%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{source}\" xlink:href=\"{source}\"/>\n").unwrap();
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
			ClassIdentifier::CircledSecondline => {
				self.generate_class_circle_svg(&counter_file);
				self.generate_class_text_svg(&counter_file, "2");
			}			
			ClassIdentifier::Green => {
				self.generate_class_text_svg(&counter_file, "G");
			}
			ClassIdentifier::Conscript => {
				self.generate_class_text_svg(&counter_file, "C");
			}
			ClassIdentifier::BoxedConscript => {
				self.generate_class_box_svg(&counter_file, size);
				self.generate_class_text_svg(&counter_file, "C");
			}
			ClassIdentifier::CircledConscript => {
				self.generate_class_circle_svg(&counter_file);
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
				self.generate_modified_class_image_svg(&counter_file, "./svg/class_first_airborne", 1.75, 1.75, 70.00, 70.00);
			}
			ClassIdentifier::RoyalMarines => {
				self.generate_class_image_svg(&counter_file, "./svg/class_royal_marines.svg");
			}			
			ClassIdentifier::KoreanMarines => {
				self.generate_modified_class_image_svg(&counter_file, "./svg/class_sk_marines.svg", 1.75, 1.75, 70.00, 70.00);
			}
			ClassIdentifier::Fn16 => {
				self.generate_class_text_svg(&counter_file, "16");
			}			
			_ => {
			}			
		}

		write!(counter_file, "\t</svg>\n").unwrap();
		//
		// For now(?) FFI/Cross of Lorraine takes precedence over Assault Engineer demo charge.
		if "ff" == self.nationality {
			generate_svg_start_element(counter_file, 1, COL_X_POSITION, COL_Y_POSITION, COL_WIDTH, COL_HEIGHT, "Cross of Lorraine", &"yellow".to_string());
			write!(counter_file, "\t\t<image id=\"Cross of Lorraine\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{0}\" xlink:href=\"{0}\"/>\n", "./svg/CoL.svg").unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();			
		} else if self.assault_engineer {
			generate_svg_start_element(counter_file, 1, AE_X_POSITION, AE_Y_POSITION, AE_WIDTH, AE_HEIGHT, "Assault Engineer", &"red".to_string());
			write!(counter_file, "\t\t<image x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid meet\" href=\"{0}\" xlink:href=\"{0}\"/>\n", "./svg/dc.svg").unwrap();
			write!(counter_file, "\t</svg>\n").unwrap();
		}

		if ClassIdentifier::ParaMarines == self.class {
			self.generate_paramarines_image_svg(&counter_file);
		}
	}

	fn generate_broken_morale_svg_elements(&mut self, mut counter_file: &std::fs::File) {
		if 0 != self.broken_morale || self.self_rally {
			let size = BROKEN_MORALE_SIZE - (2.0 * BROKEN_MORALE_STROKE_WIDTH);
			
			generate_svg_start_element(counter_file, 1, BROKEN_MORALE_X_POSITION, BROKEN_MORALE_Y_POSITION, BROKEN_MORALE_SIZE, BROKEN_MORALE_SIZE, "Morale", &"yellow".to_string());
	
			if self.self_rally {
				write!(counter_file, "\t\t<rect id=\"Self rally\" x=\"1\" y=\"1\" width=\"{size}\" height=\"{size}\" style=\"display:inline;fill:none;fill-opacity:0.0;stroke:{0};stroke-width:{1};stroke-dasharray:none;stroke-opacity:1\"/>\n", self.colors.text, BROKEN_MORALE_STROKE_WIDTH).unwrap();
			}
		
			if 0 != self.broken_morale {
				write!(counter_file, "\t<text id=\"Morale\" x=\"50%\" y=\"80%\" style=\"font-size:{BROKEN_MORALE_FONT_SIZE}px;font-style:normal;font-variant:normal;{FONT_WEIGHT_BOLD};font-stretch:semi-expanded;text-anchor:middle;fill:{0};fill-opacity:1;{FONT_MAIN}\">{1}</text>\n", self.colors.text, self.broken_morale).unwrap();
			}
			
			write!(counter_file, "\t</svg>\n").unwrap();
		}
	}
}

fn generate_svg_counter(record: &mut Record) {
	let path = &record.args.destination.to_string();
	let mut size: u32 = 48;
	
	if 0 != record.overrides.counter_size {
		size = record.overrides.counter_size;
	} else if ClassIdentifier::LargeConcealment == record.class {
		size = 60;
	}
	//
	// Create the counter file.
	//
	let counter_file = match open_counter_file(&path, &record.piece) {
		Err(why) => panic!("couldn't create file: {0} {1}", record.piece, why),
		Ok(counter_file) => counter_file,
	};

	generate_counter_header_svg_elements("vasl_multiman_counters", &counter_file, size, &record.piece, &"".to_string(), &record.comments, &record.version);
	generate_counter_background_svg(&counter_file, size, &record.colors, &record.overrides);
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
	let path = &record.args.destination.to_string();
	let size = 48;
	let mini_size = 24;
	//
	// Create the counter file.
	//
	let mut counter_file = match open_counter_file(&path, &record.piece) {
		Err(why) => panic!("couldn't create file: {0} {1}", record.piece, why),
		Ok(counter_file) => counter_file,
	};

	generate_counter_header_svg_elements("vasl_multiman_counters", &counter_file, size, &record.piece, &"".to_string(), &record.comments, &record.version);

	write!(counter_file, "\t<svg width=\"{0:.2}\" height=\"{0:.2}\" viewBox=\"0 0 48 48\">\n", mini_size).unwrap();

	generate_counter_background_svg(&counter_file, mini_size, &record.colors, &record.overrides);

	write!(counter_file, "\t</svg>\n").unwrap();

	generate_mini_concealment_counter_svg_elements(&counter_file, record);
	generate_footer_svg(&counter_file);

	drop(counter_file);
}

fn generate_multiman_counter_svg_elements(counter_file: &std::fs::File, record: &mut Record, path: &String) {
	record.generate_unit_depiction_svg_elements(&counter_file, &path);
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
	if !record.args.quiet {
		print!("Generating '{0}.svg' ...", record.piece);
	} else {
		println!("{0}", record.piece);
	}
	
	if ClassIdentifier::MiniConcealment == record.class {
		generate_mini_concealment_svg_counter(record);
	} else {
		generate_svg_counter(record);
	}
	
	if !record.args.quiet {
		println!(" done.");
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
	bpv: String,
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
		result.args.destination = format!("{0}{1}/", result.args.destination, result.nationality);
		
		result.overrides.sanitize(&self.overrides);
		
		result.sanitize_class_identifier(&self.class);
		
		result.firepower = self.firepower.parse::<usize>().unwrap_or(0);
		
		result.range = self.range.to_string();
		
		result.sanitize_morale(&self.morale);
		
		result.assault_fire = "yes" == self.assault_fire;
		
		result.smoke = self.smoke.parse::<usize>().unwrap_or(0);
		
		result.spraying_fire = "yes" == self.spraying_fire;
		
		result.elr = "yes" == self.elr;
		
		result.self_rally = "yes" == self.self_rally;
		
		result.assault_engineer = "yes" == self.assault_engineer;
		
		result.bpv = self.bpv.parse::<usize>().unwrap_or(0);
		
		result.pieces = extract_vector(&self.piece, OVERRIDE_DELIMITER);
		
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
	let mut args = Arguments::parse();
	
	args.sanitize_destination();
	
	let mut rdr = csv::Reader::from_reader(io::stdin());

	for result in rdr.deserialize() {
		let mut spreadsheet_record: SpreadsheetRecord = result?;

		if !spreadsheet_record.nationality.is_empty() {
			let mut record: Record = spreadsheet_record.sanitize(&"".to_string(), &args);
			let pieces = record.pieces.clone();
			
			for piece in pieces {
				if !piece.contains('@') {
					record.piece = piece.to_string();
					generate_svg_counter_announcer(&mut record);
				} else {
					let (piece, nationality) = piece.split_once("@").unwrap();
					let mut alt_record: Record = spreadsheet_record.sanitize(&nationality.to_string(), &args);
					
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
