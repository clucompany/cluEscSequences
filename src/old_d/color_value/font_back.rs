
use crate::color_value::ColorValue;
use std::fmt::Arguments;
use std::io;
use std::fmt;
use cluExtIO::WriteFmt;
use crate::len::ArgLen;

#[macro_use]
use crate::color_value;

color_value!{
	Back_Black_Color[2] "40", b"40";
	fmt	|| "40";
	
	Back_Red_Color[2] "41", b"41";
	fmt	|| "41";
	
	Back_Green_Color[2] "42", b"42";
	fmt	|| "42";
	
	Back_Yellow_Color[2] "43", b"43";
	fmt	|| "43";
	
	Back_DarkBlue_Color[2] "44", b"44";
	fmt	|| "44";
	
	Back_Purple_Color[2] "45", b"45";
	fmt	|| "45";
	
	Back_LightBlue_Color[2] "46", b"46";
	fmt	|| "46";
	
	Back_White_Color[2] "47", b"47";
	fmt	|| "47";
}