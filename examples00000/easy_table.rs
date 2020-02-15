
#[macro_use]
extern crate cluEscSequence;

use cluEscSequence::cluExtIO::ImmutWrite;
use cluEscSequence::colors::LightBlue;
use cluEscSequence::heads::Underline;
use cluEscSequence::colors::White;
use cluEscSequence::back_colors::BackBlack;
use cluEscSequence::heads::Bold;
use cluEscSequence::EscSequence;
use cluEscSequence::colors::Red;
use cluEscSequence::colors::Blue;


escseq_scheme! {
	Head(BackBlack, Bold)
	
	
	OutHead(Head, Blue)["[1]: "](Red)["[2]: "];
	OutTwoHead(Head, White)["[3]: "](Underline, LightBlue)["[4]: "];
}

fn main() {
	println!("{}", OutHead::display());
	std::io::stdout().flush().unwrap();
	println!("{}", OutTwoHead::display());
	std::io::stdout().flush().unwrap();
}


