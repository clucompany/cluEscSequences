
use std::fmt::Display;
use std::fmt::Formatter;
use crate::write::EscSeqWrite;
use cluExtIO::generic::WriteStr;
use crate::display::EscSeqDisplay;
use std::marker::PhantomData;
use crate::EscSequency;
use crate::reset::EscSeqReset;
use std::io;
use std::io::Write;
use std::fmt;

#[derive(Debug)]
pub struct EscSequencyData<T> where T: EscSequency {
	_p: PhantomData<T>,
}

impl<T> Clone for EscSequencyData<T> where T: EscSequency {
	#[inline(always)]
	fn clone(&self) -> Self {
		Self::new()
	}
}

impl<T> EscSequencyData<T> where T: EscSequency {
	#[inline]
	pub const fn new() -> Self {
		Self {
			_p: PhantomData,
		}
	}
	
	#[inline(always)]
	pub const fn display(&self) -> EscSeqDisplay<T> {
		EscSeqDisplay::<T>::new()
	}
	
	#[inline(always)]
	pub const fn writer<W: WriteStr<Ok=OK, Err=ERR>, OK, ERR>(&self, write: W) -> EscSeqWrite<W, OK, ERR> {
		EscSeqWrite::new(write)
	}
	
	
	#[inline(always)]
	pub fn head_io_write<W: io::Write>(&self, w: W) -> Result<usize, io::Error> {
		T::head_io_write(w)
	}
		
	#[inline(always)]
	pub fn head_fmt_io_write<W: fmt::Write>(&self, w: W) -> Result<(), fmt::Error> {
		T::head_fmt_io_write(w)
	}
		
	#[inline(always)]
	pub fn head_clustr_write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(&self, w: W) -> Result<W::Ok, W::Err> {
		T::head_clustr_write(w)
	}
	
	#[inline(always)]
	pub fn io_write<W: io::Write>(&self, w: W) -> Result<usize, io::Error> {
		T::io_write(w)
	}
	
	#[inline(always)]
	pub fn fmt_write<W: fmt::Write>(&self, w: W) -> Result<(), fmt::Error> {
		T::fmt_write(w)
	}
	
	#[inline(always)]
	pub fn clustr_write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(&self, w: W) -> Result<W::Ok, W::Err> {
		T::clustr_write(w)
	}
	
	#[inline(always)]
	pub fn reset<W: WriteStr<Ok=OK, Err=ERR>, OK, ERR>(&self, w: W) -> Result<OK, ERR> {
		EscSeqReset::clustr_write(w)
	}
	
	
	pub fn reset_println(&self) -> Result<usize, io::Error> {
		std::io::stdout().write(EscSeqReset::R_ESC_DATA)
	}
}

impl<T> From<()> for EscSequencyData<T> where T: EscSequency {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Self::new()
	}
}

impl<T> Display for EscSequencyData<T> where T: EscSequency {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		f.write_str(T::ESC_DATA)
	}
}

