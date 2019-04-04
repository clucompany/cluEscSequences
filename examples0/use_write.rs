
#[macro_use]
extern crate ColorGeneric;

use ColorGeneric::colors::Blue;
use ColorGeneric::colors::BrightBlue;
use ColorGeneric::ColorGeneric;

fn main() {
	{//macros write

		let stdout = ::std::io::stdout();
		let mut lock_stdio = stdout.lock();
	
		write_color!(&mut lock_stdio, BrightBlue, "{} {} macro write...", 12345, "str");
		write_color!(lock_stdio, BrightBlue, "{} {} macro write...", 123465, "str");
	}
	
	{//write struct
		let writer = Blue::writer();
	
		let stdout = ::std::io::stdout();
		let mut lock_stdio = stdout.lock();
	
		writer.writen(&mut lock_stdio, b"TestWriten").unwrap();

	}

	{//macros writen

		writen_color!(&mut ::std::io::stdout(), BrightBlue, "{} {}", 123, "str");
		//mut 

		writen_color!(::std::io::stdout(), BrightBlue, "{} {}", 12345, "str");
		//move mut
	}
}


