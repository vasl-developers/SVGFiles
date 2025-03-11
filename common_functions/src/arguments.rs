use clap::Parser;
//
// Sanitized and parsed program arguments.
//
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Arguments {
	#[arg(long)]
	pub destination: String,
	
	#[arg(long)]
	pub debug: bool,

	#[arg(long)]
	pub notes: bool,
	
	#[arg(long)]
	pub quiet: bool,
}

impl Clone for Arguments {
	fn clone(&self) -> Self {
		Self {
			destination: String::from(&self.destination),
			debug: self.debug,
			notes: self.notes,
			quiet: self.quiet
		}  
	}
}

impl Arguments {
	pub fn sanitize_destination(&mut self) {
		self.destination.push_str("/"); // Make sure there's a trailing '/' --- lazy (TODO: for now)
	}
}
