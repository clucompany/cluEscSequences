
#[macro_use]
extern crate cluEscSequence;

use cluEscSequence::EscSequence;
use cluEscSequence::EscLenSeq;
use cluEscSequence::colors::Red;
use cluEscSequence::back_colors::BackWhite;
use cluEscSequence::heads::Bold;
use cluEscSequence::heads::Flashing;
use cluEscSequence::EscSeqWrite;
use std::io;
use std::io::Write;
use std::fmt;
use cluEscSequence::EscSeqReset;


escseq_scheme! {
	TwoReset(EscSeqReset, EscSeqReset)
	BoldFlashing(Flashing, Bold)
	EmptyBoldFlashing(Flashing, Bold, EscSeqReset)
	
	TestOut(BackWhite, Red)
	TestOut2(Red)
}

fn main() -> Result<(), IoOrFmtErr> {
	{
		let mut vec = Vec::with_capacity(12);
		TwoReset::io_write(&mut vec).unwrap();
		
		assert_eq!(TwoReset::ESC_DATA_LEN, "\x1b[0;0m".len());
		assert_eq!(TwoReset::HEAD_DATA_LEN, "0;0".len());
		
		
		assert_eq!(vec, b"\x1b[0;0m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
	}
	
	{
		let mut vec = Vec::with_capacity(12);
		BoldFlashing::io_write(&mut vec)?;
		
		assert_eq!(BoldFlashing::ESC_DATA_LEN, "\x1b[5;1m".len());
		assert_eq!(BoldFlashing::HEAD_DATA_LEN, "5;1".len());
		
		
		assert_eq!(vec, b"\x1b[5;1m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
		
		
		let mut str = String::with_capacity(12);
		BoldFlashing::fmt_write(&mut str)?;
		
		assert_eq!(str, "\x1b[5;1m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
	}
	
	{
		let mut vec = Vec::with_capacity(12);
		EmptyBoldFlashing::io_write(&mut vec)?;
		
		assert_eq!(EmptyBoldFlashing::ESC_DATA_LEN, "\x1b[5;1;0m".len());
		assert_eq!(EmptyBoldFlashing::HEAD_DATA_LEN, "5;1;0".len());
		
		assert_eq!(vec, b"\x1b[5;1;0m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
		
		
		let mut str = String::with_capacity(12);
		EmptyBoldFlashing::fmt_write(&mut str)?;
		
		assert_eq!(str, "\x1b[5;1;0m");
		println!("{:?}", unsafe{ std::str::from_utf8_unchecked(&vec) });
	}
	
	
	//RUN
	{
		let stdout = std::io::stdout();
   		let mut handle = stdout.lock();
		
		{
			let mut safe_out = EscSeqWrite::new(&mut handle as &mut std::io::Write);
			safe_out.write::<TestOut>()?;
			
			write!(safe_out, "WhiteRed")?;
		}
		write!(handle, "\n")?;
		handle.flush()?;
	}
	{
		let stdout = std::io::stdout();
   		let mut handle = stdout.lock();
		
		{
			let mut safe_out = EscSeqWrite::new(&mut handle as &mut std::io::Write);
			safe_out.write::<TestOut2>()?;
			
			write!(safe_out, "Red")?;
		}
		write!(handle, "\n")?;
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