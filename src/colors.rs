

use crate::EscSequency;

pub trait ColorEscSequency: EscSequency {}

escseq_data! {
	//Colors
	ColorEscSequency {
		pub Black = "30";
		pub Red = "31";
		pub Green = "32";
		pub Yellow = "33";
		pub Blue = "34";
		pub Purple = "35";
		pub LightBlue = "36";
		pub White = "37";
	}
}
