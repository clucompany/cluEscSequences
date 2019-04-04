

use std::fmt::Display;
use std::fmt::Arguments;
use std::fmt::Debug;
use std::io;
use std::fmt;

pub trait ColorHead: Debug {
	fn len() -> usize;
	
	fn data() -> &'static str;
	fn raw_data() -> &'static [u8];
	
	fn name_arg() -> &'static str;
	
	fn args<'a>() -> Arguments<'a>;
	
	fn io_write<W: io::Write>(w: W) -> Result<(), io::Error>;
	fn fmt_write<W: fmt::Write>(w: W) -> Result<(), fmt::Error>;
}

macro_rules! color_args {
	[
		$name: ident[$len:expr] $data:expr , $r_data:expr;
		io|$io_w:ident| $io:expr;
		fmt|$fmt_w:ident| $fmt:expr;
		
		$($tt:tt)*
	] => {
		#[derive(Debug)]
		pub enum $name {}
		
		impl ColorHead for $name {
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
			fn name_arg() -> &'static str {
				stringify!($name)
			}
			
			#[inline(always)]
			fn args<'a>() -> Arguments<'a> {
				format_args!($data)
			}
			
			#[inline(always)]
			fn io_write<W: io::Write>(w: W) -> Result<(), io::Error> {
				let mut $io_w = w;
				$io
			}
			
			#[inline(always)]
			fn fmt_write<W: fmt::Write>(w: W) -> Result<(), fmt::Error> {
				let mut $fmt_w = w;
				$fmt
			}
		}
		
		color_args! {
			$($tt)*
		}
	};
	
	() => ()
}

color_args!{
	Empty[1] "0", b"0";
	io	|w| write!(w, "0");
	fmt	|w| write!(w, "0");
	
	Bold[1] "1", b"1";
	io	|w| write!(w, "1");
	fmt	|w| write!(w, "1");
	
	//Жирный
	Underline[1] "4", b"4";
	io	|w| write!(w, "4");
	fmt	|w| write!(w, "4");
	
	//Подчёркнутый
	Flashing[1] "5", b"5";
	io	|w| write!(w, "5");
	fmt	|w| write!(w, "5");
	
	InvertedColors[1] "7", b"7";
	io	|w| write!(w, "7");
	fmt	|w| write!(w, "7");
	
	Invisible[1] "8", b"8";
	io	|w| write!(w, "8");
	fmt	|w| write!(w, "8");
}

#[derive(Debug)]
pub struct Head<T> where T: ColorHead {
	head: T	
}

impl<T> Head<T> where T: ColorHead {
	#[inline(always)]
	pub const fn new(a: T) -> Self {
		Self {
			head: a	
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
	pub fn name_arg(&self) -> &'static str {
		T::name_arg()
	}
	
	#[inline(always)]
	pub fn args(&self) -> Arguments<'static> {
		T::args()
	}
	
	#[inline(always)]
	pub fn io_write<W: io::Write>(&self, w: W) -> Result<(), io::Error> {
		T::io_write(w)
	}
	
	#[inline(always)]
	pub fn fmt_write<W: fmt::Write>(&self, w: W) -> Result<(), fmt::Error> {
		T::fmt_write(w)
	}
}

impl<T> Display for Head<T> where T: ColorHead {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.args())
	}
}