#![feature(const_fn)]
//Copyright 2019 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	  http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1819
//

/*!
Methods for formatted recording of color output.

# Easy methods of formatted recording

```
#[macro_use]
extern crate ColorGeneric;

let str_colored = color!(blue, "test");
println!("{}", str_colored);
	
let str_colored = color!(blue, bold, "test");
println!("{}", str_colored);

	
let str_colored = color!(bright_red, bold, "test");
println!("{}", str_colored);
```

# Generating a string using color types

```
#[macro_use]
extern crate ColorGeneric;

use ColorGeneric::colors::ColorGeneric;
use ColorGeneric::colors::BrightRed;

let string = BrightRed::string_fmt( format_args!("[{:?}] {}", TEST, str) );
let string = BrightRed::stringn( "color str!" );

```


# Recording macros in `Write trait`

```
#[macro_use]
extern crate ColorGeneric;

use ColorGeneric::colors::Blue;
use ColorGeneric::colors::BrightBlue;


writen_color!(&mut ::std::io::stdout(), BrightBlue, "OutValueTest {}", 123);
writen_color!(&mut ::std::io::stdout(), BrightBlue, "OutValueTest2 {}", 12345);
writen_color!(&mut File::open("color_file.txt"), BrightBlue, "Color Str:)", 12345);
```

# Recording using color types

```
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

```

# Use move color arguments
```
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

```


# Use ColorWriter
```
#[macro_use]
extern crate ColorGeneric;

use ColorGeneric::colors::Blue;

let writer = Blue::writer();
	
let stdout = ::std::io::stdout();
let mut lock_stdio = stdout.lock();
	
writer.writen(&mut lock_stdio, b"TestWriten").unwrap();
```

All other functions are implemented in color mod with the help of ColorGeneric!

*/


#[macro_use]
///Manual methods for color formatting.
pub mod raw;

///Generalized types of colors.
pub mod colors;

///Additional methods of color recording.
pub mod writer;


macro_rules! build_colored {
	( $(  $color:tt | $name:ident | $name2:ident )+ ) => {
		
		///Concat macro for color generation.
		///```	
		///DATA | NAME_COLOR		| NAME2_COLOR
		///---------------------------------------------
		///"30" | black			| BLACK
		///"31" | red			| RED
		///"32" | green			| GREEN
		///"33" | yellow			| YELLOW
		///"34" | blue			| BLUE
		///"35" | magenta			| MAGENTA
		///"36" | cyan			| CYAN
		///"37" | white			| WHITE
		///
		///"90" | bright_black		| BRIGHT_BLACK
		///"91" | bright_red		| BRIGHT_RED
		///"92" | bright_green		| BRIGHT_GREEN
		///"93" | bright_yellow		| BRIGHT_YELLOW
		///"94" | bright_blue		| BRIGHT_BLUE
		///"95" | bright_magenta		| BRIGHT_MAGENTA
		///"96" | bright_cyan		| BRIGHT_CYAN
		///"97" | bright_white		| BRIGHT_WHITE
		///```
		///
		///```
		///let str_colored = color!(blue, bold, "test");
		///println!("{}", str_colored);
		///```
		
		#[macro_export]
		macro_rules! color_args {
			$(
				
				($name, $s:expr) => {
					format_args!(
						"{}{}{}",
						
						concat!(
							raw_color!(start),
							$color,
							raw_color!(end_color),
						), 
						$s, 
						raw_color!(end),
					)
				};
				
				($name, bold, $s:expr) => {
					format_args!(
						"{}{}{}{}",
						
						concat!(
							raw_color!(start),
							$color,
							raw_color!(end_color),
						), 
						raw_color!(bold), 
						$s, 
						raw_color!(end),
					)
				};
				
				($name2, $s:expr) => { color_args!($name, $s) };
				($name2, bold, $s:expr) => { color_args!($name, bold, $s) };
				
			)+
		}
		
		///A concatenated macro for generating a colored static string.
		#[macro_export]
		macro_rules! color {
			$(
				
				($name, $s:expr) => {
					concat!(
						concat!(
							raw_color!(start),
							$color,
							raw_color!(end_color),
						), 
						$s, 
						raw_color!(end),
					)
				};
				
				($name, bold, $s:expr) => {
					concat!(
						concat!(
							raw_color!(start),
							$color,
							raw_color!(end_color),
						), 
						raw_color!(bold), 
						$s, 
						raw_color!(end),
					)
				};
				
				($name2, $s:expr) => { color_args!($name, $s) };
				($name2, bold, $s:expr) => { color_args!($name, bold, $s) };
				
			)+
		}
		
		
		///A concatenated macro for creating a color dynamic string.
		///```	
		///DATA | NAME_COLOR		| NAME2_COLOR
		///---------------------------------------------
		///"30" | black			| BLACK
		///"31" | red			| RED
		///"32" | green			| GREEN
		///"33" | yellow			| YELLOW
		///"34" | blue			| BLUE
		///"35" | magenta			| MAGENTA
		///"36" | cyan			| CYAN
		///"37" | white			| WHITE
		///
		///"90" | bright_black		| BRIGHT_BLACK
		///"91" | bright_red		| BRIGHT_RED
		///"92" | bright_green		| BRIGHT_GREEN
		///"93" | bright_yellow		| BRIGHT_YELLOW
		///"94" | bright_blue		| BRIGHT_BLUE
		///"95" | bright_magenta		| BRIGHT_MAGENTA
		///"96" | bright_cyan		| BRIGHT_CYAN
		///"97" | bright_white		| BRIGHT_WHITE
		///```
		#[macro_export]
		macro_rules! color_format {
			$(
				($name, $s:expr) => {
					format!("{}", color_args!($name, $s))
				};
				($name2, $s:expr) => { color!($name, $s) };
				
				($name, bold, $s:expr) => {
					format!("{}", color_args!($name, bold, $s))
				};
				($name2, bold, $s:expr) => { color!($name, bold, $s) };	
			)+
		}
		
	}
}

build_colored! (
	"30" |	black			| BLACK
	"31" |	red			| RED
	"32" |	green			| GREEN
	"33" |	yellow		| YELLOW
	"34" |	blue			| BLUE
	"35" |	magenta		| MAGENTA
	"36" |	cyan			| CYAN
	"37" |	white			| WHITE
	
	"90" |	bright_black	| BRIGHT_BLACK
	"91" |	bright_red		| BRIGHT_RED
	"92" |	bright_green	| BRIGHT_GREEN
	"93" |	bright_yellow	| BRIGHT_YELLOW
	"94" |	bright_blue		| BRIGHT_BLUE
	"95" |	bright_magenta	| BRIGHT_MAGENTA
	"96" |	bright_cyan		| BRIGHT_CYAN
	"97" |	bright_white	| BRIGHT_WHITE
);


use std::marker::PhantomData;
use std::fmt::Arguments;
use std::io::Write;
use writer::ColorGenericWriter;
use std::hash::Hash;
use std::fmt::Display;
use std::fmt::Debug;
use std::io;



#[derive(Debug)]
pub struct Color<T> where T: ColorGeneric0 {
	gen_color: PhantomData<T>,
}

impl<T> Color<T> where T: ColorGeneric0 {
	#[inline]
	pub const fn new() -> Self {
		Self {
			gen_color: PhantomData,
		}	
	}
	
}

impl<T> From<()> for Color<T> where T: ColorGeneric0 {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Self::new()
	}
}


pub trait ColorGeneric0: Display {
	fn color_name() -> &'static str;
	
	fn new_color() -> Color<Self> where Self: Sized {
		Color::new()
	}
}




///Common features implemented by the generalized type.
#[allow(non_camel_case_types)]
pub trait ColorGeneric: Debug + Display + Eq + Hash + Ord + PartialEq + PartialOrd {
	///Color str type
	fn raw_color<'a>() -> &'a str;
	
	///Color array type
	fn raw_color_b<'a>() -> &'a [u8];
	
	///Name color
	fn name<'a>() -> &'a str;
	

	#[inline]
	fn writer() -> ColorGenericWriter<Self> where Self: Sized {
		ColorGenericWriter::<Self>::new()
	}
	
	
	#[inline]
	fn string_as<'a, A: AsRef<str>>(asref: A) -> String {
		Self::string(asref.as_ref())
	}
	#[inline]
	fn stringn_as<'a, A: AsRef<str>>(asref: A) -> String {
		Self::stringn(asref.as_ref())
	}
	
	
	fn string_fmt<'a>(fmt: Arguments<'a>) -> String {
		format!("{}{}{}{}{}",
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			fmt, 
			raw_color!(end)
		)
	}
	fn string<'a>(str: &'a str) -> String {
		format!("{}{}{}{}{}",
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			str, 
			raw_color!(end)
		)
	}

	fn stringn<'a>(str: &'a str) -> String {
		format!("{}{}{}{}{}\n",
				raw_color!(start),
					Self::raw_color(),
				raw_color!(end_color),
			str, 
			raw_color!(end)
		)
	}
	fn stringn_fmt<'a>(fmt: Arguments<'a>) -> String {
		format!("{}{}{}{}{}\n",
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			fmt, 
			raw_color!(end)
		)
	}
	
	
	#[inline]
	fn write_as<'a, W: Write, A: AsRef<[u8]>>(w: W, asref: A) -> io::Result<()> {
		Self::write(w, asref.as_ref())
	}
	#[inline]
	fn writen_as<'a, W: Write, A: AsRef<[u8]>>(w: W, asref: A) -> io::Result<()> {
		Self::writen(w, asref.as_ref())
	}
	

	fn write<'a, W: Write>(mut w: W, array: &'a [u8]) -> io::Result<()> {
		write!(w, "{}{}{}{}{}",
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			unsafe { ::std::str::from_utf8_unchecked(array) }, 
			raw_color!(end)
		)
	}

	fn write_str<'a, W: Write>(mut w: W, str: &'a str) -> io::Result<()> {
		write!(w, "{}{}{}{}{}",
						
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			str, 
			raw_color!(end)
		)
	}
	fn write_fmt<'a, W: Write>(mut w: W, fmt: Arguments<'a>) -> io::Result<()> {
		write!(w, "{}{}{}{}{}",
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			fmt, 
			raw_color!(end)
		)
	}
	
	fn writen<'a, W: Write>(mut w: W, array: &'a [u8]) -> io::Result<()> {
		write!(w, "{}{}{}{}{}\n",
						
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			unsafe { ::std::str::from_utf8_unchecked(array) }, 
			raw_color!(end)
		)
	}
	
	fn writen_str<'a, W: Write>(mut w: W, str: &'a str) -> io::Result<()> {
		write!(w, "{}{}{}{}{}\n",
						
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			str, 
			raw_color!(end)
		)
	}

	fn writen_fmt<'a, W: Write>(mut w: W, fmt: Arguments<'a>) -> io::Result<()> {
		write!(w, "{}{}{}{}{}\n",
			raw_color!(start),
				Self::raw_color(),
			raw_color!(end_color),
			fmt, 
			raw_color!(end)
		)
	}

	fn with_color_fmt<'a, F: Fn(&Arguments) -> T, T: 'a>(args: Arguments<'a>, function: F) -> T {
		function(
			&format_args!("{}{}{}{}{}",
						
				raw_color!(start),
					Self::raw_color(),
				raw_color!(end_color),
				args, 
				raw_color!(end)
			)
		)
	}
	fn once_with_color_fmt<'a, F: FnOnce(&Arguments) -> T, T: 'a>(args: Arguments<'a>, function: F) -> T {
		function(
			&format_args!("{}{}{}{}{}",
						
				raw_color!(start),
					Self::raw_color(),
				raw_color!(end_color),
				args, 
				raw_color!(end)
			)
		)
	}
	
	fn mut_with_color_fmt<'a, F: FnMut(&Arguments) -> T, T: 'a>(args: Arguments<'a>, mut function: F) -> T {
		function(
			&format_args!("{}{}{}{}{}",
						
				raw_color!(start),
					Self::raw_color(),
				raw_color!(end_color),
				args, 
				raw_color!(end)
			)
		)
	}
}


impl<'l, T: ColorGeneric> ColorGeneric for &'l T {
	#[inline(always)]
	fn raw_color<'a>() -> &'a str { T::raw_color() }
	
	#[inline(always)]
	fn raw_color_b<'a>() -> &'a [u8] { T::raw_color_b() }
	
	#[inline(always)]
	fn name<'a>() -> &'a str { T::name() }
	

	#[inline(always)]
	fn writer() -> ColorGenericWriter<Self> where Self: Sized { ColorGenericWriter::<Self>::new() }
	
	
	#[inline(always)]
	fn string_as<'a, A: AsRef<str>>(asref: A) -> String { T::string_as(asref) }

	#[inline(always)]
	fn stringn_as<'a, A: AsRef<str>>(asref: A) -> String { T::stringn_as(asref) }
	
	#[inline(always)]
	fn string_fmt<'a>(fmt: Arguments<'a>) -> String { T::string_fmt(fmt) }

	#[inline(always)]
	fn string<'a>(str: &'a str) -> String { T::string(str) }

	#[inline(always)]
	fn stringn<'a>(str: &'a str) -> String { T::stringn(str) }

	#[inline(always)]
	fn stringn_fmt<'a>(fmt: Arguments<'a>) -> String { T::stringn_fmt(fmt) }
	
	#[inline(always)]
	fn write_as<'a, W: Write, A: AsRef<[u8]>>(w: W, asref: A) -> io::Result<()> { T::write_as(w, asref) }

	#[inline(always)]
	fn writen_as<'a, W: Write, A: AsRef<[u8]>>(w: W, asref: A) -> io::Result<()> { T::writen_as(w, asref) }

	#[inline(always)]
	fn write<'a, W: Write>(w: W, array: &'a [u8]) -> io::Result<()> { T::write(w, array) }

	#[inline(always)]
	fn write_str<'a, W: Write>(w: W, str: &'a str) -> io::Result<()> { T::write_str(w, str) }

	#[inline(always)]
	fn write_fmt<'a, W: Write>(w: W, fmt: Arguments<'a>) -> io::Result<()> { T::write_fmt(w, fmt) }

	#[inline(always)]
	fn writen<'a, W: Write>(w: W, array: &'a [u8]) -> io::Result<()> { T::writen(w, array) }
	
	#[inline(always)]
	fn writen_str<'a, W: Write>(w: W, str: &'a str) -> io::Result<()> { T::writen_str(w, str) }

	#[inline(always)]
	fn writen_fmt<'a, W: Write>(w: W, fmt: Arguments<'a>) -> io::Result<()> { T::writen_fmt(w, fmt) }

	#[inline(always)]
	fn with_color_fmt<'a, F: Fn(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::with_color_fmt(args, function) }

	#[inline(always)]
	fn once_with_color_fmt<'a, F: FnOnce(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::once_with_color_fmt(args, function) }
	
	#[inline(always)]
	fn mut_with_color_fmt<'a, F: FnMut(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::mut_with_color_fmt(args, function) }
}



impl<'l, T: ColorGeneric> ColorGeneric for &'l mut T {
	#[inline(always)]
	fn raw_color<'a>() -> &'a str { T::raw_color() }
	
	#[inline(always)]
	fn raw_color_b<'a>() -> &'a [u8] { T::raw_color_b() }
	
	#[inline(always)]
	fn name<'a>() -> &'a str { T::name() }
	

	#[inline(always)]
	fn writer() -> ColorGenericWriter<Self> where Self: Sized { ColorGenericWriter::<Self>::new() }
	
	
	#[inline(always)]
	fn string_as<'a, A: AsRef<str>>(asref: A) -> String { T::string_as(asref) }

	#[inline(always)]
	fn stringn_as<'a, A: AsRef<str>>(asref: A) -> String { T::stringn_as(asref) }
	
	#[inline(always)]
	fn string_fmt<'a>(fmt: Arguments<'a>) -> String { T::string_fmt(fmt) }

	#[inline(always)]
	fn string<'a>(str: &'a str) -> String { T::string(str) }

	#[inline(always)]
	fn stringn<'a>(str: &'a str) -> String { T::stringn(str) }

	#[inline(always)]
	fn stringn_fmt<'a>(fmt: Arguments<'a>) -> String { T::stringn_fmt(fmt) }
	
	#[inline(always)]
	fn write_as<'a, W: Write, A: AsRef<[u8]>>(w: W, asref: A) -> io::Result<()> { T::write_as(w, asref) }

	#[inline(always)]
	fn writen_as<'a, W: Write, A: AsRef<[u8]>>(w: W, asref: A) -> io::Result<()> { T::writen_as(w, asref) }

	#[inline(always)]
	fn write<'a, W: Write>(w: W, array: &'a [u8]) -> io::Result<()> { T::write(w, array) }

	#[inline(always)]
	fn write_str<'a, W: Write>(w: W, str: &'a str) -> io::Result<()> { T::write_str(w, str) }

	#[inline(always)]
	fn write_fmt<'a, W: Write>(w: W, fmt: Arguments<'a>) -> io::Result<()> { T::write_fmt(w, fmt) }

	#[inline(always)]
	fn writen<'a, W: Write>(w: W, array: &'a [u8]) -> io::Result<()> { T::writen(w, array) }
	
	#[inline(always)]
	fn writen_str<'a, W: Write>(w: W, str: &'a str) -> io::Result<()> { T::writen_str(w, str) }

	#[inline(always)]
	fn writen_fmt<'a, W: Write>(w: W, fmt: Arguments<'a>) -> io::Result<()> { T::writen_fmt(w, fmt) }

	#[inline(always)]
	fn with_color_fmt<'a, F: Fn(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::with_color_fmt(args, function) }

	#[inline(always)]
	fn once_with_color_fmt<'a, F: FnOnce(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::once_with_color_fmt(args, function) }
	
	#[inline(always)]
	fn mut_with_color_fmt<'a, F: FnMut(&Arguments) -> C, C: 'a>(args: Arguments<'a>, function: F) -> C { T::mut_with_color_fmt(args, function) }
}


#[macro_export]
///Macro of the formatted entry in the trait.
macro_rules! write_color {
	( $write:expr, $color:tt, $( $arg:tt )* ) => {
		$color::write_fmt($write, format_args!( $( $arg )* ))
	};
}


#[macro_export]
///Macro of the formatted entry in the trait. Adds /n to end.
macro_rules! writen_color {
	( $write:expr, $color:tt, $( $arg:tt )* ) => {
		$color::writen_fmt($write, format_args!( $( $arg )* ))
	};
}


