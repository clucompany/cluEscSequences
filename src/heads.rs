

fontseg_escape! {
	pub enum Empty {
		#len		[1]
		#write	["0", b"0";]
	}
	
	pub enum Bold {
		#len		[1]
		#write	["1", b"1";]
	}
	
	//Жирный
	pub enum Underline {
		#len		[1]
		#write	["4", b"4";]
	}
	
	//Подчёркнутый
	pub enum Flashing {
		#len		[1]
		#write	["5", b"5";]
	}
	
	pub enum InvertedColors {
		#len		[1]
		#write	["7", b"7";]
	}
	
	pub enum Invisible {
		#len		[1]
		#write	["8", b"8";]
	}
}


