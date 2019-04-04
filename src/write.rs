
use std::ops::DerefMut;
use std::ops::Deref;
use crate::FontSeqEscape;
use cluExtIO::generic::WriteStr;
use cluExtIO::phantom::PhantomWriteStr;
use crate::reset::FontSeqReset;


#[derive(Debug)]
pub struct FontSeqWrite<T, OK, ERR>(T, PhantomWriteStr<OK, ERR>) where T: WriteStr<Ok=OK, Err=ERR>;

impl<T, OK, ERR> FontSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	#[inline]
	pub const fn new(a: T) -> Self {
		FontSeqWrite(a, PhantomWriteStr::new())
	}
	
	#[inline(always)]
	pub fn reset(&mut self) -> Result<OK, ERR> {
		FontSeqReset::clustr_write(&mut self.0)
	}
	
	#[inline(always)]
	pub fn write<Esc: FontSeqEscape>(&mut self) -> Result<OK, ERR> {
		Esc::clustr_write(&mut self.0)
	}
}

impl<T, OK, ERR> Deref for FontSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0	
	}
}

impl<T, OK, ERR> DerefMut for FontSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {	
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}



impl<T, OK, ERR> From<T> for FontSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)	
	}
}

impl<T, OK, ERR> WriteStr for FontSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	type Ok = OK;
	type Err = ERR;
	
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> Result<Self::Ok, Self::Err> {
		self.0.write_str(s)
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.0.write_str_array(arr)
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, all_size: usize, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.0.write_str_lenarray(all_size, arr)
	}
}

impl<T, OK, ERR> Drop for FontSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	fn drop(&mut self) {
		let _e = FontSeqReset::clustr_write(&mut self.0);
	}
}