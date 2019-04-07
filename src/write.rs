
use std::ops::DerefMut;
use std::ops::Deref;
use crate::EscSequency;
use cluExtIO::generic::WriteStr;
use cluExtIO::phantom::PhantomWriteStr;
use crate::reset::EscSeqReset;
use std::fmt;

#[derive(Debug)]
pub struct EscSeqWrite<T, OK, ERR>(T, PhantomWriteStr<OK, ERR>) where T: WriteStr<Ok=OK, Err=ERR>;

impl<T, OK, ERR> EscSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	#[inline]
	pub const fn new(a: T) -> Self {
		EscSeqWrite(a, PhantomWriteStr::new())
	}
	
	#[inline(always)]
	pub fn reset(&mut self) -> Result<OK, ERR> {
		EscSeqReset::clustr_write(&mut self.0)
	}
	
	#[inline(always)]
	pub fn write<Esc: EscSequency>(&mut self) -> Result<OK, ERR> {
		Esc::clustr_write(&mut self.0)
	}
	
	
}

impl<T, OK, ERR> Deref for EscSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0	
	}
}

impl<T, OK, ERR> DerefMut for EscSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {	
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}



impl<T, OK, ERR> From<T> for EscSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)	
	}
}

impl<T, OK, ERR> WriteStr for EscSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
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

impl<T> fmt::Write for EscSeqWrite<T, (), fmt::Error> where T: WriteStr<Ok=(), Err=fmt::Error> {
	#[inline(always)]
	fn write_str(&mut self, str: &str) -> Result<(), fmt::Error> {
		self.0.write_str(str)
	}
}


impl<T, OK, ERR> Drop for EscSeqWrite<T, OK, ERR> where T: WriteStr<Ok=OK, Err=ERR> {
	fn drop(&mut self) {
		let _e = self.reset();
	}
}
