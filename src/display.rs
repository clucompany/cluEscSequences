
use std::fmt::Formatter;
use std::fmt::Display;
use crate::EscSequency;
use std::marker::PhantomData;
use std::fmt;

#[derive(Debug)]
pub struct EscSeqDisplay<E>(PhantomData<E>) where E: EscSequency;

impl<E> EscSeqDisplay<E> where E: EscSequency {
	#[inline(always)]
	pub const fn new() -> Self {
		EscSeqDisplay(PhantomData)	
	}	
}

impl<E> Display for EscSeqDisplay<E> where E: EscSequency {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		f.write_str(E::ESC_DATA)
	}
}

