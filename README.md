# ColorGeneric

[![Build Status](https://travis-ci.org/clucompany/ColorGeneric.svg?branch=master)](https://travis-ci.org/clucompany/ColorGeneric)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/ColorGeneric)](https://crates.io/crates/ColorGeneric)
[![Documentation](https://docs.rs/ColorGeneric/badge.svg)](https://docs.rs/ColorGeneric)


Methods for formatted recording of color output.
# Easy methods of formatted recording

	#[macro_use]
	extern crate ColorGeneric;

	let str_colored = color!(blue, "test");
	println!("{}", str_colored);

	let str_colored = color!(blue, bold, "test");
	println!("{}", str_colored);


	let str_colored = color!(bright_red, bold, "test");
	println!("{}", str_colored);

# Generating a string using color types

	#[macro_use]
	extern crate ColorGeneric;

	use ColorGeneric::colors::ColorGeneric;
	use ColorGeneric::colors::BrightRed;

	let string = BrightRed::string_fmt( format_args!("[{:?}] {}", TEST, str) );
	let string = BrightRed::stringn( "color str!" );

# Recording macros in Write trait

	#[macro_use]
	extern crate ColorGeneric;

	use ColorGeneric::colors::Blue;
	use ColorGeneric::colors::BrightBlue;


	writen_color!(&mut ::std::io::stdout(), BrightBlue, "OutValueTest {}", 123);
	writen_color!(&mut ::std::io::stdout(), BrightBlue, "OutValueTest2 {}", 12345);
	writen_color!(&mut File::open("color_file.txt"), BrightBlue, "Color Str:)", 12345);

# Recording using color types

	#[macro_use]
	extern crate ColorGeneric;

	use ColorGeneric::colors::Blue;
	use ColorGeneric::colors::BrightBlue;


	let mut vec: Vec<u8> = Vec::new(); // For Vec implemented Write!!


	let _e = BrightBlue::write_str(&mut vec, "color str!" );

	let _e = vec.write(b"TestValue"); // For Vec implemented Write!!
	//Also this value will remain without color formatting.

	let _e = BrightBlue::writen_str(&mut vec, "end str.." );

	let _e = BrightRed::writen(&mut vec, b"end value.." );

# Use move color arguments

	#[macro_use]
	extern crate ColorGeneric;

	use ColorGeneric::colors::BrightBlue;

	#[derive(Debug, Default)]
	pub struct Items(usize, usize);

	impl Items {
		#[inline]
		pub fn count(&self) -> usize {
		  self.0 + self.1
		}
	}

	let mut item = Items::default();

	for a in 0..15 {
		BrightGreen::with_color_fmt(format_args!("NUM #{}", a), |fmt_num| {
		  BrightBlue::with_color_fmt(format_args!("ITEM #{:?}", item), |fmt_item| {
			  BrightRed::with_color_fmt(format_args!("ALL_COUNT {}", item.count()), |fmt_count| {
			    println!("{}, {}; {};", fmt_num, fmt_item, fmt_count);
			  });
		  });
		});

		item.0 += 1;
		item.1 += 2;
	}

# Use ColorWriter

	#[macro_use]
	extern crate ColorGeneric;

	use ColorGeneric::colors::Blue;

	let writer = Blue::writer();

	let stdout = ::std::io::stdout();
	let mut lock_stdio = stdout.lock();

	writer.writen(&mut lock_stdio, b"TestWriten").unwrap();

	All other functions are implemented in color mod with the help of ColorGeneric!


# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
