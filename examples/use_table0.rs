
#[macro_use]
extern crate cluColor;

use std::io;
use std::io::Write;
use std::fmt;

use cluColor::colors::Red;
use cluColor::colors::BackWhite;
use cluColor::heads::Bold;
use cluColor::heads::Flashing;
use cluColor::FontSeqLen;
use cluColor::FontSeqReset;
use cluColor::FontSeqEscape;
use cluColor::FontSeqWrite;
use cluExtIO::drop_write::DropWriteFlush;

fontseg_table! {
	TwoReset(FontSeqReset, FontSeqReset)
	BoldFlashing(Flashing, Bold)
	EmptyBoldFlashing(Flashing, Bold, FontSeqReset)
	
	TestOut(BackWhite, Red)
	TestOut2(Red)
}

fn main() -> Result<(), IoOrFmtErr> {
	{
		let mut vec = Vec::with_capacity(12);
		TwoReset::io_write(&mut vec).unwrap();
		
		assert_eq!(TwoReset::ALL_LEN, "\x1b[0;0m".len());
		assert_eq!(vec, b"\x1b[0;0m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
	}
	
	{
		let mut vec = Vec::with_capacity(12);
		BoldFlashing::io_write(&mut vec)?;
		
		assert_eq!(BoldFlashing::ALL_LEN, "\x1b[5;1m".len());
		assert_eq!(vec, b"\x1b[5;1m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
		
		
		let mut str = String::with_capacity(12);
		BoldFlashing::fmt_write(&mut str)?;
		
		assert_eq!(BoldFlashing::ALL_LEN, "\x1b[5;1m".len());
		assert_eq!(str, "\x1b[5;1m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
	}
	
	{
		let mut vec = Vec::with_capacity(12);
		EmptyBoldFlashing::io_write(&mut vec)?;
		
		assert_eq!(EmptyBoldFlashing::ALL_LEN, b"\x1b[5;1;0m".len());
		assert_eq!(vec, b"\x1b[5;1;0m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
		
		
		let mut str = String::with_capacity(12);
		EmptyBoldFlashing::fmt_write(&mut str)?;
		
		assert_eq!(EmptyBoldFlashing::ALL_LEN, "\x1b[5;1;0m".len());
		assert_eq!(str, "\x1b[5;1;0m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
	}
	
	{
		let stdout = std::io::stdout();
   		let mut handle = stdout.lock();
		
		{
			let mut safe_out = FontSeqWrite::new(&mut handle as &mut std::io::Write);
			safe_out.write::<TestOut>()?;
			
			write!(safe_out, "WhiteRed")?;
		}
		handle.flush()?;
	}
	{
		let stdout = std::io::stdout();
   		let mut handle = stdout.lock();
		
		{
			let mut safe_out = FontSeqWrite::new(&mut handle as &mut std::io::Write);
			safe_out.write::<TestOut2>()?;
			
			write!(safe_out, "Red")?;
		}
		
		handle.flush()?;
	}
	
	Ok( () )
}



#[derive(Debug)]
pub enum IoOrFmtErr {
	IO(io::Error),
	FMT(fmt::Error)	
}

impl From<io::Error> for IoOrFmtErr {
	#[inline(always)]
	fn from(a: io::Error) -> Self {
		IoOrFmtErr::IO(a)
	}
}

impl From<fmt::Error> for IoOrFmtErr {
	#[inline(always)]
	fn from(a: fmt::Error) -> Self {
		IoOrFmtErr::FMT(a)
	}
}