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

#![feature(const_fn)]
#![allow(non_snake_case)]

extern crate cluConstData;
pub extern crate cluExtIO;

use crate::data::EscSequencyData;
use cluExtIO::generic::WriteStr;
use std::io;
use std::fmt;

#[macro_use]
mod macros_data;
pub use self::macros_data::*;

#[macro_use]
mod macros_table;
pub use self::macros_table::*;

mod reset;
pub use self::reset::*;

pub mod heads;
pub mod colors;
pub mod back_colors;

pub mod data;

mod write;
pub use self::write::*;

mod display;
pub use self::display::*;


pub trait EscSequency {
	const ESC_DATA: &'static str;
	const HEAD_DATA: &'static str;
	//const STR_DATA: &'static str;
	
	const R_ESC_DATA: &'static [u8] = 
		unsafe {
			cluConstData::ignore_feature::const_str_as_bytes(
				<Self as EscSequency>::ESC_DATA
			)
		};
	const R_HEAD_DATA: &'static [u8] = unsafe {
			cluConstData::ignore_feature::const_str_as_bytes(
				<Self as EscSequency>::HEAD_DATA
			)
		};
	//const R_STR_DATA: &'static [u8] = <Self as EscSequency>::STR_DATA.as_bytes();
	
	#[inline(always)]
	fn data() -> EscSequencyData<Self> where Self: Sized {
		EscSequencyData::<Self>::new()
	}
	
	#[inline(always)]
	fn display() -> EscSeqDisplay<Self> where Self : Sized {
		EscSeqDisplay::<Self>::new()
	}
	
	#[inline(always)]
	fn writer<T: WriteStr<Ok=OK, Err=ERR>, OK, ERR>(write: T) -> EscSeqWrite<T, OK, ERR> where Self : Sized {
		EscSeqWrite::new(write)
	}
	
	
	#[inline(always)]
	fn head_io_write<W: io::Write>(mut w: W) -> Result<usize, io::Error> {
		w.write(Self::R_HEAD_DATA)
	}
		
	#[inline(always)]
	fn head_fmt_io_write<W: fmt::Write>(mut w: W) -> Result<(), fmt::Error> {
		w.write_str(Self::HEAD_DATA)
	}
		
	#[inline(always)]
	fn head_clustr_write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(mut w: W) -> Result<W::Ok, W::Err> {
		w.write_str(Self::HEAD_DATA)
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
	fn clustr_write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(mut w: W) -> Result<W::Ok, W::Err> {
		w.write_str(Self::ESC_DATA)
	}
}


/*
impl<A, T> EscSequency for (A, T) where A: EscSequency, T: EscSequency {
	const ESC_DATA: &'static str = T::ESC_DATA;
	const R_ESC_DATA: &'static [u8] = T::R_ESC_DATA;
	
	const DATA: &'static str = T::DATA;
	const RAW_DATA: &'static [u8] = 
		&cluConstData::const_concat!(u8: <A as EscSequency>::RAW_DATA, b"2");
}*/
/*
pub trait EscSequency: EscSeqLen
the trait bound `A: EscSequency` is not satisfied

the trait `EscSequency` is not implemented for `A`

help: consider adding a `where A: EscSequency` boundrustc(E0277)
lib.rs(169, 38): the trait `EscSequency` is not implemented for `A`

*/

/*impl<A, B> EscSequency for (A, B) where A: EscSequency, B: EscSequency {
	
		
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

pub trait EscSeqLen: EscSequency {	
	const HEAD_DATA_LEN: usize = 
		unsafe {
			cluConstData::ignore_feature::const_str_len(
				<Self as EscSequency>::HEAD_DATA
			)
		};
	const ESC_DATA_LEN: usize =
		unsafe {
			cluConstData::ignore_feature::const_str_len(
				<Self as EscSequency>::ESC_DATA
			)
		};
	
	#[inline]
	fn head_data_len() -> usize {
		Self::HEAD_DATA_LEN	
	}
	
	#[inline]
	fn esc_data_len() -> usize {
		Self::ESC_DATA_LEN	
	}
}

impl<T> EscSeqLen for T where T: EscSequency {}

/*
impl<'a, T> EscSeqLen for &'a T where T: EscSeqLen {	
	const LEN_ELEMENTS: usize = T::LEN_ELEMENTS;
	const ELEMENT_DATA_LEN: usize = T::ELEMENT_DATA_LEN;
	const ESC_DATA_LEN: usize = T::ESC_DATA_LEN;
	
	#[inline(always)]
	fn len_elements() -> usize {
		T::len_elements()
	}
	
	#[inline(always)]
	fn element_data_len() -> usize {
		T::element_data_len()
	}
	
	#[inline(always)]
	fn esc_data_len() -> usize {
		T::esc_data_len()
	}	
}*/

/*impl<A, T> EscSeqLen for (A, T) where A: EscSeqLen, T: EscSeqLen {
	const LEN_ELEMENTS: usize = <A as EscSeqLen>::LEN_ELEMENTS + <T as EscSeqLen>::LEN_ELEMENTS;
	const ELEMENT_LEN: usize = <A as EscSeqLen>::ELEMENT_LEN + <T as EscSeqLen>::ELEMENT_LEN;
	const ALL_LEN: usize = <A as EscSeqLen>::ALL_LEN + <T as EscSeqLen>::ALL_LEN;
	
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
}*/


#[cfg(test)]
mod tests {
	use crate::heads::Bold;
	use crate::heads::Flashing;
	use crate::EscSequency;
	use crate::EscSeqLen;
	use crate::reset::EscSeqReset;
	
	escseq_table! {
		SafeFlashlingBold(Flashing, Bold, EscSeqReset)
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
		
		
		assert_eq!(SafeFlashlingBold::ESC_DATA, "\x1b[5;1;0m");
		assert_eq!(SafeFlashlingBold::R_ESC_DATA, b"\x1b[5;1;0m");
		
		assert_eq!(SafeFlashlingBold::HEAD_DATA, "5;1;0");
		assert_eq!(SafeFlashlingBold::R_HEAD_DATA, b"5;1;0");
		
		assert_eq!(SafeFlashlingBold::HEAD_DATA_LEN, "5;1;0".len());
		assert_eq!(SafeFlashlingBold::ESC_DATA_LEN, "\x1b[5;1;0m".len());
	}	
}
