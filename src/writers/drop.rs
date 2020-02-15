
use crate::data::EscSeqData;
use core::ops::DerefMut;
use core::ops::Deref;
use core::marker::PhantomData;
use cluExtIO::generic::WriteStr;

#[derive(Debug)]
pub struct DropWriter<W, TOK, TERR, ES> where W: WriteStr<Ok=TOK, Err=TERR>, ES: EscSeqData {
	_p: PhantomData<ES>,
	data: W,
}

impl<W, TOK, TERR, ES> DropWriter<W, TOK, TERR, ES> where W: WriteStr<Ok=TOK, Err=TERR>, ES: EscSeqData {
	#[inline]
	pub const fn new(data: W) -> Self {
		Self {
			_p: PhantomData,
			data,
		}
	}
	
	pub fn write(self) {}
}

impl<W, TOK, TERR, ES> WriteStr for DropWriter<W, TOK, TERR, ES> where W: WriteStr<Ok=TOK, Err=TERR>, ES: EscSeqData {
	type Ok = TOK;
	type Err = TERR;
	
	#[inline(always)]
	fn write_str(&mut self, str: &str) -> Result<TOK, TERR> {
		self.data.write_str(str)
	}
	
	#[inline(always)]
	fn write_str_array<'l>(&mut self, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.data.write_str_array(arr)
	}
	
	#[inline(always)]
	fn write_str_lenarray<'l>(&mut self, _all_size: usize, arr: &'l [&str]) -> Result<(), Self::Err> {
		self.data.write_str_lenarray(_all_size, arr)
	}
}

impl<W, TOK, TERR, ES> Deref for DropWriter<W, TOK, TERR, ES> where W: WriteStr<Ok=TOK, Err=TERR>, ES: EscSeqData {
	type Target = W;
	
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<W, TOK, TERR, ES> DerefMut for DropWriter<W, TOK, TERR, ES> where W: WriteStr<Ok=TOK, Err=TERR>, ES: EscSeqData {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl<W, TOK, TERR, ES> Drop for DropWriter<W, TOK, TERR, ES> where W: WriteStr<Ok=TOK, Err=TERR>, ES: EscSeqData {
	fn drop(&mut self) {
		let _e = self.data.write_str(ES::ESC_DATA);
	}
}


#[cfg(test)]
mod test {
	use crate::writers::drop::DropWriter;
	use crate::static_element::EscSeqStatic;
	
	#[test]
	fn checker_drop_writer() {
		let mut string = String::with_capacity(20);
		{
			let mut reset: DropWriter<_,_,_, crate::elements::reset::ResetAllAttr> = DropWriter::new(&mut string);
			reset.push_str("test");
		}
		string.push_str("end");
		assert_eq!(string, "test\u{1b}[0mend");
	}
	
	#[test]
	fn checker_drop_writer_2() {
		let mut string = String::with_capacity(20);
		{
			let mut reset = crate::elements::reset::ResetAllAttr::drop_write(&mut string);
			reset.push_str("test");
		}
		string.push_str("end");
		assert_eq!(string, "test\u{1b}[0mend");
	}
}