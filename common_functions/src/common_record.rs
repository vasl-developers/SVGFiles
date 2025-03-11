use regex::Regex;
//
// Local defines.
//
use crate::armament::*;
use crate::colors::*;
use crate::malfunction::*;
use crate::overrides::*;
use crate::turret::*;
use crate::utils::*;
//
// Sanitized and parsed record (common elements).
//
#[derive(Default)]
pub struct CommonRecord {
	pub nationality: String,
	pub note: String,
	pub name: String,
	pub display_name: bool,
	pub turret: Turret,
	pub ma: Armament,
	pub malfunction: Malfunction,
	pub version: String,
	pub pieces: Vec<std::string::String>,
	pub piece_front: String,
	pub overrides: Overrides,
	pub comments: String,
	pub colors: Colors,
	pub svg_image_transform: String,
}

impl CommonRecord {
	pub fn initialize(&mut self, nationality: &String, notes: &String, name: &String, ma: &String, range: &String, rof_ife: &String, breakdown: &String, version: &String, piece: &String, svg_image_transform: &String, comments: &String) {
		self.nationality = nationality.to_string();

		if !self.overrides.background_color.is_empty() {
			self.colors = nationality_to_colors(&self.overrides.background_color);
		} else {
			self.colors = nationality_to_colors(&nationality);
		}

		self.note = extract_note_number(&notes);
		
		if !self.overrides.note_qualifier.is_empty() {
			self.note.push_str(&self.overrides.note_qualifier);
		}
		
		if !self.overrides.name.is_empty() {
			self.name = self.overrides.name.to_string();
		} else {
			self.name = name.to_string();
		}

		self.ma.sanitize(&ma, &range, &rof_ife, &self.overrides, &self.colors);	
		self.malfunction.sanitize(&breakdown, &self.overrides.ma, &self.colors);
		self.version = version.to_string();

		self.pieces = extract_vector(&piece, OVERRIDE_DELIMITER);

		// TODO NOT YET ?!? if !self.overrides.piece_back.is_empty() {
		// TODO NOT YET ?!? 	self.piece_back = self.overrides.piece_back.clone();
		// TODO NOT YET ?!? } else {
		// TODO NOT YET ?!? 	self.piece_back = piece.to_string();
		// TODO NOT YET ?!? }

		self.svg_image_transform = svg_image_transform.to_string();
		
		self.comments = comments.to_string();
		
		self.display_name = self.overrides.display_name;
	}
}

pub fn extract_note_number(source: &String) -> std::string::String {
	let re = Regex::new(r"(?<keep>[0-9gv][0-9a-zA-Z\.]*)").unwrap();
	let Some(caps) = re.captures(&source) else { panic!("extract_note_number regex failed!") };

	return (&caps["keep"]).to_string();
}
