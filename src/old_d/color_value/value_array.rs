
use cluExtIO::WriteStr;
use crate::color_value::ColorValue;
use std::fmt::Arguments;
use std::fmt::Debug;
use std::io;
use std::fmt;
use cluExtIO::WriteFmt;
use crate::ArgLen;

pub trait ColorValueArray: Debug {
	fn len() -> usize;
	fn len_elements() -> usize;
	
	fn raw_io_write<W: io::Write>(w: W) -> Result<(), io::Error>;
	fn raw_fmt_write<W: fmt::Write>(w: W) -> Result<(), fmt::Error>;
	fn raw_clustr_write<'a, W: WriteStr<'a, O, E>, O, E>(w: W) -> Result<(), E>;
	
	fn io_write<W: io::Write>(w: W) -> Result<(), io::Error>;
	fn fmt_write<W: fmt::Write>(w: W) -> Result<(), fmt::Error>;
	fn clustr_write<'a, W: WriteStr<'a, O, E>, O, E>(w: W) -> Result<(), E>;
}

macro_rules! color_value_array {
	[ 
		($($head_traits:ident),*):	len[$len: expr, $len_elements:expr] = 
		
		$x1b:expr, $str: expr $(, $e_str:expr)*;
		$x1b_arr: expr,	$arr:expr;
		//fmt	|$fmt_w:ident|		$fmt_data:expr; 
		//io	|$io_w:ident|		$io_data:expr; 
		//args	||				$args:expr;
	
		$($tt:tt)* 
	] => {
		impl<$($head_traits),*> ArgLen for ($($head_traits),*) where $($head_traits: ColorValue),*{
			#[inline(always)]
			fn len() -> usize {
				$len + ($len_elements - 1)
			}
		}
		
		impl<$($head_traits),*> ColorValueArray for ($($head_traits),*) where $($head_traits: ColorValue),*{
			#[inline(always)]
			fn len() -> usize {
				$len + ($len_elements - 1)
			}
			
			#[inline(always)]
			fn len_elements() -> usize {
				$len_elements
			}
			
			#[inline(always)]
			fn raw_io_write<IoW: io::Write>(mut w: IoW) -> Result<(), io::Error> {
				write!(w, $str $(, $e_str)*)
			}
			
			#[inline(always)]
			fn raw_fmt_write<FmtW: fmt::Write>(mut w: FmtW) -> Result<(), fmt::Error> {
				write!(w, $str $(, $e_str)*)
			}
			
			#[inline(always)]
			fn raw_clustr_write<'a, W: WriteStr<'a, Ok, Err>, Ok, Err>(mut w: W) -> Result<(), Err> {
				//write!(w, $str $(, $e_str)*)
				w.write_str_array($x1b_arr)
			}
			
			
			#[inline(always)]
			fn io_write<W: io::Write>(mut w: W) -> Result<(), io::Error> {
				write!(w, $x1b $(, $e_str)*)
			}
			
			#[inline(always)]
			fn fmt_write<W: fmt::Write>(mut w: W) -> Result<(), fmt::Error> {
				write!(w, $x1b $(, $e_str)*)
			}
			
			#[inline(always)]
			fn clustr_write<'a, W: WriteStr<'a, Ok, Err>, Ok, Err>(mut w: W) -> Result<(), Err> {
				//write!(w, $x1b $(, $e_str)*)
				w.write_str_array($x1b_arr)
			}
		}
		
		color_value_array! {
			$($tt)*
		}
	};
	
	() => ()
}


color_value_array! {
	(A):					len[A::len(), 1] = 
	"\x1b[{}m", 			"{}", A::data();
	
	&["\x1b[", A::data(), "m"],	
	&[A::data(), "m"];
	
	(A, B):				len[A::len() + B::len(), 2] = 
	"\x1b[{};{}m", 			"{};{}", A::data(), B::data();
	
	&["\x1b[", A::data(), ";", B::data(), "m"], 
	&[A::data(), ";", B::data(), "m"];
	
	(A, B, C):				len[A::len() + B::len() + C::len(), 3] = 
	"\x1b[{};{};{}m",			"{};{};{}", A::data(), B::data(), C::data();
	
	&["\x1b[", A::data(), ";", B::data(), ";", C::data(), "m"], 
	&[A::data(), ";", B::data(), ";", C::data(), "m"];
	
	
	(A, B, C, D):			len[A::len() + B::len() + C::len() + D::len(), 4] = 
	"\x1b[{};{};{};{}m",		"{};{};{};{}", A::data(), B::data(), C::data(), D::data();
	
	&["\x1b[", A::data(), ";", B::data(), ";", C::data(), ";", D::data(), "m"], 
	&[A::data(), ";", B::data(), ";", C::data(), ";", D::data(), "m"];
	
	
	(A, B, C, D, E):			len[A::len() + B::len() + C::len() + D::len() + E::len(), 5] = 
	"\x1b[{};{};{};{};{}m",		"{};{};{};{};{}", A::data(), B::data(), C::data(), D::data(), E::data();
	
	&["\x1b[", A::data(), ";", B::data(), ";", C::data(), ";", D::data(), ";", E::data(), "m"], 
	&[A::data(), ";", B::data(), ";", C::data(), ";", D::data(), ";", E::data(), "m"];
	
	
	(A, B, C, D, E, F):		len[A::len() + B::len() + C::len() + D::len() + E::len() + F::len(), 6] = 
	"\x1b[{};{};{};{};{};{}m",	"{};{};{};{};{};{}", A::data(), B::data(), C::data(), D::data(), E::data(), F::data();
	
	&["\x1b[", A::data(), ";", B::data(), ";", C::data(), ";", D::data(), ";", E::data(), ";", F::data(), "m"], 
	&[A::data(), ";", B::data(), ";", C::data(), ";", D::data(), ";", E::data(), ";", F::data(), "m"];
}

#[derive(Debug)]
pub struct ValueArray<T> where T: ColorValueArray {
	array: T
}

impl<T> ValueArray<T> where T: ColorValueArray {
	#[inline]
	pub const fn new(a: T) -> Self {
		Self {
			array: a	
		}	
	}
	
	#[inline(always)]
	pub fn len(&self) -> usize {
		T::len()	
	}
	
	#[inline(always)]
	pub fn len_elements(&self) -> usize {
		T::len_elements()	
	}
	
	#[inline(always)]
	pub fn raw_io_write<W: io::Write>(&self, w: W) -> Result<(), io::Error> {
		T::raw_io_write(w)	
	}
	
	#[inline(always)]
	pub fn raw_fmt_write<W: fmt::Write>(&self, w: W) -> Result<(), fmt::Error> {
		T::raw_fmt_write(w)	
	}
}

