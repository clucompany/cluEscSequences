
use crate::ColorGeneric;
use std::fmt::Display;
use std::marker::PhantomData;
use std::fmt;
use std::fmt::Arguments;
use std::io::Write;
use std::io;

///Lightweight wrap for generalized color type.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorGenericWriter<C: ColorGeneric>(PhantomData<C>);

impl<C: ColorGeneric> ColorGenericWriter<C> {
	#[inline]
	pub fn new() -> Self {
		ColorGenericWriter(PhantomData)
	}
	
	#[inline(always)]
	pub fn raw_color<'a>(&self) -> &'a str {
		C::raw_color()
	}
	
	#[inline(always)]
	pub fn raw_color_b<'a>(&self) -> &'a [u8] {
		C::raw_color_b()
	}
	
	#[inline(always)]
	pub fn name<'a>(&self) -> &'a str {
		C::name()
	}
	
	#[inline(always)]
	pub fn write<'a, W: Write>(&self, w: W, buf: &'a [u8]) -> io::Result<()> {
		C::write(w, buf)
	}
	
	#[inline(always)]
	pub fn write_str<'a, W: Write>(&self, w: W, str: &'a str) -> io::Result<()> {
		C::write_str(w, str)
	}
	
	#[inline(always)]
	pub fn write_fmt<'a, W: Write>(&self, w: W, fmt: Arguments<'a>) -> io::Result<()> {
		C::write_fmt(w, fmt)
	}
	
	// add /n
	
	#[inline(always)]
	pub fn writen<'a, W: Write>(&self, w: W, buf: &'a [u8]) -> io::Result<()> {
		C::writen(w, buf)
	}
	
	#[inline(always)]
	pub fn writen_str<'a, W: Write>(&self, w: W, str: &'a str) -> io::Result<()> {
		C::writen_str(w, str)
	}
	
	#[inline(always)]
	pub fn writen_fmt<'a, W: Write>(&self, w: W, fmt: Arguments<'a>) -> io::Result<()> {
		C::writen_fmt(w, fmt)
	}
	
	
	#[inline(always)]
	pub fn string_as<'a, A: AsRef<str>>(&self, asref: A) -> String {
		C::string(asref.as_ref())
	}
	
	#[inline(always)]
	pub fn stringn_as<'a, A: AsRef<str>>(&self, asref: A) -> String {
		C::stringn(asref.as_ref())
	}
	
	#[inline(always)]
	pub fn string<'a>(&self, str: &'a str) -> String {
		C::string(str)
	}
	
	#[inline(always)]
	pub fn string_fmt<'a>(&self, fmt: Arguments<'a>) -> String {
		C::string_fmt(fmt)
	}
	
	
	#[inline(always)]
	pub fn stringn<'a>(&self, str: &'a str) -> String {
		C::stringn(str)
	}
	
	#[inline(always)]
	pub fn stringn_fmt<'a>(&self, fmt: Arguments<'a>) -> String {
		C::stringn_fmt(fmt)
	}
}



impl<C: ColorGeneric> Display for ColorGenericWriter<C> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}



impl<'l, T: ColorGeneric> ColorGeneric for ColorGenericWriter<T> {
	#[inline(always)]
	fn raw_color<'a>() -> &'a str { T::raw_color() }
	
	#[inline(always)]
	fn raw_color_b<'a>() -> &'a [u8] { T::raw_color_b() }
	
	#[inline(always)]
	fn name<'a>() -> &'a str { T::name() }
	

	#[inline(always)]
	fn writer() -> ColorGenericWriter<Self> where Self: Sized { ColorGenericWriter::<Self>::new() }
	
	
	#[inline(always)]
	fn string_as<'a, A: AsRef<str>>(asref: A) -> String { T::string_as(asref) }

	#[inline(always)]
	fn stringn_as<'a, A: AsRef<str>>(asref: A) -> String { T::stringn_as(asref) }
	
	#[inline(always)]
	fn string_fmt<'a>(fmt: Arguments<'a>) -> String { T::string_fmt(fmt) }

	#[inline(always)]
	fn string<'a>(str: &'a str) -> String { T::string(str) }

	#[inline(always)]
	fn stringn<'a>(str: &'a str) -> String { T::stringn(str) }

	#[inline(always)]
	fn stringn_fmt<'a>(fmt: Arguments<'a>) -> String { T::stringn_fmt(fmt) }
	
	#[inline(always)]
	fn write_as<'a, W: Write, A: AsRef<[u8]>>(w: W, asref: A) -> io::Result<()> { T::write_as(w, asref) }

	#[inline(always)]
	fn writen_as<'a, W: Write, A: AsRef<[u8]>>(w: W, asref: A) -> io::Result<()> { T::writen_as(w, asref) }

	#[inline(always)]
	fn write<'a, W: Write>(w: W, array: &'a [u8]) -> io::Result<()> { T::write(w, array) }

	#[inline(always)]
	fn write_str<'a, W: Write>(w: W, str: &'a str) -> io::Result<()> { T::write_str(w, str) }

	#[inline(always)]
	fn write_fmt<'a, W: Write>(w: W, fmt: Arguments<'a>) -> io::Result<()> { T::write_fmt(w, fmt) }

	#[inline(always)]
	fn writen<'a, W: Write>(w: W, array: &'a [u8]) -> io::Result<()> { T::writen(w, array) }
	
	#[inline(always)]
	fn writen_str<'a, W: Write>(w: W, str: &'a str) -> io::Result<()> { T::writen_str(w, str) }

	#[inline(always)]
	fn writen_fmt<'a, W: Write>(w: W, fmt: Arguments<'a>) -> io::Result<()> { T::writen_fmt(w, fmt) }

	#[inline(always)]
	fn with_color_fmt<'a, F: Fn(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::with_color_fmt(args, function) }

	#[inline(always)]
	fn once_with_color_fmt<'a, F: FnOnce(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::once_with_color_fmt(args, function) }
	
	#[inline(always)]
	fn mut_with_color_fmt<'a, F: FnMut(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::mut_with_color_fmt(args, function) }
}
