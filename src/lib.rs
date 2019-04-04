#![feature(const_str_len)]
#![feature(const_slice_len)]
#![feature(test)]
#![feature(plugin)]
#![feature(associated_type_defaults)]
#![feature(type_ascription)]
#![feature(const_str_as_bytes)]
#![feature(const_fn)]

//Copyright 2019 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	  http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1819
//

/*!

# cluEscSequency

*/

/*mod elements;
pub use self::elements::*;*/


#![plugin(cluConcatBytes)]

extern crate cluConcatBytes;
extern crate cluExtIO;

#[macro_use]
extern crate cluConstConcat;

use cluConstConcat::ConstConcat;

/*mod color_args;
pub use self::color_args::*;

pub mod color_value;
//pub use self::color_value::*;

pub mod write;

mod len;
pub use self::len::*;

mod reset;
pub use self::reset::*;

mod write_array;
pub use self::write_array::*;
*/


use cluExtIO::generic::WriteStr;
use std::io;
use std::fmt;

#[macro_use]
mod macros;
pub use self::macros::*;

#[macro_use]
mod macros_table;
pub use self::macros_table::*;

mod reset;
pub use self::reset::*;

pub mod heads;
pub mod colors;
#[macro_use]
pub mod maybe_data;

mod write;
pub use self::write::*;


pub trait FontSeqEscape: FontSeqLen {
	const ESC_DATA: &'static str;
	const R_ESC_DATA: &'static [u8];
	
	const ELEMENT_DATA: &'static str;
	const R_ELEMENT_DATA: &'static [u8];
	
	#[inline(always)]
	fn element_io_write<W: io::Write>(mut w: W) -> Result<usize, io::Error> {
		w.write(Self::R_ELEMENT_DATA)
	}
		
	#[inline(always)]
	fn element_fmt_io_write<W: fmt::Write>(mut w: W) -> Result<(), fmt::Error> {
		w.write_str(Self::ELEMENT_DATA)
	}
		
	#[inline(always)]
	fn element_clustr_write<W: WriteStr<Ok=T_OK, Err=T_ERR>, T_OK, T_ERR>(mut w: W) -> Result<W::Ok, W::Err> {
		w.write_str(Self::ELEMENT_DATA)
	}
	
	#[inline(always)]
	fn io_write<W: io::Write>(mut w: W) -> Result<usize, io::Error> {
		w.write(Self::R_ESC_DATA)
	}
	
	#[inline(always)]
	fn fmt_write<W: fmt::Write>(mut w: W) -> Result<(), fmt::Error> {
		w.write_str(Self::ESC_DATA)
	}
	
	#[inline(always)]
	fn clustr_write<W: WriteStr<Ok=T_OK, Err=T_ERR>, T_OK, T_ERR>(mut w: W) -> Result<W::Ok, W::Err> {
		w.write_str(Self::ESC_DATA)
	}
}

impl<'l, T> FontSeqEscape for &'l T where T: FontSeqEscape {
	const ESC_DATA: &'static str = T::ESC_DATA;
	const R_ESC_DATA: &'static [u8] = T::R_ESC_DATA;
	
	const ELEMENT_DATA: &'static str = T::ELEMENT_DATA;
	const R_ELEMENT_DATA: &'static [u8] = T::R_ELEMENT_DATA;
	
	
	#[inline(always)]
	fn element_io_write<W: io::Write>(w: W) -> Result<usize, io::Error> {
		T::element_io_write(w)
	}
		
	#[inline(always)]
	fn element_fmt_io_write<W: fmt::Write>(w: W) -> Result<(), fmt::Error> {
		T::element_fmt_io_write(w)
	}
		
	#[inline(always)]
	fn element_clustr_write<W: WriteStr<Ok=T_OK, Err=T_ERR>, T_OK, T_ERR>(mut w: W) -> Result<W::Ok, W::Err> {
		T::element_clustr_write(w)
	}
	
	#[inline(always)]
	fn io_write<W: io::Write>(w: W) -> Result<usize, io::Error> {
		T::io_write(w)
	}
	
	#[inline(always)]
	fn fmt_write<W: fmt::Write>(w: W) -> Result<(), fmt::Error> {
		T::fmt_write(w)
	}
	
	#[inline(always)]
	fn clustr_write<W: WriteStr<Ok=T_OK, Err=T_ERR>, T_OK, T_ERR>(mut w: W) -> Result<W::Ok, W::Err> {
		T::clustr_write(w)
	}
}

/*
impl<A, T> FontSeqEscape for (A, T) where A: FontSeqEscape, T: FontSeqEscape {
	const ESC_DATA: &'static str = T::ESC_DATA;
	const R_ESC_DATA: &'static [u8] = T::R_ESC_DATA;
	
	const DATA: &'static str = T::DATA;
	const RAW_DATA: &'static [u8] = 
		&cluConstConcat::const_concat!(u8: <A as FontSeqEscape>::RAW_DATA, b"2");
}*/
/*
pub trait FontSeqEscape: FontSeqLen
the trait bound `A: FontSeqEscape` is not satisfied

the trait `FontSeqEscape` is not implemented for `A`

help: consider adding a `where A: FontSeqEscape` boundrustc(E0277)
lib.rs(169, 38): the trait `FontSeqEscape` is not implemented for `A`

*/

/*impl<A, B> FontSeqEscape for (A, B) where A: FontSeqEscape, B: FontSeqEscape {
	
		
	const_data! {
		const ESC_DATA: &'static str = A::ESC_DATA;
		const R_ESC_DATA: &'static [u8] = A::R_ESC_DATA;
		
		const DATA: &'static str = A::DATA;
		//const RAW_DATA: &'static [u8] = A::RAW_DATA, b"12";
	}
	const RAW_DATA: &'static [u8] = &unsafe {
		ConstConcat::<A::RAW_U8Array_TYPE, B::RAW_U8Array_TYPE>
			::const_concat::<[u8; 1 + 1], u8>
			
		(A::RAW_DATA, B::RAW_DATA) 
	};
}

Раст на данный момент не предоставляет возможности провернуть 

ConstConcat::<A::RAW_U8Array_TYPE, B::RAW_U8Array_TYPE>
	::const_concat::<[u8; 1 + 1], u8>
!!!!!

Но раст работает с точными, не с типажами!
Тоесть если конкатить известные типы то это сработает.

*/

pub trait FontSeqLen {	
	const LEN_ELEMENTS: usize = 1;
	const ELEMENT_LEN: usize = 1;
	const ALL_LEN: usize = "\x1b[".len() + Self::LEN_ELEMENTS + "m".len();
	
	#[inline]
	fn len_elements() -> usize {
		Self::LEN_ELEMENTS
	}
	
	#[inline]
	fn element_len() -> usize {
		Self::ELEMENT_LEN	
	}
	
	#[inline]
	fn all_len() -> usize {
		Self::ALL_LEN	
	}
}

impl<'a, T> FontSeqLen for &'a T where T: FontSeqLen {	
	const LEN_ELEMENTS: usize = T::LEN_ELEMENTS;
	const ELEMENT_LEN: usize = T::ELEMENT_LEN;
	const ALL_LEN: usize = T::ALL_LEN;
	
	#[inline(always)]
	fn len_elements() -> usize {
		T::len_elements()
	}
	
	#[inline(always)]
	fn element_len() -> usize {
		T::element_len()	
	}
	
	#[inline(always)]
	fn all_len() -> usize {
		T::all_len()	
	}	
}

impl<A, T> FontSeqLen for (A, T) where A: FontSeqLen, T: FontSeqLen {
	const LEN_ELEMENTS: usize = <A as FontSeqLen>::LEN_ELEMENTS + <T as FontSeqLen>::LEN_ELEMENTS;
	const ELEMENT_LEN: usize = <A as FontSeqLen>::ELEMENT_LEN + <T as FontSeqLen>::ELEMENT_LEN;
	const ALL_LEN: usize = <A as FontSeqLen>::ALL_LEN + <T as FontSeqLen>::ALL_LEN;
	
	#[inline]
	fn len_elements() -> usize {
		Self::LEN_ELEMENTS
	}
	
	#[inline]
	fn element_len() -> usize {
		Self::ELEMENT_LEN	
	}
	
	#[inline]
	fn all_len() -> usize {
		Self::ALL_LEN	
	}
}


#[cfg(test)]
mod tests {
	use crate::heads::Bold;
	use crate::heads::Flashing;
	use crate::FontSeqEscape;
	use crate::FontSeqLen;
	use crate::reset::FontSeqReset;
	
	fontseg_table! {
		SafeFlashlingBold(Flashing, Bold, FontSeqReset)
	}
	
	#[test]
	fn test_map() {		
		{
			let mut vec = Vec::with_capacity(12);
			SafeFlashlingBold::io_write(&mut vec).unwrap();
			
			assert_eq!(vec, b"\x1b[5;1;0m");
		}
		
		{
			let mut vec = String::with_capacity(12);
			SafeFlashlingBold::fmt_write(&mut vec).unwrap();
			
			assert_eq!(vec, "\x1b[5;1;0m");
		}
		
		{
			let mut str = String::with_capacity(12);
			SafeFlashlingBold::clustr_write(&mut str).unwrap();
			
			assert_eq!(str, "\x1b[5;1;0m");
		}
		
		
		assert_eq!(FontSeqReset::ELEMENT_DATA, "5;1;0");
		assert_eq!(FontSeqReset::R_ELEMENT_DATA, b"5;1;0");
		
		assert_eq!(FontSeqReset::LEN_ELEMENTS, 3);
		assert_eq!(FontSeqReset::ELEMENT_LEN, "5;1;0".len());
		assert_eq!(FontSeqReset::ALL_LEN, "\x1b[5;1;0m".len());
	}	
}
