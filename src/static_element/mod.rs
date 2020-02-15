
use crate::data::EscSeqData;
use cluExtIO::generic::WriteStr;
use crate::writers::drop::DropWriter;
use crate::static_element::r#static::EscElementSeq;

pub mod r#static;

pub trait EscSeqStatic: EscSeqData {
	#[inline]
	fn new() -> EscElementSeq<Self> where Self: Sized {
		EscElementSeq::new()
	}

	#[inline]
	fn drop_write<W, TOK, TERR>(data: W) -> DropWriter<W, TOK, TERR, Self> where W: WriteStr<Ok=TOK, Err=TERR>, Self: std::marker::Sized {
		DropWriter::new(data)
	}
		
	#[inline(always)]
	fn head_write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(mut w: W) -> Result<W::Ok, W::Err> {
		w.write_str(Self::HEAD_DATA)
	}

	#[inline(always)]
	fn write<W: WriteStr<Ok=TOK, Err=TERR>, TOK, TERR>(mut w: W) -> Result<W::Ok, W::Err> {
		w.write_str(Self::ESC_DATA)
	}
}

impl<T> EscSeqStatic for T where T: EscSeqData {}