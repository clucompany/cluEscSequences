
use cluExtIO::WriteStr;
use std::io;

pub trait WriteColoreds {
	fn len() -> usize;
	fn len_elements() -> usize;
	
	fn raw_io_write<W: io::Write>(w: W) -> Result<(), io::Error>;
	
	
	fn clustr_write<'a, W: WriteStr<'a, O, E>, O, E>(w: W) -> Result<(), E>;
}

