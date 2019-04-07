
use crate::EscSequency;

pub trait HeadEscSequency: EscSequency {}


escseq_data! {
	HeadEscSequency {
		pub Empty = "0";
		//Жирный
		pub Bold = "1";
		
		//Подчеркивание
		pub Underline = "4";
		
		//мигающий
		pub Flashing = "5";
		pub InvertedColors = "7";
		pub Invisible = "8";
	}
}


