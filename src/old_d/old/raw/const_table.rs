macro_rules! build_const_color {
	( 	
		$name_mode:tt : $name_mode_str:expr;
		
		$(  $const_name:tt : $t:tt = $value:expr )+ 
	) => {
		
		build_const_color!(
			$name_mode : $name_mode_str;
			
			$(  
				$const_name : $t = $value , stringify!($const_name), stringify!($t), stringify!($value) ;
			)+ 
		);
		
	};
	
	( 	
		$name_mode:tt : $name_mode_str:expr;
		

		$(  $const_name:tt : $t:tt = $value:expr , $c_str:expr, $t_str:expr, $v_str:expr ; )+ 
	) => {
		
		#[doc = $name_mode_str]
		pub mod $name_mode {
			$(
				#[doc = "Color Const `"]
				#[doc = $c_str]
				#[doc = "` `"]
				#[doc = $v_str]
				#[doc = "` `"]
				pub const $const_name: &$t = $value;
			)+
		}
		
	};
}



build_const_color!(
	default: "30-37: BLACK-WHITE; 90-97: BRIGHT_BLACK-BRIGHT_WHITE";
	
	BLACK: str 	= "30"
	RED: str 	= "31"
	GREEN: str 	= "32"
	YELLOW: str 	= "33"
	BLUE: str 	= "34"
	MAGENTA: str 	= "35"
	CYAN: str	= "36"
	WHITE: str 	= "37"
	
	BRIGHT_BLACK: str 	= "90"
	BRIGHT_RED: str 	= "91"
	BRIGHT_GREEN: str 	= "92"
	BRIGHT_YELLOW: str 	= "93"
	BRIGHT_BLUE: str 	= "94"
	BRIGHT_MAGENTA: str 	= "95"
	BRIGHT_CYAN: str	= "96"
	BRIGHT_WHITE: str 	= "97"
);


build_const_color!(
	b_default: "30-37: BLACK-WHITE; 90-97: BRIGHT_BLACK-BRIGHT_WHITE";
	
	BLACK: [u8] 			= b"30"
	RED: [u8] 			= b"31"
	GREEN: [u8] 			= b"32"
	YELLOW: [u8] 			= b"33"
	BLUE: [u8] 			= b"34"
	MAGENTA: [u8] 		= b"35"
	CYAN: [u8]			= b"36"
	WHITE: [u8] 			= b"37"
	
	BRIGHT_BLACK: [u8] 		= b"90"
	BRIGHT_RED: [u8] 		= b"91"
	BRIGHT_GREEN: [u8] 		= b"92"
	BRIGHT_YELLOW: [u8] 		= b"93"
	BRIGHT_BLUE: [u8] 		= b"94"
	BRIGHT_MAGENTA: [u8] 	= b"95"
	BRIGHT_CYAN: [u8]		= b"96"
	BRIGHT_WHITE: [u8] 		= b"97"
);


