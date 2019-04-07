

use crate::EscSequency;

pub trait BackEscSequency: EscSequency {}


escseq_data! {
	//Background
	BackEscSequency {
		pub BackBlack = "40";
		pub BackRed = "41";
		pub BackGreen = "42";
		pub BackYellow = "43";
		pub BackBlue = "44";
		pub BackPurple = "45";
		pub BackLightBlue = "46";
		pub BackWhite = "47";
	}
}
