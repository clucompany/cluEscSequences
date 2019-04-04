


#[macro_export]
macro_rules! fontseg_table {
	[$name:ident ($a:ty $(, $b:ty)*)  $($tt:tt)*] => {
		pub enum $name {}
		
		impl crate::FontSeqLen for $name {
			//$crate::raw_len_fontseg_escape! {
				const ELEMENT_LEN: usize = 
					<$a as crate::FontSeqLen>::ELEMENT_LEN 
					
					$(
						+ unsafe { cluConstConcat::ignore_feature::const_str_len(";") } + 
						<$b as crate::FontSeqLen>::ELEMENT_LEN
					)*;
					
				const LEN_ELEMENTS: usize = 
					<$a as crate::FontSeqLen>::LEN_ELEMENTS 
					
					$(
						+ unsafe { cluConstConcat::ignore_feature::const_str_len(";") } + 
						<$b as crate::FontSeqLen>::LEN_ELEMENTS
					)*;
			//}
		}
		
		impl crate::FontSeqEscape for $name {
			cluConstConcat::const_data! {
				const ESC_DATA: &'static str = "\x1b[",
					<$name as crate::FontSeqEscape>::ELEMENT_DATA,
					"m";
					
				const R_ESC_DATA: &'static [u8] = b"\x1b[",
					<$name as crate::FontSeqEscape>::R_ELEMENT_DATA,
					b"m";
				
				const ELEMENT_DATA: &'static str =
					<$a as crate::FontSeqEscape>::ELEMENT_DATA
					$(,";", <$b as crate::FontSeqEscape>::ELEMENT_DATA)*;
					
				const R_ELEMENT_DATA: &'static [u8] =
					<$a as crate::FontSeqEscape>::R_ELEMENT_DATA
					$(,b";", <$b as crate::FontSeqEscape>::R_ELEMENT_DATA)*;
			}
		
			
		}
		
		$crate::fontseg_table!{ $($tt)* }
	};
	
	() => ()
}

/*
fontseg_table! {
	(FontSeqReset, FontSeqReset)
}
*/
