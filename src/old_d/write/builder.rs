
use cluExtIO::WriteFmt;
use std::io::StdoutLock;
use std::fmt::Arguments;
use crate::write::write::ColorWrite;
use std::marker::PhantomData;
use crate::color_value::ColorValueArray;
use crate::head_args::ColorHeadArray;
use std::ops::Deref;
use std::ops::DerefMut;
use std::fmt;
use cluExtIO::WriteStr;

#[derive(Debug)]
pub struct ColorBuilder<'a, W, Ok, Err> where W: WriteStr<'a, Ok, Err> {
	write: W,
	
	_e: PhantomData<&'a ()>,
	_ee: PhantomData<Ok>,
	_eee: PhantomData<Err>,
}

impl<'a, W, Ok, Err> ColorBuilder<'a, W, Ok, Err> where W: WriteStr<'a, Ok, Err> {
	#[inline]
	pub const fn new(w: W) -> Self {
		Self {
			write: w,
			
			_e: PhantomData,
			_ee: PhantomData,
			_eee: PhantomData,
		}
	}
	
	pub fn once<F: FnOnce(&mut W) -> RF, RF>(mut w: W, f: F) -> Result<(RF, W), Err> {
		let result = f(&mut w);
		//write!(&mut w, "\x1b[0m")?;
		w.write_str("\x1b[0m")?;
		Ok( (result, w) )
	}
	
	#[inline(always)]
	pub fn write_head<H: ColorHeadArray>(&mut self) -> Result<(), Err> {
		H::clustr_write(&mut self.write as &mut WriteStr<'a, Ok, Err>)
	}
	
	#[inline(always)]
	pub fn write_color<V: ColorValueArray>(&mut self) -> Result<(), Err> {
		V::clustr_write(&mut self.write as &mut WriteStr<'a, Ok, Err>)
	}
	
	/*#[inline(always)]
	pub fn write_low_color<H: ColorHeadArray, V: ColorValueArray>(&mut self) -> Result<(), fmt::Error> {
		ColorWrite::<H, V>::static_fmt_write(&mut self.write)
	}*/
	
	pub fn write_reset(&mut self) -> Result<Ok, Err> {
		//write!(&mut self.write, "\x1b[0m")
		self.write.write_str("\x1b[0m")
	}
	pub fn reset_len() -> usize {
		"\x1b[0m".len()
	}
}

/*impl ColorBuilder<Vec<u8>, std::io::Error> {
	pub fn write_reset(&mut self) -> Result<(), std::io::Error> {
		//write!(self.write, "\x1b[0m");
		self.write.extend_from_slice(b"\x1b[0m");
		Ok( () )
	}
}*/


impl<'a, W, Ok, Err>  From<W> for ColorBuilder<'a, W, Ok, Err> where W: WriteStr<'a, Ok, Err> {
	#[inline(always)]
	fn from(w: W) -> Self {
		Self::new(w)
	}
}


/*impl<'a, W, Ok, Err> WriteFmt<E> for ColorBuilder<'a, W, Ok, Err> where W: WriteStr<'a, Ok, Err> {
	#[inline(always)]
	fn write_fmt(&mut self, args: Arguments) -> Result<(), E> {
		self.write.write_fmt(args)
	}
}*/

impl<'a, W, Ok, Err> Deref for ColorBuilder<'a, W, Ok, Err> where W: WriteStr<'a, Ok, Err> {
	type Target = W;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.write	
	}
}

impl<'a, W, Ok, Err> DerefMut for ColorBuilder<'a, W, Ok, Err> where W: WriteStr<'a, Ok, Err> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.write
	}
}

impl<'a, W, Ok, Err> Drop for ColorBuilder<'a, W, Ok, Err> where W: WriteStr<'a, Ok, Err> {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = self.write_reset();
	}
}


