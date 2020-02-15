
extern crate cluEscSequence;

use cluEscSequence::EscSequence;
use cluEscSequence::colors::Blue;
use std::io;

fn main() -> Result<(), io::Error> {
	let color_data = Blue::data();
	println!("{}test", color_data);
	color_data.reset_println()?;
	
	println!("End.");
	
	Ok( () )
}
