
extern crate ColorGeneric;

use ColorGeneric::ColorGeneric;
use ColorGeneric::colors::BrightRed;
use std::time::SystemTime;

fn main() {
	
	let time_start = SystemTime::now();
	println!("Start time {:?} nanos", time_start.elapsed().unwrap().subsec_nanos());
	

	let writer = BrightRed::writer();

	println!("#1 {}", test(&writer, "Test"));
	println!("#2 {}", test(&writer, "Test2"));
	println!("#3 {}", test(writer, "Test move"));
}

fn test<C: ColorGeneric>(_c: C, str: &str) -> String {
	C::string_fmt( format_args!("[{:?} nanos] {}", SystemTime::now().elapsed().unwrap().subsec_nanos(), str) )
}




