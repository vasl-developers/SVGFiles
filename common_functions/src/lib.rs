use std::io::prelude::*;
use std::path::Path;
use chrono::Utc;
use regex::Regex;
//
// Local files.
//
pub mod armament;
pub mod armor;
pub mod debugging;
pub mod defines;
pub mod common_record;
pub mod colors;
pub mod machine_guns;
pub mod malfunction;
pub mod movement;
pub mod overrides;
pub mod special;
pub mod text;
pub mod text_field;
pub mod transport;
pub mod turret;
pub mod utils;

use crate::colors::*;
use crate::debugging::*;
use crate::defines::*;
use crate::overrides::*;
use crate::text_field::*;
//
// Values for a 1000 x 1000 pixel image.
//
pub const BEVEL_WIDTH_WIDE: f32 =		50.2572;
pub const BEVEL_WIDTH_MEDIUM: f32 =		25.1286;
pub const BEVEL_WIDTH_NARROW: f32 =		12.5643;

pub const BEVEL_HIGHLIGHT_HIGH: f32 =	 1.0;
pub const BEVEL_HIGHLIGHT_MEDIUM: f32 =	 0.75;
pub const BEVEL_HIGHLIGHT_LOW: f32 =	 0.5;

pub const BEVEL_SHADOW_HIGH: f32 =		 0.8;
pub const BEVEL_SHADOW_MEDIUM: f32 =	 0.55;
pub const BEVEL_SHADOW_LOW: f32 =		 0.3;

pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn generate_counter_background_svg(mut counter_file: &std::fs::File, colors: &Colors, overrides: &Overrides) {
	let mut rectangle_size: u32 = 1000;
	let mut inset: u32 = 0;
	let delta: u32 = 75;

	if CREATE_BEVEL {
		let stroke_width = BEVEL_WIDTH_NARROW;
		let highlight = BEVEL_HIGHLIGHT_HIGH;
		let shadow = BEVEL_SHADOW_LOW;
		//
		// TODO: Is there a better, more precise way to create the bevel and to clip the counter?
		//
		write!(counter_file, "\t\t<!-- Beveled counter - is there a better/more efficient way to achieve this? -->\n").unwrap();
		write!(counter_file, "\t\t<svg x=\"{0}\" y=\"{0}\" width=\"{1}\" height=\"{1}\">\n", inset, rectangle_size).unwrap();
		write!(counter_file, "\t\t\t<defs id=\"bevel\">\n").unwrap();
		write!(counter_file, "\t\t\t\t<filter id=\"highlight_filter\" style=\"color-interpolation-filters:sRGB;\" x=\"-0.058226637\" y=\"-0.058226637\" width=\"1.1144796\" height=\"1.1144796\">\n").unwrap();
		write!(counter_file, "\t\t\t\t\t<feGaussianBlur stdDeviation=\"10 10\" result=\"fbSourceGraphic\" id=\"feGaussianBlur3\"/>\n").unwrap();
		write!(counter_file, "\t\t\t\t\t<feColorMatrix result=\"fbSourceGraphicAlpha\" in=\"fbSourceGraphic\" values=\"0 0 0 -1 0 0 0 0 -1 0 0 0 0 -1 0 0 0 0 1 0\" id=\"feColorMatrix3\"/>\n").unwrap();
		write!(counter_file, "\t\t\t\t\t<feGaussianBlur id=\"feGaussianBlur4\" stdDeviation=\"10 10\" result=\"blur\" in=\"fbSourceGraphic\"/>\n").unwrap();
		write!(counter_file, "\t\t\t\t</filter>\n").unwrap();
		write!(counter_file, "\t\t\t\t<filter id=\"shadow_filter\" style=\"color-interpolation-filters:sRGB;\" x=\"-0.030508946\" y=\"-0.030508946\" width=\"1.0629916\" height=\"1.0629916\">\n").unwrap();
		write!(counter_file, "\t\t\t\t\t<feGaussianBlur id=\"feGaussianBlur5\" stdDeviation=\"10 10\" result=\"blur\"/>\n").unwrap();
		write!(counter_file, "\t\t\t\t</filter>\n").unwrap();
		write!(counter_file, "\t\t\t\t<clipPath id=\"counter_clipping\">\n").unwrap();
		write!(counter_file, "\t\t\t\t\t<rect id=\"color\" style=\"display:inline;fill:red;fill-opacity:1;stroke:none;stroke-width:0;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:0\" width=\"100%\" height=\"100%\" x=\"0\" y=\"0\" ry=\"60\" rx=\"60\"/>\n").unwrap();
		write!(counter_file, "\t\t\t\t</clipPath>\n").unwrap();	
		write!(counter_file, "\t\t\t</defs>\n").unwrap();
		write!(counter_file, "\t\t\t<g id=\"background\">\n").unwrap();
		write!(counter_file, "\t\t\t\t<rect id=\"color\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:0;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:0\" width=\"100%\" height=\"100%\" x=\"0\" y=\"0\" ry=\"60\" rx=\"60\"/>\n", colors.background).unwrap();
		write!(counter_file, "\t\t\t\t<g id=\"bevel\" clip-path=\"url(#counter_clipping)\">\n").unwrap();
		write!(counter_file, "\t\t\t\t\t<path id=\"shadow\" style=\"display:inline;fill:none;stroke:#000000;stroke-width:{0};filter:url(#shadow_filter);stroke-opacity:{1}\" d=\"m 1023.6424,120.45592 c 10.8149,10.81495 17.4864,25.7734 17.4864,42.34736 v 830.07515 c 0,33.14797 -26.6859,59.83377 -59.83378,59.83377 H 151.21981 c -16.57396,0 -31.54851,-6.6875 -42.34737,-17.4864\" transform=\"matrix(1.0503456,0,0,1.0590455,-36.912259,-39.56161) translate(-60.0, -75.0)\"/>\n", stroke_width, shadow).unwrap();
		write!(counter_file, "\t\t\t\t\t<path id=\"highlight\" style=\"display:inline;fill:none;stroke:#ffffff;stroke-width:{0};stroke-opacity:{1};filter:url(#highlight_filter)\" d=\"M 108.87244,1035.2258 C 98.057485,1024.4109 91.386017,1009.4524 91.386017,992.87843 V 162.80328 c 0,-33.14792 26.685873,-59.83379 59.833793,-59.83379 h 830.07521 c 16.57394,0 31.53238,6.67147 42.34738,17.48643\" transform=\"matrix(1.0387133,0,0,1.0339712,-12.412385,-8.9484139) translate(-66.0, -77.0)\"/>\n", stroke_width, highlight).unwrap();
		write!(counter_file, "\t\t\t\t</g>\n").unwrap();
		write!(counter_file, "\t\t\t</g>\n").unwrap();
		write!(counter_file, "\t\t</svg>\n").unwrap();	
	} else {
		write!(counter_file, "\t\t<rect x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", colors.background).unwrap();
	}

	if !colors.inner_background.is_empty() {
		rectangle_size -= 2 * delta;
		inset += delta;
		write!(counter_file, "\t\t<rect x=\"{0}\" y=\"{0}\" rx=\"40\" ry=\"40\" width=\"{1}\" height=\"{1}\" style=\"display:inline;fill:{2};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", inset, rectangle_size, colors.inner_background).unwrap();
	}

	if !overrides.captured.is_empty() {
		rectangle_size -= 2 * delta;
		inset += delta;

		generate_captured_background_svg(counter_file, &overrides, rectangle_size, inset, delta);
	}
	
	generate_debug_grid_svg(counter_file);
}

pub fn generate_unit_depiction_svg(mut counter_file: &std::fs::File, root_path: &String, filename: &String, note: &String, svg_transform: &String, front: bool, name: &String, display_name: bool, colors: &Colors) {
	if INCLUDE_IMAGES {
		let path_prefix = "svg/";
		let file_type_svg = ".svg";
		let file_type_png = ".png";

		let paths: Vec<std::string::String> = [
			format!("{}{}{}", path_prefix, filename, file_type_svg),
			format!("{}{}{}", path_prefix, filename, file_type_png)
		].to_vec();

		for mut path in paths {
			let mut pathname: String = root_path.to_string();
			pathname.push_str(&path.to_string());

			if Path::new(&pathname).exists() {
				let mut transform: String = Default::default();

				if pathname.contains(".png") { // Temporary (eventually).
					if !front {
						transform = "rotate(-90, 30, 30)".to_string();
					}
				} else if !svg_transform.is_empty() {
					transform = svg_transform.to_string();
				} else {
					transform = "scale(1.00) translate(0, 0) rotate(60, 30, 30)".to_string();
				}

				if path.contains(SPACE) {
					path = path.replace(SPACE, "%20");
				}

				write!(counter_file, "\t<!-- Unit depiction -->\n").unwrap();
				write!(counter_file, "\t<image x=\"0\" y=\"0\" width=\"60\" height=\"60\" preserveAspectRatio=\"xMidYMid meet\" transform=\"{0}\" href=\"{1}\" xlink:href=\"{1}\"/>\n", transform, path).unwrap();
				break;	// Our work here is done.
			}
		}
	}

	generate_debug_note_svg(counter_file, note);
	
	if INCLUDE_NAME || display_name {
		let mut temp_name: String = name.to_string();

		if temp_name.contains("<i>") {
			temp_name = strip_vehicle_type_from_name(&temp_name);
		}
		
		write!(counter_file, "\t<text x=\"39\" y=\"27\" style=\"font-size:{0:.2}px;font-style:light;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{1};fill-opacity:1;font-family:{2}\" transform=\"rotate(60,45,17)\">{3}</text> <!-- Name -->\n", NAME_FONT_SIZE, colors.text, FONT_MAIN, temp_name).unwrap();
	}	
}

pub fn generate_unit_depiction_svg_elements(mut counter_file: &std::fs::File, note: &String, name: &String, display_name: bool, colors: &Colors) {
	generate_debug_note_svg(counter_file, note);
	
	if INCLUDE_NAME || display_name {
		let mut temp_name: String = name.to_string();

		if temp_name.contains("<i>") {
			temp_name = strip_vehicle_type_from_name(&temp_name);
		}
		
		write!(counter_file, "\t<text x=\"39\" y=\"27\" style=\"font-size:{0:.2}px;font-style:light;font-variant:normal;font-weight:normal;font-stretch:normal;text-anchor:middle;fill:{1};fill-opacity:1;font-family:{2}\" transform=\"rotate(60,39,27)\">{3}</text> <!-- Name -->\n", NAME_FONT_SIZE, colors.text, FONT_MAIN, temp_name).unwrap();
	}	
}

pub fn generate_large_counter_header_svg_elements(program_name: &'static str, mut counter_file: &std::fs::File, note_number: &String, name: &String, comment: &String, version: &String) {
	write!(counter_file, "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n").unwrap();
	write!(counter_file, "<svg width=\"60\" height=\"60\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:svg=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n").unwrap();
	write!(counter_file, "\t<!--\n").unwrap();
	write!(counter_file, "\t\tNote #:\t\t{0}\n", note_number).unwrap();
	write!(counter_file, "\t\tName:\t\t{0}\n", name).unwrap();

	if 0 != comment.len() {
		write!(counter_file, "\t\tComment:\t{0}\n", comment).unwrap();
	}

	write!(counter_file, "\t\tVersion:\t{0}\n\n", version).unwrap();
	write!(counter_file, "\t\tGenerated by {0} version {1} on {2}\n", program_name, VERSION, Utc::now().format("%F")).unwrap();

	let mut authors_list: String = "\t\tAuthor(s):".to_string();

	if AUTHORS.contains(':') {
		let authors = AUTHORS.split(':');
		let mut first = true;

		for author in authors {
			if !author.is_empty() {
				if !first {
					authors_list.push_str(",");
				} else {
					first = false;
				}

				authors_list.push_str(" ");
				authors_list.push_str(author);
			}
		}
	} else {
		authors_list.push_str(" ");
		authors_list.push_str(AUTHORS);
	}

	write!(counter_file, "{0}\n", authors_list).unwrap();
	write!(counter_file, "\t\tTester(s): Alan Bills, Alan Cannamore, Doug Rimmer\n").unwrap();

	let filename: String = if "vasl_vehicle_counters" == program_name { "Vehicle".to_string() } else { "Ordnance".to_string() };
	
	write!(counter_file, "\t\tCounter data scraped with permission from: https://www.klasm.com/ASL/Listings/{filename}Listings.html\n").unwrap();
	write!(counter_file, "\t-->\n\n").unwrap();

	embed_fonts_svg(&counter_file);
}

pub fn generate_counter_header_svg_elements(program_name: &'static str, mut counter_file: &std::fs::File, size: usize, name: &String, comment: &String, version: &String) {
	write!(counter_file, "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n").unwrap();
	write!(counter_file, "<svg width=\"{size}\" height=\"{size}\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:svg=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n").unwrap();
	write!(counter_file, "\t<!--\n").unwrap();
	write!(counter_file, "\t\tName:\t\t{0}\n", name).unwrap();

	if 0 != comment.len() {
		write!(counter_file, "\t\tComment:\t{0}\n", comment).unwrap();
	}

	write!(counter_file, "\t\tVersion:\t{0}\n\n", version).unwrap();
	write!(counter_file, "\t\tGenerated by {0} version {1} on {2}\n", program_name, VERSION, Utc::now().format("%F")).unwrap();

	let mut authors_list: String = "\t\tAuthor(s):".to_string();

	if AUTHORS.contains(':') {
		let authors = AUTHORS.split(':');
		let mut first = true;

		for author in authors {
			if !author.is_empty() {
				if !first {
					authors_list.push_str(",");
				} else {
					first = false;
				}

				authors_list.push_str(" ");
				authors_list.push_str(author);
			}
		}
	} else {
		authors_list.push_str(" ");
		authors_list.push_str(AUTHORS);
	}

	write!(counter_file, "{0}\n", authors_list).unwrap();
	write!(counter_file, "\t\tTester(s): Alan Bills, Alan Cannamore, Doug Rimmer\n").unwrap();
	write!(counter_file, "\t-->\n\n").unwrap();

	embed_fonts_svg(&counter_file);
}

pub fn generate_footer_svg(mut counter_file: &std::fs::File) {
	write!(counter_file, "</svg>\n").unwrap();
}

fn generate_captured_background_svg(mut counter_file: &std::fs::File, overrides: &Overrides, rectangle_size: u32, inset: u32, delta: u32) {
	let colors = nationality_to_color(&overrides.captured);

	write!(counter_file, "\t\t<rect x=\"{0}\" y=\"{0}\" rx=\"40\" ry=\"40\" width=\"{1}\" height=\"{1}\" style=\"display:inline;fill:{2};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", inset, rectangle_size, colors[0]).unwrap();

	if UNDEFINED_COLOR != colors[1] {
		let my_rectangle_size = rectangle_size - (2 * delta);
		let my_inset = inset + delta;

		write!(counter_file, "\t\t\t<rect x=\"{0}\" y=\"{0}\" rx=\"40\" ry=\"40\" width=\"{1}\" height=\"{1}\" style=\"display:inline;fill:{2};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", my_inset, my_rectangle_size, colors[1]).unwrap();
	}
}

fn strip_vehicle_type_from_name(original: &String) -> std::string::String {
	let re_html_i = Regex::new(r"(?<keep>.*)(?<drop1>[ ]<i>[a-zA-Z0-9/\-]*)(?<drop2><\/i>)").unwrap();
	let Some(caps) = re_html_i.captures(&original) else { panic!("strip_italics_from_name regex failed!") };

	return (&caps["keep"]).to_string();
}

fn embed_fonts_svg(mut counter_file: &std::fs::File) {
	if INCLUDE_FONTS {
		write!(counter_file, "\t<defs>\n").unwrap();
		write!(counter_file, "\t\t<style> <!-- Fonts references. -->\n").unwrap();
		write!(counter_file, "\t\t\t@font-face {{\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-family: {0};\n", FONT_MAIN).unwrap();
		write!(counter_file, "\t\t\t\tfont-weight: normal;\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-style: normal;\n").unwrap();
		write!(counter_file, "\t\t\t\tsrc:\turl(\"../../../fonts/helvetica_condensed-webfont.woff2\") format(\"woff2\"),\n").unwrap();
		write!(counter_file, "\t\t\t\t\t\turl(\"../../../fonts/helvetica_condensed-webfont.woff\") format(\"woff\");\n").unwrap();
		write!(counter_file, "\t\t\t}}\n").unwrap();
		write!(counter_file, "\t\t\t@font-face {{\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-family: 'Times-New-Roman';\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-weight: normal;\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-style: normal;\n").unwrap();
		write!(counter_file, "\t\t\t\tsrc:\turl(\"../../../fonts/times-webfont.woff2\") format(\"woff2\"),\n").unwrap();
		write!(counter_file, "\t\t\t\t\t\turl(\"../../../fonts/times-webfont.woff\") format(\"woff\");\n").unwrap();
		write!(counter_file, "\t\t\t}}\n").unwrap();
		write!(counter_file, "\t\t</style>\n\n").unwrap();
		write!(counter_file, "\t</defs>\n").unwrap();
	} else if EMBED_FONTS {
		write!(counter_file, "\t<defs>\n").unwrap();
		write!(counter_file, "\t\t<style> <!-- Embedded Fonts. -->\n").unwrap();
		write!(counter_file, "\t\t\t@font-face {{\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-family: '{0}-e';\n", FONT_MAIN).unwrap();
		write!(counter_file, "\t\t\t\tfont-weight: normal;\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-style: normal;\n").unwrap();
		write!(counter_file, "\t\t\t\tsrc:\turl(\"data:font/woff2;base64,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\") format(\"woff2\");\n").unwrap();
		write!(counter_file, "\t\t\t}}\n").unwrap();
		write!(counter_file, "\t\t\t@font-face {{\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-family: 'Times-New-Roman-e';\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-weight: normal;\n").unwrap();
		write!(counter_file, "\t\t\t\tfont-style: normal;\n").unwrap();
		write!(counter_file, "\t\t\t\tsrc:\turl(\"data:font/woff2;base64,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\" format(\"woff2\");\n").unwrap();
		write!(counter_file, "\t\t\t}}\n").unwrap();
		write!(counter_file, "\t\t</style>\n\n").unwrap();
		write!(counter_file, "\t</defs>\n").unwrap();
	} else if LINK_FONTS {
		write!(counter_file, "\t<!-- Linked Fonts. -->\n").unwrap();
		write!(counter_file, "\t<link xmlns=\"http://ww.w3.org/1999/xhtml\" rel=\"stylesheet\" href=\"../../../fonts.svg\" type=\"text/css\"/>\n\n").unwrap();
	}
}
