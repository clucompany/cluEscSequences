
use cluExtIO::WriteFmt;

use std::fmt::Display;
use std::fmt::Arguments;
use std::fmt::Debug;
use std::io;
use std::fmt;
use crate::len::ArgLen;

pub mod font_color;
pub mod font_back;

mod value_array;
pub use self::value_array::*;


pub trait ColorValue: Debug {
	fn len() -> usize;
	
	fn data() -> &'static str;
	fn raw_data() -> &'static [u8];
	
	fn args<'a>() -> Arguments<'a>;
	
	fn name_value() -> &'static str;
	
	fn io_write<W: io::Write>(w: W) -> Result<(), io::Error>;
	fn fmt_write<W: fmt::Write>(w: W) -> Result<(), fmt::Error>;
	fn clufmt_write<W: WriteFmt<E>, E>(w: W) -> Result<(), E>;
}

#[macro_export]
macro_rules! color_value {
	[
		$name: ident[$len:expr] $data:expr , $r_data:expr;
		//io|$io_w:ident| $io:expr;
		//fmt|$fmt_w:ident| $fmt:expr;
		fmt	||	$fmt:expr $(, $e_fmt:expr)*;
		
		$($tt:tt)*
	] => {
		#[derive(Debug)]
		#[allow(non_camel_case_types)]
		pub enum $name {}
		
		impl ArgLen for $name {
			#[inline(always)]
			fn len() -> usize {
				$len
			}	
		}
		
		impl ColorValue for $name {
			#[inline(always)]
			fn len() -> usize {
				$len
			}
			#[inline(always)]
			fn data() -> &'static str {
				$data
			}
			#[inline(always)]
			fn raw_data() -> &'static [u8] {
				$r_data
			}
			#[inline(always)]
			fn name_value() -> &'static str {
				stringify!($fmt $(, $e_fmt)*)
			}
			
			#[inline(always)]
			fn args<'a>() -> Arguments<'a> {
				format_args!($fmt $(, $e_fmt)*)
			}
			
			#[inline(always)]
			fn io_write<W: io::Write>(mut w: W) -> Result<(), io::Error> {
				write!(w, $fmt $(, $e_fmt)*)
			}
			
			#[inline(always)]
			fn fmt_write<W: fmt::Write>(mut w: W) -> Result<(), fmt::Error> {
				write!(w, $fmt $(, $e_fmt)*)
			}
			
			#[inline(always)]
			fn clufmt_write<W: WriteFmt<E>, E>(mut w: W) -> Result<(), E> {
				write!(w, $fmt $(, $e_fmt)*)
			}
		}
		
		color_value! {
			$($tt)*
		}
	};
	
	() => ()
}

#[derive(Debug)]
pub struct Value<T> where T: ColorValue {
	value: T	
}

impl<T> Value<T> where T: ColorValue {
	#[inline(always)]
	pub const fn new(a: T) -> Self {
		Self {
			value: a	
		}	
	}
	
	#[inline(always)]
	pub fn len(&self) -> usize {
		T::len()	
	}
	
	#[inline(always)]
	pub fn data(&self) -> &'static str {
		T::data()	
	}
	
	#[inline(always)]
	pub fn raw_data(&self) -> &'static [u8] {
		T::raw_data()	
	}
	
	#[inline(always)]
	pub fn args(&self) -> Arguments<'static> {
		T::args()	
	}
	
	#[inline(always)]
	pub fn name_value(&self) -> &'static str {
		T::name_value()	
	}
	
	#[inline(always)]
	pub fn io_write<W: io::Write>(&self, w: W) -> Result<(), io::Error> {
		T::io_write(w)	
	}
	
	#[inline(always)]
	pub fn fmt_write<W: fmt::Write>(&self, w: W) -> Result<(), fmt::Error> {
		T::fmt_write(w)	
	}
	
	#[inline(always)]
	pub fn clufmt_write<W: WriteFmt<E>, E>(w: W) -> Result<(), E> {
		T::clufmt_write(w)	
	}
}

impl<T> Display for Value<T> where T: ColorValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.args())
	}
}
