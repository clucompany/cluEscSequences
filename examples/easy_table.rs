
#[macro_use]
extern crate cluEscSequency;

use cluEscSequency::cluExtIO::ImmutWrite;
use cluEscSequency::colors::LightBlue;
use cluEscSequency::heads::Underline;
use cluEscSequency::colors::White;
use cluEscSequency::back_colors::BackBlack;
use cluEscSequency::heads::Bold;
use cluEscSequency::EscSequency;
use cluEscSequency::colors::Red;
use cluEscSequency::colors::Blue;


escseq_table! {
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


