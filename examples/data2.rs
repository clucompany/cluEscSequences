
extern crate cluEscSequency;

use cluEscSequency::EscSequency;
use cluEscSequency::colors::Blue;
use std::io;




pub struct MyWriter {
	
}

fn main() -> Result<(), io::Error> {
	let color_data = Blue::data();
	println!("{}test", color_data);
	color_data.reset_println()?;
	
	println!("End.");
	
	Ok( () )
}
