
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;

pub trait ColorElement {
	fn color_name(&self) -> &'static str;
	fn raw_data(&self) -> &'static [u8];
	fn str_data(&self) -> &'static str;
	
	#[inline(always)]
	fn as_dyn<'a>(&self) -> &(dyn ColorElement + 'a) where Self: Sized + 'a {
		self
	}
}

impl<'a> AsRef<(dyn ColorElement + 'a)> for dyn ColorElement + 'a {
	#[inline(always)]
	fn as_ref(&self) -> &(dyn ColorElement + 'a) {
		self
	}
}

impl<'a> Display for dyn ColorElement + 'a {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
		write!(f, "{}", self.color_name())
	}
}

impl<'a> Debug for dyn ColorElement + 'a {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
		write!(f, "ColorElement{{ {} }}", self.color_name())
	}
}

impl<'a> AsRef<[u8]> for dyn ColorElement + 'a {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.raw_data()
	}
}

impl<'a> AsRef<str> for dyn ColorElement + 'a {
	#[inline(always)]
	fn as_ref(&self) -> &str {
		self.str_data()
	}
}
