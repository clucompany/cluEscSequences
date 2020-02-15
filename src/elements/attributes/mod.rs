
use crate::writers::drop::DropWriter;
use crate::elements::reset::EscResetAllAttr;
use crate::elements::reset::EscResetAttr;
use cluExtIO::generic::WriteStr;
use crate::elements::reset::ResetFlashing;
use crate::elements::reset::ResetHidden;
use crate::elements::reset::ResetReverse;
use crate::elements::reset::ResetUnderline;
use crate::elements::reset::ResetDim;
use crate::elements::reset::ResetBold;
use crate::static_element::EscSeqStatic;

pub mod colors;

escseq_data! {
	@attr() {
		//Жирный
		#[reset(ResetBold)]
		pub Bold or Bright = "1";
		
		//Тусклый
		#[reset(ResetDim)]
		pub Dim = "2";
		
		//Подчеркивание
		#[reset(ResetUnderline)]
		pub Underline = "4";
		
		//мигающий
		#[reset(ResetFlashing)]
		pub Flashing or Blink = "5";
		
		#[reset(ResetReverse)]
		pub Reverse or InvertedColors = "7";
		
		#[reset(ResetHidden)]
		pub Hidden or Invisible = "8";
	}
}

// methods
pub trait EscAttr {
	type ResetAttr: EscSeqStatic + EscResetAttr;
	type ResetAllAttr: EscSeqStatic + EscResetAllAttr;
	
	#[inline]
	fn reset_attr<W, TOK, TERR>(data: W) -> std::result::Result<TOK, TERR> where W: WriteStr<Ok=TOK, Err=TERR> {
		Self::ResetAttr::write(data)
	}
	
	#[inline]
	fn reset_all_attr<W, TOK, TERR>(data: W) -> std::result::Result<TOK, TERR> where W: WriteStr<Ok=TOK, Err=TERR> {
		Self::ResetAllAttr::write(data)
	}
	
	#[inline]
	fn reset_attr_write<W, TOK, TERR>(data: W) -> DropWriter<W, TOK, TERR, Self::ResetAttr> where W: WriteStr<Ok=TOK, Err=TERR> {
		DropWriter::new(data)
	}
	
	#[inline]
	fn reset_all_attr_write<W, TOK, TERR>(data: W) -> DropWriter<W, TOK, TERR, Self::ResetAllAttr> where W: WriteStr<Ok=TOK, Err=TERR> {
		DropWriter::new(data)
	}
}