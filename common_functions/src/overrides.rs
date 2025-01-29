//
// Local files.
//
use crate::utils::*;

//
// Override constants.
//
pub const OVERRIDE_DELIMITER: char =	'|';		// Delimiter to separate multiple overrides.

pub const NOVR_ANNOUNCE: &str =				"announce";			// Announce the line.
pub const NOVR_SPECIAL_AMMO: &str =			"ammo=";			// Special ammunition values, override values with MOD_TEXT (include '[' and ']'), can also specify alternate location (MOD_LOCATION) and font size (MOD_FONT_SIZE).
pub const NOVR_BACKGROUND_COLOR: &str =		"bkg=";				// Override the counter's background color.
pub const NOVR_CAPTURED: &str =				"cap=";				// Piece is captured.
pub const NOVR_COPY: &str =					"copy";	
pub const NOVR_DISPLAY_NAME: &str =			"display_name";		// Display name (overrides INCLUDE_NAME)
pub const NOVR_FIXED_BMG: &str =			"fixed_bmg";		// Fixed BMG. Following the '@' is the x-axis center of the white circle that should be displayed behind the BMG factor.
pub const NOVR_GP: &str =					"gp=";	
pub const NOVR_GT: &str =					"gt=";	
pub const NOVR_IFE: &str =					"ife=";	
pub const NOVR_IGNORE: &str =				"ignore";			// Ignore entry (or reverse counter creation).
pub const NOVR_MA: &str =					"ma=";				// Main armament.
pub const NOVR_MANHANDLING: &str =			"man=";				// Manhandling.
pub const NOVR_MA_MOVING_TARGET: &str =		"ma_movt";			// Main armament moving target penalty.
pub const NOVR_MB: &str =					"mb=";				// Main armament breakdown.
pub const NOVR_MGS: &str =					"mgs=";				// Override BMG/CMG/AAMG values.
pub const NOVR_MOUNT: &str =				"mount=";			// Mount or turret type (e.g., T - "Fast Turret", NT - "Non-turreted", etc.
pub const NOVR_MP: &str =					"mp=";	
pub const NOVR_MT: &str =					"mt=";				// Override movement type normally extracted from "name" field.
pub const NOVR_MULTIPLE_HITS: &str =		"multi_hits";		// Main armament is eligible for multiple hits.
pub const NOVR_NATIONALITY: &str =			"nat=";				// Provides nationality for vehicle counters that don't include it in the piece name like ordnance pieces do and for Axis Minors which have unique prefixes.
pub const NOVR_NAME: &str =					"name=";			// Replace name on counter and in SVG documentation.
pub const NOVR_PP_NUMBER: &str =			"pp=";				// PP number.
pub const NOVR_QUALIFIER: &str =			"qual=";	
pub const NOVR_RANGE: &str =				"range=";			// Show range value(s), override values with MOD_TEXT (include '[' and ']'), can also specify alternate location (MOD_LOCATION) and font size (MOD_FONT_SIZE).
pub const NOVR_RANGE2: &str =				"range2=";			// Show alternate range value(s), override values with MOD_TEXT (include '[' and ']'), can also specify alternate location (MOD_LOCATION) and font size (MOD_FONT_SIZE).
pub const NOVR_ROF: &str =					"rof=";	
pub const NOVR_SA: &str =					"sa=";				// Secondary armament.
pub const NOVR_SA_MOVING_TARGET: &str =		"sa_movt";			// Secondary armament moving target penalty.
pub const NOVR_SB: &str =					"sb=";				// Secondary armament breakdown.
pub const NOVR_SHIFT_ARMOR: &str =			"shift_armor_down";	// Shift armor values down.
pub const NOVR_SIZE: &str =					"size=";			// Target size.
pub const NOVR_TA: &str =					"ta=";				// Turret armor modifies (Superior, inferior ...).
pub const NOVR_TOWING_NUMBER: &str =		"tow=";				// Towing number.
pub const NOVR_ARMOR_FRONT: &str =			"far=";				// Front armor.
pub const NOVR_ARMOR_SIDE: &str =			"sar=";				// Side armor.
pub const NOVR_ARMOR_REAR: &str =			"rar=";				// Rear armor.
//
// Modifiers to new hotness overrides.
//
pub const MOD_DELIMITER1: char =		':';	// Delimiter to separate multiple modifiers.
pub const MOD_DELIMITER2: char =		'@';	// Delimiter to separate multiple modifiers.
// TODO DEPRECATED? //
// TODO DEPRECATED? // Modifiers for sticking asterisks in various and sundry places.
// TODO DEPRECATED? //
// TODO DEPRECATED? pub const MOD_NOTES: &str =				"$notes=";	// Add "Note" asterisk to element(s).
// TODO DEPRECATED? pub const MOD_NOTES_SEPARATOR: char =	'.';		// Separates "pre/post/in/del" from "asterisk".
pub const MOD_NOTES_PREFIX: &str =		"pre";		// Places "Note" before the field's data.
pub const MOD_NOTES_POSTFIX: &str =		"post";		// Places "Note" after the field's data.
pub const MOD_NOTES_INFIX: &str =		"in";		// Places "Note" wherever the element's generation code decides.
// TODO DEPRECATED? pub const MOD_NOTES_DELETE: &str =		"del";		// Deletes all "Notes" in the field.
//
// Alternate positions for fields (prefix with MOD_DELIMITER2).
//
pub const MOD_LOCATION_GS: &str =				"in_gs";		// Display in the "gun stack".
pub const MOD_LOCATION_MGS: &str =				"mgs";			// Display where the MGs would be displayed.
pub const MOD_LOCATION_ABOVE_MGS: &str =		"above_mgs";	// Display above where the MGs would be displayed (where T# is usually shown).
pub const MOD_LOCATION_BEFORE_TOWING: &str =	"before_tow";	// Display before where the T# is usually shown.
//
// Alternate font sizes for fields (prefix with MOD_DELIMITER2).
//
pub const MOD_INC_SIZE: &str =	"sz+";	// Positive delta to the normal font size.
pub const MOD_DEC_SIZE: &str =	"sz-";	// Negative delta to the normal font size.

pub const LIMBERED_NO_FIRE: &str =	"no_fire";

#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct MalfunctionOverrides {
	pub ignore: bool,
	pub text: String,
	pub notes: String,
}

#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct ArmamentOverrides {
	pub ignore: bool,
	pub text: String,
	pub mount: String,
	pub multiple_hits: bool,
	pub moving_target_penalty: bool,
	pub malf: MalfunctionOverrides,
}

#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct Overrides {
	pub announce: bool,
	pub shift_armor_down: bool,
	pub armor_front: String,
	pub armor_rear: String,
	pub armor_side: String,
	pub background_color: String,
	pub captured: String,
	pub copy: bool,
	pub display_name: bool,
	pub fixed_bmg: bool,
	pub ground_pressure: String,
	pub gt: String,
	pub ife: String,
	pub ignore: bool,
	pub ma: ArmamentOverrides,
	pub machine_guns: String,
	pub machine_guns_set: bool,
	pub manhandling: String,
	pub movement_type: String,
	pub mp: String,
	pub name: String,
	pub nationality: String,
	pub nm: bool,
	pub note_qualifier: String,
	pub pp_number: String,
	pub pp_number_ignore: bool,
	pub rfnm: bool,
	pub range_values: String,
	pub range2_values: String,
	pub rof: String,
	pub sa: ArmamentOverrides,
	pub target_size: String,
	pub special_ammo: String,
	pub towing_number: String,
	pub turret_armor_modifiers: String,
}

impl Overrides {
	pub fn sanitize(&mut self, overrides: &String) {
		if !overrides.is_empty() {
			let entries: Vec<std::string::String> = extract_vector(&overrides, OVERRIDE_DELIMITER);
			
			for entry in entries {
				if entry.contains(NOVR_ANNOUNCE) {
					self.announce = true;
				} else if NOVR_SHIFT_ARMOR == entry {
					self.shift_armor_down = true;
				} else if entry.contains(NOVR_ARMOR_FRONT) {
					self.armor_front = extract_from(&entry, NOVR_ARMOR_FRONT);
				} else if entry.contains(NOVR_ARMOR_REAR) {
					self.armor_rear = extract_from(&entry, NOVR_ARMOR_REAR);
				} else if entry.contains(NOVR_ARMOR_SIDE) {
					self.armor_side = extract_from(&entry, NOVR_ARMOR_SIDE);
				} else if entry.contains(NOVR_BACKGROUND_COLOR) {
					self.background_color = extract_from(&entry, NOVR_BACKGROUND_COLOR);
				} else if entry.contains(NOVR_CAPTURED) {
					self.captured = extract_from(&entry, NOVR_CAPTURED);
				} else if entry.contains(NOVR_COPY) {
					self.copy = true;
				} else if entry.contains(NOVR_DISPLAY_NAME) {
					self.display_name = true;
				} else if entry.contains(NOVR_FIXED_BMG) {
					self.fixed_bmg = true;
				} else if entry.contains(NOVR_GP) {
					self.ground_pressure = extract_from(&entry, NOVR_GP);
				} else if entry.contains(NOVR_GT) {
					self.gt = extract_from(&entry, NOVR_GT);
				} else if entry.contains(NOVR_IFE) {
					self.ife = entry; // PASS whole entry so we can recognize override to set IFE to "". extract_from(&entry, NOVR_IFE);
				} else if entry.contains(NOVR_IGNORE) {
					self.ignore = true;
				} else if entry.contains(NOVR_MA) {
					self.ma.text = extract_from(&entry, NOVR_MA);
					self.ma.ignore = self.ma.text.is_empty();
				} else if entry.contains(NOVR_MA_MOVING_TARGET) {
					self.ma.moving_target_penalty = true;
				} else if entry.contains(NOVR_MANHANDLING) {
					self.manhandling = extract_from(&entry, NOVR_MANHANDLING);
				} else if entry.contains(NOVR_MB) {
					self.ma.malf.text = extract_from(&entry, NOVR_MB);
					self.ma.malf.ignore = self.ma.malf.text.is_empty();
				} else if entry.contains(NOVR_MGS) {
					self.machine_guns_set = true;
					self.machine_guns = extract_from(&entry, NOVR_MGS);
				} else if entry.contains(NOVR_MOUNT) {
					self.ma.mount = extract_from(&entry, NOVR_MOUNT);
				} else if entry.contains(NOVR_MP) {
					self.mp = extract_from(&entry, NOVR_MP);
					
					if entry.contains("RFNM") {
						self.rfnm = true;
					} else if entry.contains("NM") {
						self.nm = true;
					}
				} else if entry.contains(NOVR_MT) {
					self.movement_type = extract_from(&entry, NOVR_MT);
				} else if entry.contains(NOVR_MULTIPLE_HITS) {
					self.ma.multiple_hits = true;
				} else if entry.contains(NOVR_NAME) {
					self.name = extract_from(&entry, NOVR_NAME);
				} else if entry.contains(NOVR_NATIONALITY) {
					self.nationality = extract_from(&entry, NOVR_NATIONALITY);
				} else if entry.contains(NOVR_PP_NUMBER) {
					self.pp_number = extract_from(&entry, NOVR_PP_NUMBER);
					self.pp_number_ignore = self.pp_number.is_empty();
				} else if entry.contains(NOVR_QUALIFIER) {
					self.note_qualifier = extract_from(&entry, NOVR_QUALIFIER);
				} else if entry.contains(NOVR_RANGE) {
					self.range_values = extract_from(&entry, NOVR_RANGE);
				} else if entry.contains(NOVR_RANGE2) {
					self.range2_values = extract_from(&entry, NOVR_RANGE2);					
				} else if entry.contains(NOVR_ROF) {
					self.rof = entry; // PASS whole entry so we can recognize override to set ROF to "".
				} else if entry.contains(NOVR_SA) {
					self.sa.text = extract_from(&entry, NOVR_SA);
					self.sa.ignore = self.sa.text.is_empty();
				} else if entry.contains(NOVR_SB) {
					self.sa.malf.text = extract_from(&entry, NOVR_SB);
					self.sa.malf.ignore = self.sa.malf.text.is_empty();
				} else if entry.contains(NOVR_SA_MOVING_TARGET) {
					self.sa.moving_target_penalty = true;
				} else if entry.contains(NOVR_SIZE) {
					self.target_size = extract_from(&entry, NOVR_SIZE);
				} else if entry.contains(NOVR_SPECIAL_AMMO) {
					self.special_ammo = extract_from(&entry, NOVR_SPECIAL_AMMO);
				} else if entry.contains(NOVR_TOWING_NUMBER) {
					self.towing_number = extract_from(&entry, NOVR_TOWING_NUMBER);
				} else if entry.contains(NOVR_TA) {
					self.turret_armor_modifiers = extract_from(&entry, NOVR_TA);
				} else {
					panic!("Overrides.santize()@{0}: unimplemented! entry '{1}'", line!(), entry);
				}
			}
		}
	}
}

pub fn is_alternate_location(value: std::string::String) -> bool {
	return MOD_LOCATION_GS == value || MOD_LOCATION_MGS == value || MOD_LOCATION_ABOVE_MGS == value || MOD_LOCATION_BEFORE_TOWING == value;
}
