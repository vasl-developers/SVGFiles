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
use common_functions::armament::*;
use common_functions::colors::*;
use common_functions::common_record::*;
use common_functions::debugging::*;
use common_functions::defines::*;
use common_functions::machine_guns::*;
use common_functions::overrides::*;
use common_functions::text_field::*;
use common_functions::transport::*;
use common_functions::utils::*;

const DATE_FONTS: [[f64; 4]; 8] = [
	[   6.0,   4.0,  80.0,   6.6 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

const DATE_FONT_WEIGHT: &str =	"normal";

const DATE_LINE_X_POSITION: f64 =		21.0;
const DATE_LINE_Y_POSITION: f64 =		37.0;
const DATE_WIDTH: f64 =					36.0;

pub const ORD_FONTS: [[f64; 4]; 8] = [
	[  12.0,   6.0,  86.00,  12.0 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]		
	[  12.0,   6.0,  60.00,  15.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[  12.0,   6.0,  80.00,  15.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[  12.0,   6.0,  66.67,  18.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  66.67,   9.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  54.55,  11.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  72.73,  11.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  61.54,  13.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

pub const ML_FONTS: [[f64; 4]; 8] = [
	[   8.0,   4.0,  98.00,   6.0 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   7.5,   4.0,  73.00,   8.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  98.00,   8.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   8.0,   4.0,  80.00,  10.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,    0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,    0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,    0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,    0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

const ML_Y_POSITION: f64 =	37.0;

pub const CLASS_FONTS: [[f64; 4]; 8] = [
	[  13.0,   7.0,  90.0,  10.2 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]		
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

const CLASS_X_POSITION: f64 =	30.0;
const CLASS_Y_POSITION: f64 =	 3.0;
const CLASS_HEIGHT: f64 =		11.22;

pub const AA_FONTS: [[f64; 4]; 8] = [
	[   8.4,   6.0,  75.0,   6.6 ],	// FONT_NORMAL:			[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_UNDERLINED: 	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_OVERLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_BOTHLINED:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_NORMAL:		[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_UNDERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_OVERLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
	[   0.0,   0.0,   0.0,   0.0 ],	// FONT_ALT_BOTHLINED:	[ FA_SIZE, FA_SUP_SIZE, FA_Y_PERCENTAGE, FA_HEIGHT ]
];

const AA_X_POSITION: f64 =		 3.0;
const AA_Y_POSITION: f64 =		 3.0;
const AA_HEIGHT: f64 =			11.22;
//
// Sanitized and parsed aircraft-specific record fields.
//
#[derive(Default)]
struct Record {
	args: Arguments,
	nationality: String,
	name: String,
	class: TextField,
	date: TextField,
	aa: TextField,
	ml: TextField,
	ord: TextField,
	ord_type: String,
	rof_aerial: TextField,
	rof_bomb: TextField,
	mgs: MachineGuns,
	transport_values: TransportValues,
	note: String,
	version: String,
	piece: String,
	overrides: Overrides,
	colors: Colors,
	svg_image_transform: String,
	comments: String,
}

impl Record {
	fn generate_aircraft_depiction_svg_elements(&mut self, mut counter_file: &std::fs::File, root_path: &String) {
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

					write!(counter_file, "\t<!-- Aircraft depiction -->\n").unwrap();
					write!(counter_file, "\t<image x=\"6\" y=\"6\" width=\"48\" height=\"48\" preserveAspectRatio=\"xMidYMid meet\" transform=\"{transform}\" href=\"{path}\" xlink:href=\"{path}\"/>\n").unwrap();
					break;	// Our work here is done.
				}
			}
		}
	}
}

fn generate_date_svg_elements(mut counter_file: &std::fs::File, date: &TextField) {
	if !date.text.is_empty() {
		let x_position = DATE_LINE_X_POSITION;
		let y_position = DATE_LINE_Y_POSITION - date.fonts.height();
	
		generate_svg_start_element(counter_file, 1, x_position, y_position, DATE_WIDTH, date.fonts.height(), "Date", &"lightgreen".to_string());

		write!(counter_file, "\t\t<text x=\"100%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"end\"><tspan style=\"font-size:{1:.2}px;font-weight:{2};font-family:{3};fill:{4};fill-opacity:1;stroke-width:0.2\">{5}</tspan></text>\n", date.fonts.y_percentage(), date.fonts.size(), DATE_FONT_WEIGHT, FONT_MAIN, date.color, date.text).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}
}

fn generate_ordnance_svg_elements(mut counter_file: &std::fs::File, ord: &TextField, ord_type: &String) -> f64 {
	let x_position = 0.0;
	let mut y_position = 0.0;

	if "B" == ord_type {
		y_position = GUN_CALIBER_BASELINE - ord.fonts.height();
		
		generate_svg_start_element(counter_file, 1, GUN_COLUMN_X_POSITION, y_position, 54.0, ord.fonts.height(), "Bomb, text position adjusts to account for any overlined/underlined text.", "white");
		
		write!(counter_file, "\t\t<text x=\"{0:.2}\" y=\"{1:.2}%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">{5}</text>\n", x_position, ord.fonts.y_percentage(), ord.fonts.size(), &FONT_MAIN.to_string(), &ord.color, &ord.text).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	} else if "R" == ord_type {
		let height = 22.00; /* magic! */
		
		y_position = 38.00; /* magic! */
		
		generate_svg_start_element(counter_file, 1, GUN_COLUMN_X_POSITION, y_position, 54.00 /* magic! */, height, "Rocket, text position adjusts to account for any overlined/underlined text.", "white");

		write!(counter_file, "\t\t<text x=\"0.00\" y=\"44.00%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:6.00px;font-weight:normal;font-family:Nimbus Sans L;fill:black\"><tspan style=\"text-decoration:overline\">Rocket</tspan></text>\n").unwrap();
		write!(counter_file, "\t\t<text x=\"{0:.2}\" y=\"{1:.2}%\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:{2:.2}px;{FONT_WEIGHT_BOLD};font-family:{3};fill:{4}\">{5}</text>\n", x_position, ord.fonts.y_percentage(), ord.fonts.size(), &FONT_MAIN.to_string(), &ord.color, &ord.text).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();		
	}
	
	return y_position - gun_column_y_gap(&counter_file, GUN_COLUMN_X_POSITION, y_position, "blue");
}

fn generate_ml_number_svg_elements(mut counter_file: &std::fs::File, ml: &TextField, y_position: f64)
{
	if !ml.text.is_empty() {
		generate_svg_start_element(counter_file, 1, GUN_COLUMN_X_POSITION, y_position, 36.0, ml.fonts.height(), "ML", &"white".to_string()); // Magic!
		
		write!(counter_file, "\t\t<text x=\"0.00\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"start\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}ML</tspan></text>\n", ml.fonts.y_percentage(), ml.fonts.size(), FONT_MAIN, ml.color, ml.text).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}
}

fn generate_aa_svg_elements(mut counter_file: &std::fs::File, aa: &TextField, _colors: &Colors /* TODO: ignored, for now? */) {	
	let color = "white".to_string();
	
	generate_svg_start_element(counter_file, 1, AA_X_POSITION, AA_Y_POSITION, AA_HEIGHT, AA_HEIGHT, "AA Fire Target DRM", "white");

	write!(counter_file, "\t\t<text x=\"50%\" y=\"90%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:14.00px;{FONT_WEIGHT_BOLD};font-family:{0};fill:{1};fill-opacity:1;stroke-width:0.2\">{2}</tspan></text>\n", FONT_MAIN, aa.color, STAR.to_string()).unwrap();
	write!(counter_file, "\t\t<text x=\"50%\" y=\"75%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:8.00px;{FONT_WEIGHT_BOLD};font-family:{0};fill:{1};fill-opacity:1;stroke-width:0.2\">{2}</tspan></text>\n", FONT_MAIN, color, aa.text).unwrap();
	write!(counter_file, "\t</svg>\n").unwrap();
}

fn generate_rof_svg_elements(mut counter_file: &std::fs::File, rof: &TextField, x_position: f64, y_position: f64, color: &String, comment: &str) {	
	if !rof.text.is_empty() {
		generate_svg_start_element(counter_file, 1, x_position, y_position, ROF_HEIGHT, ROF_HEIGHT, comment, "white");

		write!(counter_file, "\t\t<text x=\"50%\" y=\"{0}%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", rof.fonts.y_percentage(), rof.fonts.size(), FONT_MAIN, color, rof.text).unwrap();
		write!(counter_file, "\t\t<rect x=\"{0:.2}\" y=\"{0:.2}\" width=\"{1:.2}\" height=\"{1:.2}\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{2};stroke-width:{3:.2}px;stroke-dasharray:none;stroke-opacity:1\"/>\n", ROF_OFFSET, ROF_BOX_SIZE, color, ROF_STROKE_WIDTH).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}
}

fn generate_class_svg_elements(mut counter_file: &std::fs::File, class: &TextField) {
	if !class.text.is_empty() {
		generate_svg_start_element(counter_file, 1, CLASS_X_POSITION, CLASS_Y_POSITION, 27.0, CLASS_HEIGHT, "Aircraft Class", &"blue".to_string());

		write!(counter_file, "\t\t<text x=\"100%\" y=\"{0:.2}%\" dominant-baseline=\"auto\" text-anchor=\"end\"><tspan style=\"font-size:{1:.2}px;{FONT_WEIGHT_BOLD};font-family:{2};fill:{3};fill-opacity:1;stroke-width:0.2\">{4}</tspan></text>\n", class.fonts.y_percentage(), class.fonts.size(), FONT_MAIN, class.color, class.text).unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}	
}

fn generate_landed_glider_counter_front(mut counter_file: &std::fs::File, record: &mut Record, crashed: bool) {
	record.class.text = if crashed { "".to_string() } else { "GL".to_string() };
	generate_class_svg_elements(&counter_file, &record.class);
	
	if !crashed {
		write!(counter_file, "\t<text x=\"3.00\" y=\"47.00\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:6.00px;{FONT_WEIGHT_NORM};font-family:{0};fill:black\">cs 7</text>\n", &FONT_MAIN.to_string()).unwrap();
		write!(counter_file, "\t<text x=\"30.00\" y=\"57.00\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:8.00px;{FONT_WEIGHT_NORM};font-family:{0};fill:black\">+1 Hindrance</text>\n", &FONT_MAIN.to_string()).unwrap();
		write!(counter_file, "\t<text x=\"50.00\" y=\"35.00\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:7px;font-weight:bold;font-family:Nimbus Sans L;fill:black;fill-opacity:1;stroke:none;stroke-width:0.33\">★</tspan></text>\n").unwrap();
		write!(counter_file, "\t<text x=\"50.00\" y=\"45.00\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:7px;font-weight:bold;font-family:Nimbus Sans L;fill:black;fill-opacity:1;stroke:none;stroke-width:0.33\">★</tspan></text>\n").unwrap();
	} else {
		write!(counter_file, "\t<text x=\"3.00\" y=\"50.00\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:8.00px;{FONT_WEIGHT_NORM};font-family:{0};fill:black\">+1 TEM/</text>\n", &FONT_MAIN.to_string()).unwrap();
		write!(counter_file, "\t<text x=\"3.00\" y=\"57.00\" dominant-baseline=\"auto\" text-anchor=\"start\" style=\"font-size:8.00px;{FONT_WEIGHT_NORM};font-family:{0};fill:black\">+1 Hindrance</text>\n", &FONT_MAIN.to_string()).unwrap();
	}
	
}

fn generate_fac_counter_front(mut counter_file: &std::fs::File, record: &mut Record) {
	let status = extract_string(&record.name, ")", "(").to_uppercase();

	if "NO CONTACT" == status {
		write!(counter_file, "\t<line x1=\"12.00\" y1=\"48.00\" x2=\"48.00\" y2=\"12.00\" style=\"display:inline;fill:none;fill-opacity:1;stroke:white;stroke-width:3.0;stroke-dasharray:none;stroke-opacity:1\"/>\n").unwrap();
		write!(counter_file, "\t<circle cx=\"30.00\" cy=\"30.00\" r=\"25.00\" style=\"display:inline;fill:none;fill-opacity:1;stroke:white;stroke-width:3.0;stroke-dasharray:none;stroke-opacity:1\"></circle>\n").unwrap();
	}
	
	write!(counter_file, "\t<text x=\"30.00\" y=\"11.00\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:8.00px;{FONT_WEIGHT_NORM};font-family:{0};fill:black\">FAC</text>\n", &FONT_MAIN.to_string()).unwrap();
	write!(counter_file, "\t<text x=\"30.00\" y=\"55.00\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:8.00px;{FONT_WEIGHT_NORM};font-family:{0};fill:black\">{status}</text>\n", &FONT_MAIN.to_string()).unwrap();
}

fn generate_observation_plane_counter_front(mut counter_file: &std::fs::File, record: &mut Record) {
	generate_ml_number_svg_elements(&counter_file, &record.ml, ML_Y_POSITION);
	
	write!(counter_file, "\t<text x=\"30.00\" y=\"55.00\" dominant-baseline=\"auto\" text-anchor=\"middle\" style=\"font-size:9.00px;{FONT_WEIGHT_BOLD};font-family:{0};fill:black\">Sighting TC</text>\n", &FONT_MAIN.to_string()).unwrap();
}

fn generate_counter_front(counter_file: &std::fs::File, path: &String, record: &mut Record) {
	generate_counter_background_svg(counter_file, 60, &record.colors, &record.overrides);
	generate_debug_working_area_svg(&counter_file);
	record.generate_aircraft_depiction_svg_elements(counter_file, &path);
	generate_unit_depiction_svg_elements(counter_file, &record.note, &record.name, false, &record.colors, &record.args);

	if "LG" == record.class.text {
		generate_landed_glider_counter_front(&counter_file, record, false);
	} else 	if "CG" == record.class.text {
		generate_landed_glider_counter_front(&counter_file, record, true);
	} else if "FAC" == record.class.text {
		generate_fac_counter_front(&counter_file, record);
	} else if "OBS" == record.class.text {
		generate_observation_plane_counter_front(&counter_file, record);		
	} else {
		generate_class_svg_elements(&counter_file, &record.class);
		
		generate_date_svg_elements(&counter_file, &record.date);
		
		record.mgs.generate_svg_elements(&counter_file);
		
		let mut y_position = generate_ordnance_svg_elements(&counter_file, &record.ord, &record.ord_type);
		
		generate_rof_svg_elements(&counter_file, &record.rof_bomb, 25.0, 57.0 - ROF_HEIGHT, &"black".to_string(), "Bomb ROF");
		
		if "R" == record.ord_type {
			y_position -= record.ml.fonts.height();
		} else {
			y_position = ML_Y_POSITION;
		}
		
		generate_ml_number_svg_elements(&counter_file, &record.ml, y_position);
		
		generate_aa_svg_elements(&counter_file, &record.aa, &record.colors);
		
		generate_rof_svg_elements(&counter_file, &record.rof_aerial, 57.0 - ROF_HEIGHT, 49.0 - ROF_HEIGHT, &"red".to_string(), "Aerial ROF");
		
		gl_generate_pp_svg_elements(&counter_file, &record.transport_values.pp, GUN_COLUMN_X_POSITION, GUN_COLUMN_Y_POSITION, &"start".to_string());
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
// TODO: NOT YET 		record.ma.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_SA == field {
// TODO: NOT YET 		record.sa.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_BRK_SA == field {
// TODO: NOT YET 		record.sa_malfunction.breakdown.value.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_BRK_MA == field {
// TODO: NOT YET 		record.malfunction.breakdown.value.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_ROF == field {
// TODO: NOT YET 		record.ma.rof.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_MP == field {
// TODO: NOT YET 		record.movement_values.points.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_PP == field {
// TODO: NOT YET 		record.transport_values.pp.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_TOW == field {
// TODO: NOT YET 		record.transport_values.towing.note.initialize(note, action);
// TODO: NOT YET 	} else if OVR_FIELD_MAU == field {
// TODO: NOT YET 		record.ma.underline_note.initialize(note, action);
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
// TODO: NOT YET 		record.ma.range.font_size = font_size;
// TODO: NOT YET 	} else if OVR_FIELD_MA == field {
// TODO: NOT YET 		record.ma.font_size = font_size;
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
	if !record.args.quiet {
		print!("Generating '{0}.svg' ({1}) ...", record.piece, note_number);
	} else {
		println!("{0}", record.piece);
	}

	let path = &record.args.destination.to_string();
	//
	// Create the front counter file.
	//
	let counter_file = match open_counter_file(&path, &record.piece) {
		Err(why) => panic!("couldn't create file: {0} {1}", record.piece, why),
		Ok(counter_file) => counter_file,
	};

	generate_counter_header_svg_elements("vasl_aircraft_counters", &counter_file, 60, &record.name, &note_number, &record.comments, &record.version);
	generate_counter_front(&counter_file, &path, record);
	generate_footer_svg(&counter_file);

	drop(counter_file);

/* TODO: NOT YET?
	if CREATE_WRECKS && !ignore_element(&record.overrides, &IGNORE_REVERSE) {
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

	if !record.args.quiet {
		println!(" done.");
	}
}

fn generate_counters(record: &mut Record) {
	let note_number: String = record.note.clone();
	
	if !record.nationality.is_empty() {
		generate_counter(record, &note_number);
	} else {
		println!("Missing nationality for piece '{0}'", record.piece);
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
	name: String,
	date: String,
	aa: String,
	ml: String,
	ord: String,
	ord_type: String,
	rof: String,
	mgs: String,
	pp: String,
	notes: String,
	version: String,
	piece: String,
	overrides: String, // Overrides various items from the original CSV entry. '|'-separated list of overrides of the form "XXX=YYY" or just "ZZZ". See const declarations in overrides.rs for complete list.
	svg_image_transform: String,
	comments: String,
}

impl SpreadsheetRecord {
	fn sanitize(&mut self, args: &Arguments) -> Record {
		let mut result: Record = Default::default();
		
		result.args = args.clone();
		
		result.nationality = extract_from(&self.overrides, NOVR_NATIONALITY);
		
		result.overrides.sanitize(&self.overrides);
		
		result.note = extract_note_number(&self.notes);
		
		if !result.overrides.name.is_empty() {
			result.name = result.overrides.name.to_string();
		} else {
			result.name = self.name.to_string();
		}

		if !result.overrides.background_color.is_empty() {
			result.colors = nationality_to_colors(&result.overrides.background_color);
		} else {
			result.colors = nationality_to_colors(&result.nationality);
		}

		result.class.text = strip_html_italics(&result.name);
		
		result.class.sanitize(&strip_html_italics(&result.name), &"".to_string(), CLASS_FONTS, &result.colors);
		
		result.date.color = result.colors.text.to_string();
		
		result.date.fonts.initialize(DATE_FONTS);
		
		result.date.text = self.date.to_string();
		
		if !self.aa.is_empty() {
			result.aa.text = self.aa.to_string();
			result.aa.sanitize(&self.aa.to_string(), &"".to_string(), AA_FONTS, &result.colors);
		}

		result.ml.text = self.ml.to_string();
		
		result.ml.sanitize(&self.ml.to_string(), &"".to_string(), ML_FONTS, &result.colors);

		if !self.ord.is_empty() {
			let mut underline: bool = false;
			let mut overline: bool = false;
			
			result.ord.text = self.ord.to_string();
			result.ord.sanitize(&self.ord.to_string(), &"".to_string(), ORD_FONTS, &result.colors);
			
			if result.ord.text.contains("text-decoration:underline") {
				underline = true;
				result.ord.fonts.selected_font = FONT_UNDERLINED;
			}
			
			if result.ord.text.contains("text-decoration:overline") {
				overline = true;
				result.ord.fonts.selected_font = FONT_OVERLINED;
			}
			
			if overline && underline {
				result.ord.fonts.selected_font = FONT_BOTHLINED;
			}
				
			result.ord_type = self.ord_type.to_string();
		}

		if !self.rof.is_empty() {
			if self.rof.contains("/") {
				let (aerial, bomb) = self.rof.split_once("/").unwrap();
				
				result.rof_aerial.text = aerial.to_string();
				result.rof_aerial.sanitize(&aerial.to_string(), &"".to_string(), ROF_FONTS, &result.colors);
				
				if !bomb.is_empty() {
					result.rof_bomb.text = bomb.to_string();
					result.rof_bomb.sanitize(&bomb.to_string(), &"".to_string(), ROF_FONTS, &result.colors);
				}
			} else {
				result.rof_aerial.text = self.rof.to_string();
				result.rof_aerial.sanitize(&self.rof.to_string(), &"".to_string(), ROF_FONTS, &result.colors);
			}
		}

		result.mgs.sanitize_single(&self.mgs, &result.overrides, &result.colors);
		
		result.transport_values.gl_sanitize(&self.pp, &result.overrides, &result.colors);
		
		result.version = self.version.to_string();

		result.piece = self.piece.to_string();

		result.svg_image_transform = self.svg_image_transform.to_string();
		
		result.comments = self.comments.to_string();
		
		return result;
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let mut args = Arguments::parse();
	
	args.sanitize_destination();
	args.destination.push_str("sh/");
	
	let mut rdr = csv::Reader::from_reader(io::stdin());

	for result in rdr.deserialize() {
		let mut spreadsheet_record: SpreadsheetRecord = result?;

		if !spreadsheet_record.overrides.contains(NOVR_IGNORE) {
			let mut record: Record = spreadsheet_record.sanitize(&args);
			
			generate_counters(&mut record);
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
