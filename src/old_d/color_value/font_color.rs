

use crate::color_value::ColorValue;
use std::fmt::Arguments;
use std::io;
use std::fmt;
use cluExtIO::WriteFmt;
use crate::len::ArgLen;

use crate::color_value;

color_value!{
	Black_Color[2] "30", b"30";
	fmt	|| "30";
	
	Red_Color[2] "31", b"31";
	fmt	|| "31";
	
	Green_Color[2] "32", b"32";
	fmt	|| "32";
	
	Yellow_Color[2] "33", b"33";
	fmt	|| "33";
	
	DarkBlue_Color[2] "34", b"34";
	fmt	|| "34";
	
	Purple_Color[2] "35", b"35";
	fmt	|| "35";
	
	LightBlue_Color[2] "36", b"36";
	fmt	|| "36";
	
	White_Color[2] "37", b"37";
	fmt	|| "37";
}


