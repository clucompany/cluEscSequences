
use crate::static_element::r#static::EscElementSeq;
use crate::runtime_element::EscRunElementSeq;

pub trait EscSeqData {
	const ESC_DATA: &'static str;
	const HEAD_DATA: &'static str;
	
	fn new() -> EscElementSeq<Self> where Self: std::marker::Sized  {
		EscElementSeq::new()
	}
	
	#[inline]
	fn new_runtime() -> EscRunElementSeq where Self: std::marker::Sized {
		EscRunElementSeq::new::<Self>()
	}
	
	#[inline]
	fn esc_data() -> &'static str {
		Self::ESC_DATA
	}
	
	#[inline]
	fn as_esc_data(&self) -> &'static str {
		Self::ESC_DATA
	}
	
	#[inline]
	fn head_data() -> &'static str {
		Self::HEAD_DATA
	}
	
	#[inline]
	fn as_head_data(&self) -> &'static str {
		Self::HEAD_DATA
	}
}

impl<'a, T> EscSeqData for &'a T where T: EscSeqData {
	const ESC_DATA: &'static str = T::ESC_DATA;
	const HEAD_DATA: &'static str = T::HEAD_DATA;
	
	#[inline]
	fn esc_data() -> &'static str {
		T::ESC_DATA
	}
	
	#[inline]
	fn as_esc_data(&self) -> &'static str {
		T::ESC_DATA
	}
	
	#[inline]
	fn head_data() -> &'static str {
		T::HEAD_DATA
	}
	
	#[inline]
	fn as_head_data(&self) -> &'static str {
		T::HEAD_DATA
	}
}


pub trait EsqSeqLenData {
	const HEAD_DATA_LEN: usize;
	const ESC_DATA_LEN: usize;
	
	#[inline]
	fn head_data_len() -> usize {
		Self::HEAD_DATA_LEN
	}
	
	#[inline]
	fn esc_data_len() -> usize {
		Self::ESC_DATA_LEN
	}
	
	#[inline]
	fn as_head_data_len(&self) -> usize {
		Self::HEAD_DATA_LEN
	}
	
	#[inline]
	fn as_esc_data_len(&self) -> usize {
		Self::ESC_DATA_LEN
	}
}

impl<T> EsqSeqLenData for T where T: EscSeqData {
	const HEAD_DATA_LEN: usize = T::HEAD_DATA.len();
	const ESC_DATA_LEN: usize = T::ESC_DATA.len();
	
	#[inline]
	fn head_data_len() -> usize {
		Self::HEAD_DATA_LEN
	}
	
	#[inline]
	fn esc_data_len() -> usize {
		Self::ESC_DATA_LEN
	}
	
	#[inline]
	fn as_head_data_len(&self) -> usize {
		Self::HEAD_DATA_LEN
	}
	
	#[inline]
	fn as_esc_data_len(&self) -> usize {
		Self::ESC_DATA_LEN
	}
}
