
#[macro_use]
extern crate ColorGeneric;

pub fn main() {
	//blue string
	let str_colored = color!(blue, "test");
	println!("{}", str_colored);
	
	//bold blue string
	let str_colored = color!(blue, bold, "test");
	println!("{}", str_colored);
	
	//bright_blue bold string
	let str_colored = color!(bright_blue, bold, "test");
	println!("{}", str_colored);
}
