

escseq_data! {
	pub EscSeqReset = "0";
}

#[cfg(test)]
mod tests {
	use crate::EscSequency;
	use crate::EscSeqLen;
	use crate::reset::EscSeqReset;
	
	#[test]
	fn test_reset() {		
		{
			let mut vec = Vec::with_capacity(12);
			EscSeqReset::io_write(&mut vec).unwrap();
			
			assert_eq!(vec, b"\x1b[0m");
		}
		
		{
			let mut vec = String::with_capacity(12);
			EscSeqReset::fmt_write(&mut vec).unwrap();
			
			assert_eq!(vec, "\x1b[0m");
		}
		
		{
			let mut str = String::with_capacity(12);
			EscSeqReset::clustr_write(&mut str).unwrap();
		
			assert_eq!(str, "\x1b[0m");
		}
		
		{
			let mut vec = Vec::with_capacity(12);
			EscSeqReset::clustr_write(&mut vec).unwrap();
			
			assert_eq!(vec, b"\x1b[0m");
		}
		
		
		assert_eq!(EscSeqReset::ESC_DATA, "\x1b[0m");
		assert_eq!(EscSeqReset::R_ESC_DATA, b"\x1b[0m");
		
		assert_eq!(EscSeqReset::HEAD_DATA, "0");
		assert_eq!(EscSeqReset::R_HEAD_DATA, b"0");
		
		assert_eq!(EscSeqReset::HEAD_DATA_LEN, "0".len());
		assert_eq!(EscSeqReset::ESC_DATA_LEN, "\x1b[0m".len());
		
	}	
}