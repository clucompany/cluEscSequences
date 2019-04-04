
use crate::color_value::ColorValueArray;
use crate::head_args::ColorHeadArray;
use std::io;
use std::fmt;

#[derive(Debug)]
pub struct ColorWrite<H, V> where H: ColorHeadArray, V: ColorValueArray {
	head: H,
	value: V,
}

impl<H, V> ColorWrite<H, V> where H: ColorHeadArray, V: ColorValueArray {
	#[inline(always)]
	pub const fn new(head: H, value: V) -> Self {
		Self {
			head: head,
			value: value,	
		}
	}
	
	#[inline(always)]
	pub const fn as_head(&self) -> &H {
		&self.head
	}
	
	#[inline(always)]
	pub const fn as_value(&self) -> &V {
		&self.value
	}
	
	pub fn get_len() -> usize {
		"\x1b[".len() + H::len() + V::len() + 1
	}
	
	#[inline(always)]
	pub fn len(&self) -> usize {
		Self::get_len()
	}
	
	pub fn io_write<W: io::Write>(&self, mut w: W) -> Result<(), io::Error> {
		write!(w, "\x1b[")?;
		H::raw_io_write(&mut w)?;
		write!(w, ";")?;
		V::raw_io_write(&mut w)?;
		write!(w, "m")
	}
	
	pub fn fmt_write<W: fmt::Write>(&self, mut w: W) -> Result<(), fmt::Error> {
		write!(w, "\x1b[")?;
		H::raw_fmt_write(&mut w)?;
		write!(w, ";")?;
		V::raw_fmt_write(&mut w)?;
		write!(w, "m")
	}
	
	pub fn static_io_write<W: io::Write>(mut w: W) -> Result<(), io::Error> {
		write!(w, "\x1b[")?;
		H::raw_io_write(&mut w)?;
		write!(w, ";")?;
		V::raw_io_write(&mut w)?;
		write!(w, "m")
	}
	
	pub fn static_fmt_write<W: fmt::Write>(mut w: W) -> Result<(), fmt::Error> {
		write!(w, "\x1b[")?;
		H::raw_fmt_write(&mut w)?;
		write!(w, ";")?;
		V::raw_fmt_write(&mut w)?;
		write!(w, "m")
	}
}

impl<H, V> From<(H, V)> for ColorWrite<H, V> where H: ColorHeadArray, V: ColorValueArray {
	#[inline(always)]
	fn from((a, v) : (H, V)) -> Self {
		Self::new(a, v)	
	}
}
