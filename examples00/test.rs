
use cluExtIO::FmtORIoErr;
use cluColor::color_value::font_back::Back_Yellow_Color;
use std::fmt::Write;
use cluColor::color_value::font_color::Black_Color;
use cluColor::head_args::Bold;
use std::io;
use std::fmt;
use cluExtIO::analytics::CounterFMTWrite;
use cluExtIO::analytics::CounterIOWrite;
use cluColor::write::ColorBuilder;

fn main() -> Result<(), io::Error> {
	let stdout = io::stdout();
	let mut handle = CounterIOWrite::from(stdout.lock());
	
	{
		let mut builder = ColorBuilder::from(&mut handle);
	
		//builder.write_low_color::<(Bold), (Black_Color)>().unwrap();
		builder.write_head::<(Bold)>()?;
		writeln!(builder, "Test")?;
		writeln!(builder, "Test")?;
		writeln!(builder, "Test")?;
		writeln!(builder, "Test")?;
		
		//builder.write_low_color::<(Bold), (Back_Yellow_Color)>().unwrap();
		writeln!(builder, "Test")?;
		
		//builder.write_stat(&mut handle)?;
	}
	//println!("len:{} capacity:{}", str.len(), str.capacity());
	println!("123");
	Ok( () )
}

