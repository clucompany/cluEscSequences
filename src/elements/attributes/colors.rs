

pub trait EscColorAttr {}

pub mod table_8or16_bit {
	pub trait Esc8or16BitColors {}
	
	//
	pub mod back {
		use crate::elements::reset::ResetBackColor;
		use crate::elements::attributes::colors::EscColorAttr;
		use crate::elements::attributes::colors::table_8or16_bit::Esc8or16BitColors;

		pub trait EscBackColorAttr: EscColorAttr {}
		escseq_data! {
			//Background
			#[reset(ResetBackColor)]
			@attr(EscBackColorAttr + EscColorAttr + Esc8or16BitColors) {
				//40
				pub BackBlack or Black					= "40";
				pub BackRed or Red						= "41";
				pub BackGreen or Green					= "42";
				pub BackYellow or Yellow					= "43";
				pub BackBlue or Blue					= "44";
				pub BackMagenta or Magenta				= "45";
				pub BackCyan or Cyan					= "46";
				
				//100
				pub BackDarkGray or DarkGray				= "100";
				pub BackLightRed or LightRed				= "101";
				pub BackLightGreen  or LightGreen			= "102";
				pub BackLightYellow  or LightYellow		= "103";
				pub BackLightBlue  or LightBlue			= "104";
				pub BackLightMagenta  or LightMagenta		= "105";
				pub BackLightCyan  or LightCyan			= "106";
				pub BackWhite or White					= "107";
			}
		}
	}
	
	//
	pub mod foreg {
		use crate::elements::reset::ResetForegColor;
		use crate::elements::attributes::colors::table_8or16_bit::Esc8or16BitColors;
		use crate::elements::attributes::colors::EscColorAttr;

		pub trait EscForegColorAttr: EscColorAttr {}
		escseq_data! {
			//Colors
			#[reset(ResetForegColor)]
			@attr(EscForegColorAttr + EscColorAttr + Esc8or16BitColors) {
				//30
				pub ForegBlack or Black				= "30";
				pub ForegRed or Red					= "31";
				pub ForegGreen or Green				= "32";
				pub ForegYellow or Yellow			= "33";
				pub ForegBlue or Blue				= "34";
				pub ForegMagenta or  Magenta 			= "35";
				pub ForegCyan or Cyan				= "36";
				pub ForeqLightGray or LightGray		= "37";
				
				//90
				pub ForeqDarkGray or DarkGray			= "90";
				pub ForeqLightRed  or LightRed		= "91";
				pub ForeqLightGreen or LightGreen		= "92";
				pub ForeqLightYellow  or LightYellow	= "93";
				pub ForeqLightBlue or LightBlue		= "94";
				pub ForeqLightMagenta or LightMagenta	= "95";
				pub ForeqLightCyan or LightCyan		= "96";
				pub ForeqWhite or White				= "97";
			}
		}
	}
}
