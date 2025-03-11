//
// Color choices.
//
pub const ALLIED_COLOR: &'static str =			"#82edbd";
pub const AMERICAN_COLOR: &'static str =		"#cddb42"; // Original: "#cdf000";
pub const AXIS_COLOR: &'static str =			"#1de256";
pub const BRITISH_COLOR: &'static str =			"#e5cea0";
pub const FINNISH_COLOR: &'static str =			"#ced3d3";
pub const FRENCH_COLOR: &'static str =			"#41a5ff";
pub const GERMAN_COLOR: &'static str =			"#91cdf5";
pub const ITALIAN_COLOR: &'static str =			"#a6adb2";
pub const JAPANESE_COLOR: &'static str =		"#ffdb00";
pub const RUSSIAN_COLOR: &'static str =			"#d68d1a";
pub const WAFFEN_SS_COLOR: &'static str =		"#000000";
pub const LAVENDER_SS_COLOR: &'static str =		"#e7cef7";
pub const SWEDISH_COLOR: &'static str =			"#629dcb";
pub const BOAT_COLOR: &'static str =			"#91cdf5"; // Same as GERMAN_COLOR
pub const SHARED_COLOR: &'static str =			"#ffffff";
pub const LANDED_GLIDER_COLOR: &'static str =	"#52A552";
pub const TEST_COLOR: &'static str =			"#ffc0ff";
pub const UNDEFINED_COLOR: &'static str =		"";

pub const BLACK: &'static str =	"black";
pub const WHITE: &'static str =	"white";
pub const RED: &'static str =	"red"; // TODO: or? "crimson";

#[derive(PartialEq)]
pub struct Colors {
	pub is_ss: bool,
	pub background: String,
	pub inner_background: String,
	pub turret_type: String,
	pub movement_type: String,
	pub movement_type_open_topped: String,
	pub text: String,
	pub movement_points_text: String,
	pub large_target: String,
	pub normal_target: String,
	pub small_target: String,
	pub small_target_circle: String,
	pub large_unarmored_target_stroke: String,
	pub large_unarmored_target_fill: String,
	pub normal_unarmored_target_stroke: String,
	pub normal_unarmored_target_fill: String,
	pub small_unarmored_target_stroke: String,
	pub small_unarmored_target_fill: String,
	pub armor_modifier: String,
	pub manhandling_fill: String,
	pub unhooking_penalty_color: String,
	pub malfunction_x: String,
}

impl Default for Colors {
	fn default() -> Colors {
		Colors {
			is_ss: false,
			background: TEST_COLOR.to_string(),
			inner_background: "".to_string(),
			turret_type: "white".to_string(),
			movement_type: "white".to_string(),
			movement_type_open_topped: "white".to_string(),
			text: "black".to_string(),
			movement_points_text: "black".to_string(),
			large_target: "#ff0000".to_string(),
			normal_target: "black".to_string(),
			small_target: "#ffffff".to_string(),
			small_target_circle: "white".to_string(),
			large_unarmored_target_stroke: "none".to_string(),
			large_unarmored_target_fill: "#ff0000".to_string(),
			normal_unarmored_target_stroke: "none".to_string(),
			normal_unarmored_target_fill: "black".to_string(),
			small_unarmored_target_stroke: "none".to_string(),
			small_unarmored_target_fill: "white".to_string(),
			manhandling_fill: "black".to_string(),
			armor_modifier: "black".to_string(),
			unhooking_penalty_color: "black".to_string(),
			malfunction_x: "white".to_string(),
		}
	}
}

pub fn nationality_to_color(nationality: &String) -> Vec<String> {
	let background_color: &str;
	let mut inner_background_color: &str = UNDEFINED_COLOR;

	match nationality.as_str() {
		"al" | "et"  => {
			background_color = ALLIED_COLOR;
		}
		"am" | "us" => {
			background_color = AMERICAN_COLOR;
		}
		"ax" => {
			background_color = AXIS_COLOR;
		}
		"br" => {
			background_color = BRITISH_COLOR;
		}
		"cc" => {
			background_color = RUSSIAN_COLOR;
			inner_background_color = BRITISH_COLOR;
		}
		"ch" => {
			background_color = RUSSIAN_COLOR;
			inner_background_color = GERMAN_COLOR;
		}
		"ff" | "fr" => {
			background_color = FRENCH_COLOR;
		}
		"fi" => {
			background_color = FINNISH_COLOR;
		}
		"ge" => {
			background_color = GERMAN_COLOR;
		}
		"hu" => {
			background_color = GERMAN_COLOR;
			inner_background_color = AXIS_COLOR;
		}
		"it" | "er" => {
			background_color = ITALIAN_COLOR;
		}
		"ja" => {
			background_color = JAPANESE_COLOR;
		}
		"nk" | "pa" | "ru" => {
			background_color = RUSSIAN_COLOR;
		}
		"sh" => {
			background_color = SHARED_COLOR;
		}
		"sk" => {
			background_color = AMERICAN_COLOR;
			inner_background_color = BRITISH_COLOR;
		}
		"ss" => {
			background_color = WAFFEN_SS_COLOR;
		}
		"sv" => {
			background_color = SWEDISH_COLOR;
		}
		"un" => {
			background_color = AMERICAN_COLOR;
			inner_background_color = FRENCH_COLOR;
		}
		"vf" => {
			background_color = GERMAN_COLOR;
			inner_background_color = FRENCH_COLOR;
		}
		"bt" => { // Landing craft and boats
			background_color = BOAT_COLOR;
		}
		"gd" => {
			background_color = LANDED_GLIDER_COLOR;
		}
		"jk" => {	// Japanese/Korean ?
			background_color = RUSSIAN_COLOR;
			inner_background_color = JAPANESE_COLOR;
		}
		&_ => {
			background_color = TEST_COLOR;
		}
	}
	
	let mut result: Vec<std::string::String> = Default::default();

	result.push(background_color.to_string());
	result.push(inner_background_color.to_string());

	return result;
}

pub fn nationality_to_colors(nationality: &String) -> Colors {
	let mut result: Colors = Default::default();
	let colors_v = nationality_to_color(nationality);
	
	result.background = colors_v[0].clone();
	result.inner_background = colors_v[1].clone();

	if "sh" == nationality {
		result.movement_type = "#b4b4b4".to_string();
		result.small_unarmored_target_stroke = "black".to_string();
	} else if "ss" == nationality {
		result.is_ss = true;
		result.turret_type = "#b4b4b4".to_string();
		result.movement_type = "#b4b4b4".to_string();
		result.movement_type_open_topped = "#8ccdf5".to_string();
		result.text = "white".to_string();
		result.normal_target = "white".to_string();
		result.small_target_circle = "#b4b4b4".to_string();
		result.normal_unarmored_target_stroke = "white".to_string();
		result.armor_modifier = "white".to_string();
		result.manhandling_fill = "white".to_string();
		result.unhooking_penalty_color = "white".to_string();
		result.malfunction_x = "#b4b4b4".to_string();
	}
	
	return result;
}
