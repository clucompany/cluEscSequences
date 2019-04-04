
extern crate ColorGeneric;

use ColorGeneric::ColorGeneric;
use ColorGeneric::colors::BrightRed;
use std::time::SystemTime;

fn main() {
	
	let time_start = SystemTime::now();
	println!("Start time {:?} nanos", time_start.elapsed().unwrap().subsec_nanos());
	
	let writer = MyWriter::new(time_start);
	writer.println("OK");
	writer.println("OK12");
	writer.println("OK123");
}


#[derive(Debug)]
pub struct MyWriter {
	time: SystemTime,
}

impl MyWriter {
	#[inline]
	pub fn new(time: SystemTime) -> MyWriter {
		MyWriter {
			time: time,
		}
	}
	
	#[inline]
	pub fn string<'a>(&self, str: &'a str) -> String {
		BrightRed::string_fmt( format_args!("[{:?} nanos] {}", SystemTime::now().elapsed().unwrap().subsec_nanos(), str) )
	}
	
	pub fn println<'a>(&self, str: &'a str) {
		println!("{}", self.string(str));
	}
}



