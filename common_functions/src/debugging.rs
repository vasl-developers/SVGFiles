use std::io::prelude::*;
//
// Local files.
//
use crate::text_field::*;

pub const DEBUG_GRID: bool =			false;
pub const DEBUG_GUN_LINE: bool =		false;
pub const DEBUG_LAYOUT: bool =			false;
pub const DEBUG_WORKING_AREA: bool =	false;

pub const DEBUG_NOTE_FONT_SIZE: f64 =		16.0;
pub const DEBUG_NOTE_FONT_ALT_SIZE: f64 =	14.0;

#[macro_export]
macro_rules! debug_layout {
	($counter_file:expr, $depth:expr, $color:expr) => {
		if DEBUG_LAYOUT {
			write!($counter_file, "{0}<rect x=\"0.00\" y=\"0.00\" ry=\"0.00\" width=\"100%\" height=\"100%\" style=\"display:inline;fill:{1};fill-opacity:0.25;stroke:pink;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect> <!-- Debugging Layout -->\n", "\t".repeat($depth.try_into().unwrap()), $color).unwrap();
		}
	}
}

#[macro_export]
macro_rules! debug_rectangle {
	($counter_file:expr, $depth:expr, $x:expr, $y:expr, $width:expr, $height:expr, $color:expr) => {
		if DEBUG_LAYOUT {
			write!($counter_file, "{0}<rect x=\"{1:.2}\" y=\"{2:.2}\" ry=\"0.00\" width=\"{3:.2}\" height=\"{4:.2}\" style=\"display:inline;fill:{5};fill-opacity:1.0;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1.0\"></rect> <!-- Debug Rectangle -->\n", "\t".repeat($depth.try_into().unwrap()), $x, $y, $width, $height, $color).unwrap();
		}
	}
}

pub fn generate_debug_note_svg(mut counter_file: &std::fs::File, note: &String) {
	let font_size: f64 = if 2 >= note.len() { DEBUG_NOTE_FONT_SIZE } else { DEBUG_NOTE_FONT_ALT_SIZE };
	
	write!(counter_file, "\t<!-- Chapter H Note # -->\n").unwrap();
	write!(counter_file, "\t<text x=\"50%\" y=\"57%\" dominant-baseline=\"auto\" text-anchor=\"middle\"><tspan style=\"font-size:{0:.2}px;font-weight:normal;font-family:{1};fill:red;fill-opacity:1;stroke:white;stroke-opacity:1;stroke-width:0.4\">{2}</tspan></text>\n", font_size, FONT_MAIN, note).unwrap();
}

pub fn generate_debug_grid_svg(mut counter_file: &std::fs::File) {
	if DEBUG_GRID {
		let mut x_position = 0;
		let mut y_position = 0;
		let mut color;

		write!(counter_file, "\n\t\t<!-- Debugging grid (begin) -->\n").unwrap();
		while x_position < 1000 {
			if 0 == (x_position % 100) {
				color = "red".to_string();
			} else {
				color = "yellow".to_string();
			}

			write!(counter_file, "\t\t<rect x=\"{0}\" y=\"0\" width=\"1\" height=\"1000\" style=\"display:inline;fill:{1};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", x_position, color).unwrap();
			write!(counter_file, "\t\t<rect x=\"0\" y=\"{0}\" width=\"1000\" height=\"1\" style=\"display:inline;fill:{1};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", y_position, color).unwrap();

			x_position += 50;
			y_position += 50;
		}
		write!(counter_file, "\t\t<!-- Debugging grid (end) -->\n\n").unwrap();
	}
}

pub fn generate_debug_gun_line_svg(mut counter_file: &std::fs::File) {
	if DEBUG_GUN_LINE {
		write!(counter_file, "\t<svg x=\"0\" y=\"57\" width=\"60\" height=\"1\">\n").unwrap();
		write!(counter_file, "\t\t<rect x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" style=\"fill:yellow\"/>\n").unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}
}

pub fn generate_debug_working_area_svg(mut counter_file: &std::fs::File) {
	if DEBUG_WORKING_AREA {
		write!(counter_file, "\t<svg x=\"0\" y=\"0\" width=\"60\" height=\"60\">\n").unwrap();
		write!(counter_file, "\t\t<rect x=\"0\" y=\"0\" width=\"60\" height=\"60\" style=\"fill:red\"/>\n").unwrap();
		write!(counter_file, "\t\t<rect x=\"1\" y=\"1\" width=\"58\" height=\"58\" style=\"fill:white\"/>\n").unwrap();
		write!(counter_file, "\t\t<rect x=\"2\" y=\"2\" width=\"56\" height=\"56\" style=\"fill:blue\"/>\n").unwrap();
		write!(counter_file, "\t\t<rect x=\"3\" y=\"3\" width=\"54\" height=\"54\" style=\"fill:cyan\"/>\n").unwrap();
		write!(counter_file, "\t</svg>\n").unwrap();
	}
}