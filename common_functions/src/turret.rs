use std::fmt;
use std::write;
use std::io::prelude::*;
//
// Local files.
//
use crate::colors::*;
use crate::overrides::*;
use crate::utils::*;

#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub enum TurretType {
	#[default]
	NonTurreted,
	FastTurret, // 360 degree mount for guns
	SlowTurret,
	RestrictedSlowTurret,
	OneManTurret,
}

impl fmt::Display for TurretType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TurretType::NonTurreted => write!(f, "NonTurreted"),
            TurretType::FastTurret => write!(f, "FastTurret"),
            TurretType::SlowTurret => write!(f, "SlowTurret"),
            TurretType::RestrictedSlowTurret => write!(f, "RestrictedSlowTurret"),
			TurretType::OneManTurret => write!(f, "OneManTurret"),
        }
    }
}

#[derive(PartialEq)]
#[derive(Default)]
pub struct Turret {
	pub speed: TurretType,
	pub color: String,
}

impl Turret {
	pub fn sanitize(&mut self, source: &String, overrides: &Overrides, colors: &Colors) {
		if !overrides.ma.ignore {
			self.speed = extract_turret_type(&source, &overrides);
			self.color = colors.turret_type.to_string();
		}
	}
	

	pub fn generate_svg_elements(&self, mut counter_file: &std::fs::File) {
		match self.speed {
			TurretType::OneManTurret => {
				generate_svg_start_element(counter_file, 1, 0.00, 0.00, 60.0, 60.0, "One Man Turret - It is the way (unless there's a better way)", "white");
				write!(counter_file, "\t\t\t<rect x=\"13.20\" y=\"4.80\" width=\"33.60\" height=\"2.40\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.color).unwrap();
				write!(counter_file, "\t\t\t<rect x=\"52.60\" y=\"13.20\" width=\"2.40\" height=\"33.60\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.color).unwrap();
				write!(counter_file, "\t\t\t<rect x=\"13.20\" y=\"52.60\" width=\"33.60\" height=\"2.40\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.color).unwrap();
				write!(counter_file, "\t\t\t<rect x=\"4.80\" y=\"13.20\" width=\"2.40\" height=\"33.60\" style=\"display:inline;fill:{0};fill-opacity:1;stroke:none;stroke-width:1;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.color).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			}
			TurretType::RestrictedSlowTurret => {
				generate_svg_start_element(counter_file, 1, 0.00, 0.00, 60.0, 60.0, "Restricted Slow Turret", "white");
				write!(counter_file, "\t\t<rect x=\"6.00\" y=\"6.00\" width=\"48.00\" height=\"48.00\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:2.4;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.color).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			}
			TurretType::SlowTurret => {
				generate_svg_start_element(counter_file, 1, 0.00, 0.00, 60.0, 60.0, "Slow Turret", "white");
				write!(counter_file, "\t\t<rect x=\"6.00\" y=\"6.00\" width=\"48.00\" height=\"48.00\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:1.8;stroke-dasharray:none;stroke-opacity:1\"></rect>\n", self.color).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			}
			TurretType::FastTurret => {
				generate_svg_start_element(counter_file, 1, 0.00, 0.00, 60.0, 60.0, "Fast Turret", "white");
				write!(counter_file, "\t\t<circle cx=\"30.00\" cy=\"30.00\" r=\"25.00\" style=\"display:inline;fill:none;fill-opacity:1;stroke:{0};stroke-width:1.8;stroke-dasharray:none;stroke-opacity:1\"></circle>\n", self.color).unwrap();
				write!(counter_file, "\t</svg>\n").unwrap();
			}
			TurretType::NonTurreted => {
			}
		}
	}	
}

pub fn string_to_turret_type(mount: &String) -> TurretType {
	let mut result = TurretType::NonTurreted;

	if mount.contains("1MT") {
		result = TurretType::OneManTurret;
	} else if mount.contains("RST") {
		result = TurretType::RestrictedSlowTurret;
	} else if mount.contains("ST") {
		result = TurretType::SlowTurret;
	} else if mount.contains("NT") {
		result = TurretType::NonTurreted;
	} else if mount.contains("T") {
		result = TurretType::FastTurret;
	}

	return result;
}

pub fn extract_turret_type(source: &String, overrides: &Overrides) -> TurretType {
	let mut mount = source.to_string();

	if !overrides.ma.mount.is_empty() {
		mount = overrides.ma.mount.clone();
	} else if source.contains(",") {
		let left: &str;
		let _right: &str;
		(left, _right) = source.split_once(",").unwrap();

		mount = left.to_string();
	} else if source.contains("-") { // Thanks M3 (Lee & Grant)!
		let left: &str;
		let _right: &str;
		(left, _right) = source.split_once("-").unwrap();

		mount = left.to_string();
	}

	return string_to_turret_type(&mount);
}
