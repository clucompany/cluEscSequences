
use crate::static_element::EscSeqStatic;
use std::fmt::Display;
use std::fmt::Formatter;
use cluExtIO::generic::WriteStr;
use std::marker::PhantomData;
use std::fmt;


#[derive(Debug)]
pub struct EscElementSeq<T> where T: EscSeqStatic {
	_p: PhantomData<T>,
}

impl<T> Clone for EscElementSeq<T> where T: EscSeqStatic {
	#[inline(always)]
	fn clone(&self) -> Self {
		Self::new()
	}
}

impl<T> EscElementSeq<T> where T: EscSeqStatic {
	#[inline]
	pub const fn new() -> Self {
		Self {
			_p: PhantomData,
		}
	}

	#[inline(always)]
	pub fn head_write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(&self, w: W) -> Result<W::Ok, W::Err> {
		T::head_write(w)
	}
	
	#[inline(always)]
	pub fn write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(&self, w: W) -> Result<W::Ok, W::Err> {
		T::write(w)
	}
}

impl<T> Display for EscElementSeq<T> where T: EscSeqStatic {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		f.write_str(T::ESC_DATA)
	}
}

