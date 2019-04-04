
use std::ops::DerefMut;
use std::ops::Deref;
use cluExtIO::generic::WriteStr;
use crate::FontSeqEscape;
use cluExtIO::phantom::PhantomWriteStr;

fontseg_escape! {
	pub enum FontSeqReset {
		#len		[1]
		#write	["0", b"0";]
	}
}

#[cfg(test)]
mod tests {
	use crate::FontSeqEscape;
	use crate::FontSeqLen;
	use crate::reset::FontSeqReset;
	
	#[test]
	fn test_reset() {		
		{
			let mut vec = Vec::with_capacity(12);
			FontSeqReset::io_write(&mut vec).unwrap();
			
			assert_eq!(vec, b"\x1b[0m");
		}
		
		{
			let mut vec = String::with_capacity(12);
			FontSeqReset::fmt_write(&mut vec).unwrap();
			
			assert_eq!(vec, "\x1b[0m");
		}
		
		{
			let mut str = String::with_capacity(12);
			FontSeqReset::clustr_write(&mut str).unwrap();
		
			assert_eq!(str, "\x1b[0m");
		}
		
		{
			let mut vec = Vec::with_capacity(12);
			FontSeqReset::clustr_write(&mut vec).unwrap();
			
			assert_eq!(vec, b"\x1b[0m");
		}
		
		
		assert_eq!(FontSeqReset::ELEMENT_DATA, "0");
		assert_eq!(FontSeqReset::R_ELEMENT_DATA, b"0");
		
		assert_eq!(FontSeqReset::LEN_ELEMENTS, 1);
		assert_eq!(FontSeqReset::ELEMENT_LEN, "0".len());
		assert_eq!(FontSeqReset::ALL_LEN, "\x1b[0m".len());
	}	
}