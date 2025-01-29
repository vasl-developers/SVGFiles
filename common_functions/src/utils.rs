use std::io::prelude::*;
use std::env;
use std::{io};
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
//
// Local files.
//
use crate::debugging::*;
use crate::debug_layout;
use crate::debug_rectangle;
use crate::overrides::*;
use crate::text_field::*;

pub fn generate_svg_start_element(mut counter_file: &std::fs::File, depth: usize, x: f64, y: f64, width: f64, height: f64, comment_text: &str, color: &str) {
	write!(counter_file, "{0}<!-- {1} -->\n", "\t".repeat(depth.try_into().unwrap()), comment_text.to_string()).unwrap();
	write!(counter_file, "{0}<svg x=\"{1:.2}\" y=\"{2:.2}\" width=\"{3:.2}\" height=\"{4:.2}\">\n", "\t".repeat(depth.try_into().unwrap()), x, y, width, height).unwrap();
	debug_layout!(counter_file, depth + 1, color);
}
//
// Ensure that gap between SVG elements in the "gun column" is 2 pixels at 100% zoom.
//
const GUN_COLUMN_Y_GAP: f64 = 2.0;

pub fn gun_column_y_gap(mut counter_file: &std::fs::File, x: f64, y: f64, color: &str) -> f64 {
	debug_rectangle!(counter_file, 1, x, y - GUN_COLUMN_Y_GAP, GUN_COLUMN_Y_GAP, GUN_COLUMN_Y_GAP, color);
	
	return GUN_COLUMN_Y_GAP;
}

pub fn strip_all_occurances(original: &str, character: char) -> String {
	return original.chars().filter(|&x| x != character).collect();
}

pub fn strip_superscript(original: &String) -> std::string::String {
	let mut result: String = Default::default();

	if original.contains("<sup>") {
		let re = Regex::new(r"(?<keep1>.*)(<sup>.*<\/sup>)(?<keep2>.*)").unwrap();
		let Some(caps) = re.captures(&original) else { panic!("strip_superscript regex failed!") };

		result.push_str(&caps["keep1"]);

		if !caps["keep2"].is_empty() {
			result.push_str(&caps["keep2"]);
		}
	}

	return result;
}

pub fn strip_dagger_and_any_superscript_from_end(original: &String) -> std::string::String {
	let mut result: String = original.to_string();

	if original.contains(DAGGER) {
		let (left, _right) = original.split_once(DAGGER).unwrap();

		result = left.to_string();
	}

	return result;
}

pub fn strip_opt(original: &String) -> std::string::String {
	let mut result = original.to_string();

	if original.contains("Opt") {
		let (left, _right) = original.split_once("Opt").unwrap();

		result = left.to_string();
	}

	return result;
}

pub fn strip_daggered_note(original: &String) -> std::string::String {
	let mut result = original.to_string();

	while result.contains(DAGGER) {
		if original.contains("†<sup>") {
			let re = Regex::new(r"†<sup>[0-9][0-9]*<\/sup>").unwrap();
	
			result = re.replace_all(original, "").to_string();
		} else if original.contains("†") {
			let re = Regex::new(r"†").unwrap();
	
			result = re.replace_all(original, "").to_string();
		}
	}	

	return result;
}

pub fn convert_superscripts(original: &String, pixels: f64) -> std::string::String {
	let mut result = original.to_string();

	while result.contains("<sup>") {
		let tspan: String = format!("<tspan style=\"font-size:{0}px;{FONT_WEIGHT_BOLD};font-family:{1}\" baseline-shift=\"super\">", &pixels.to_string(), &FONT_MAIN.to_string());

		result = original.replace("<sup>", &tspan);
		result = result.replace("</sup>", "</tspan>");
	}

	return result;
}

pub fn wrap_superscripts(original: &String, pixels: f64) -> std::string::String {
	return format!("<tspan style=\"font-size:{0}px;{FONT_WEIGHT_BOLD};font-family:{1}\" baseline-shift=\"super\">{2}</tspan>", &pixels.to_string(), &FONT_MAIN.to_string(), original);
}

pub fn mask_closing_html_tags(original: &String) -> std::string::String {
	let mut result = original.to_string();

	if result.contains("</") {
		result = result.replace("</", "<?");
	}

	return result;
}

pub fn unmask_closing_html_tags(original: &String) -> std::string::String {
	let mut result = original.to_string();

	if result.contains("<?") {
		result = result.replace("<?", "</");
	}

	return result;
}

pub fn extract_vector(source: &String, delimiter: char) -> Vec<String> {
	let mut result: Vec<std::string::String> = Default::default();

	if source.contains(delimiter) {
		let values = source.split(delimiter);

		for value in values {
			let mut temp: String = value.to_string();

			temp = temp.trim().to_string();
			result.push(temp);
		}
	} else {
		result.push(source.to_string());
	}

	return result;
}

pub fn extract_from(source: &str, target: &str) -> std::string::String {
	let mut result = String::from("");

	if source.contains(target) {
		let values = source.split(OVERRIDE_DELIMITER);
	
		for v in values {
			if v.contains(target) {
				result = (&v[target.len()..]).to_string();
			}
		}
	}

	return result;
}

pub fn extract_string(source: &String, before: &str, after: &str) -> std::string::String {
	let mut result = String::from("");
	let mut start_bytes = source.find(after).unwrap_or(usize::MAX);

	if usize::MAX != start_bytes {
		start_bytes += after.len();
		let end_bytes = source.find(before).unwrap_or(usize::MAX);

		if usize::MAX != end_bytes {
			if result.contains(",") {
				result.pop();
			}

			result = (&source[start_bytes..end_bytes]).to_string();
		} else {
			result = (&source[start_bytes..source.len()]).to_string();
		}
	}

	return result;
}

pub fn remove_string(source: &String, before: &str, after: &str) -> std::string::String {
	let mut result = String::from("");
	let mut temp = source.find(before).unwrap_or(usize::MAX);

	if usize::MAX != temp {
		result = (&source[0..temp]).to_string();
	}

	temp = source.find(after).unwrap_or(usize::MAX);
	if usize::MAX != temp {
		temp += 1;
		result.push_str(&source[temp..source.len()].to_string());
	}

	return result;
}
//
// Convert all occurances of 'old_text' in the string to 'new_text'.
//
pub fn convert_text(source: &String, old_text: &str, new_text: &str) -> std::string::String {
	return source.replace(old_text, new_text);
}

pub fn construct_path(nationality: &String, category: &'static str, destination: &String) -> std::string::String {
	return format!("{destination}{nationality}/{category}/"); // destination includes a trailing '/'	
}

pub fn construct_copy_paths(nationality: &String, category: &'static str, name: &String, destination: &String) -> Vec<String> {
	let mut result: Vec<std::string::String> = Default::default();

	let mut path = format!("./cached/{nationality}/{category}/{name}.svg");
	result.push(path.clone());	// Source.
	
	path = format!("{destination}{nationality}/{category}/{name}.svg"); // destination includes a trailing '/'	
	result.push(path.clone());	// Destination.	
	
	return result;
}

pub fn open_counter_file(path: &String, piece_name: &String) -> io::Result<File> {
	File::create(format!("{path}{piece_name}.svg"))
}

pub fn copy_counter(category: &'static str, nationality: &String, piece: &String, note_number: &String, destination: &String) -> io::Result<()> {
	print!("Copying '{0}.svg' ({1}) ...", piece, note_number);

	let paths: Vec<String> = construct_copy_paths(&nationality, &category, &piece, &destination);

	let source_file = match File::open(&paths[0]) {
		Err(why) => panic!("couldn't open file: {0} {1}", &paths[0], why),
		Ok(counter_file) => counter_file,
	};

	let mut destination_file = match File::create(&paths[1]) {
		Err(why) => panic!("couldn't create file: {0} {1}", &paths[1], why),
		Ok(counter_file) => counter_file,
	};

	let mut reader = BufReader::new(source_file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    destination_file.write_all(buffer.as_slice())?;

	println!(" done.");

	Ok(())
}

pub fn get_nationality(nationality_abbreviation: &String) -> std::string::String {
	let mut nationality = String::from("unknown");

	if "ge" == nationality_abbreviation {
		nationality = String::from("German");
	}

	return nationality;
}

pub fn strip_html_italics(original: &String) -> std::string::String {
	let re_html_i = Regex::new(r"(?<front>.*<i>)(?<keep>[a-zA-Z0-9/\-]*)(?<back><\/i>)").unwrap();
	let Some(caps) = re_html_i.captures(&original) else { panic!("strip_html_italics regex failed!") };

	return (&caps["keep"]).to_string();
}

pub fn strip_html_italics_only(original: &String) -> std::string::String {
	let mut result: String = original.to_string();

	result = result.replace("<i>", "");
	result = result.replace("</i>", "");

	return result;
}

pub fn strip_html_bold(original: &str) -> std::string::String {
	let mut result: String = Default::default();

	if original.contains("<b>") {
		let mut temp: String = Default::default();
		let mut left: &str;
		let mut right: &str;

		(left, right) = original.split_once("<b>").unwrap();
		temp.push_str(left);
		temp.push_str(right);

		(left, right) = temp.split_once("</b>").unwrap();
		result.push_str(left);
		result.push_str(right);
	} else {
		result = original.to_string();
	}

	return result;
}

pub fn get_destination_arg() -> std::string::String {
	let mut result: String = "./".to_string();
	let args: Vec<String> = env::args().collect();

	match args.len() {
		2 => {	// One argument passed, destination directory.
			result = args[1].clone();
			result.push_str("/"); // Make sure there's a trailing '/' --- lazy (TODO: for now)
		},
		_ => {
		}
	}

	return result;
}

pub fn generate_six_lobed_asterisk_svg(fonts: &FontsObj) -> std::string::String {
	return format!("<tspan style=\"font-size:{0}px;{FONT_WEIGHT_BOLD};font-family:{1}\" baseline-shift=\"super\">{2}</tspan>", &fonts.sup_size().to_string(), &FONT_ALT.to_string(), &SIX_LOBED_ASTERISK_UC);
}
