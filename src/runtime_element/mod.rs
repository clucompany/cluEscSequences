
use core::fmt::Display;
use cluExtIO::generic::WriteStr;
use crate::data::EscSeqData;
use core::fmt::Debug;


#[derive(Debug, Clone)]
pub struct EscRunElementSeq {
	esc_data: &'static str,
	head_data: &'static str,
}

impl EscRunElementSeq {
	#[inline]
	pub const fn new<T>() -> Self where T: EscSeqData {
		Self {
			esc_data: T::ESC_DATA,
			head_data: T::HEAD_DATA,
		}
	}
	
	#[inline]
	pub const unsafe fn data(esc_data: &'static str, head_data: &'static str) -> Self {
		Self {
			esc_data: esc_data,
			head_data: head_data,
		}
	}
	
	#[inline]
	pub fn head_write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(&self, mut w: W) -> Result<W::Ok, W::Err> {
		w.write_str(self.head_data)
	}
	
	#[inline]
	pub fn write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(&self, mut w: W) -> Result<W::Ok, W::Err> {
		w.write_str(self.esc_data)
	}
}

impl Display for EscRunElementSeq {
	#[inline]
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		fmt.write_str(self.esc_data)
	}
}