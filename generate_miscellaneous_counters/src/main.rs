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
use common_functions::arguments::*;
use common_functions::overrides::*;
use common_functions::utils::*;
//
// Sanitized and parsed miscellaneous record fields.
//
#[derive(Default)]
struct Record {
	args: Arguments,
	folder: String,
	piece: String,
	version: String,
	overrides: Overrides,
	svg_image_transform: String,
	comments: String,
}

fn process_counter(record: &mut Record) {
	if record.overrides.copy {
		let _ = copy_counter("", &record.folder, &record.piece, &"".to_string(), &record.args);
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
	folder: String,
	piece: String,
	version: String,
	overrides: String,
	svg_image_transform: String,
	comments: String,
}

impl SpreadsheetRecord {
	fn sanitize(&mut self, args: &Arguments) -> Record {
		let mut result: Record = Default::default();
		
		result.args = args.clone();

		result.overrides.sanitize(&self.overrides);

		result.folder = self.folder.to_string();
		
		result.piece = self.piece.to_string();

		result.version = self.version.to_string();
		
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

		if !spreadsheet_record.folder.is_empty() {
			let mut record: Record = spreadsheet_record.sanitize(&args);
			
			process_counter(&mut record);
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
