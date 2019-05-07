


#[macro_export]
macro_rules! escseq_table {
	//NO STR
	[pub $name:ident ($($t:tt)*)  $($tt:tt)*] => {
		pub enum $name {}
		
		$crate::escseq_table! {
			impl $name ($($t)*)
			
			$($tt)* 
		}
	};
	
	[$name:ident ($($t:tt)*)  $($tt:tt)*] => {
		enum $name {}
		
		$crate::escseq_table! {
			impl $name ($($t)*)
			
			$($tt)* 
		}
	};
	
	
	//@
	[pub $name:ident	$( ($($t:tt)*)[$($str:expr),*] )*  ;$($tt:tt)*] => {
		pub enum $name {}
		
		$crate::escseq_table! {
			impl $name $(	($($t)*)[$($str),*]	)* ;
			
			$($tt)* 
		}
	};
	
	[$name:ident	$( ($($t:tt)*)[$($str:expr),*] )*  ;$($tt:tt)*] => {
		enum $name {}
		
		$crate::escseq_table! {
			impl $name $(	($($t)*)[$($str),*]	)* ;
			
			$($tt)* 
		}
	};
	
	
	
	
	
	
	
	[impl $name:ident $( ( $a:ty $(, $b:ty)* ) [$($str:expr),*] )* ;$($tt:tt)*] => {
		impl crate::EscSequency for $name {
			cluConstData::const_data! {
				const ESC_DATA: &'static str = 
					$(
						"\x1b[",
						
						<$a as crate::EscSequency>::HEAD_DATA
						
						$(, ";", <$b as crate::EscSequency>::HEAD_DATA)*
						
						, "m" $(, $str)*
						,
					)* 
					$crate::EscSeqReset::ESC_DATA 
					;
				
				//const STR_DATA: &'static str = "";
				
				const HEAD_DATA: &'static str = "";
			}
		}
		
		/*
		//cluConstData::const_data! {
			//	const ESC_DATA: &'static str = "";
				/*	"\x1b[",
					$(
						<$a as crate::EscSequency>::ELEMENT_DATA
						
						$(, ";", <$b as crate::EscSequency>::ELEMENT_DATA)*
						
						, "m" $(, $str)* 
						
					)* 
					, $crate::EscSeqReset::ESC_DATA ;
				*/
				
				//const ELEMENT_DATA: &'static str = "";
					/*<$a as crate::EscSequency>::ELEMENT_DATA
					$(,";", <$b as crate::EscSequency>::ELEMENT_DATA)**/;
					
				/*	<$a as crate::EscSequency>::ELEMENT_DATA
					$(,";", <$b as crate::EscSequency>::ELEMENT_DATA)*;*/
					
				/*const R_ELEMENT_DATA: &'static [u8] =
					<$a as crate::EscSequency>::R_ELEMENT_DATA
					$(,b";", <$b as crate::EscSequency>::R_ELEMENT_DATA)*;*/
			//}*/
		
		$crate::escseq_table!{ $($tt)* }
	};
	
	
	
	
	[impl $name:ident ($a:ty $(, $b:ty)*)  $($tt:tt)*] => {
		/*impl crate::EscSeqLen for $name {
			const ELEMENT_LEN: usize = 
				<$a as crate::EscSeqLen>::ELEMENT_LEN 
				
				$(
					+ unsafe { cluConstData::ignore_feature::const_str_len(";") } + 
					<$b as crate::EscSeqLen>::ELEMENT_LEN
				)*;
				
			const LEN_ELEMENTS: usize = 
				<$a as crate::EscSeqLen>::LEN_ELEMENTS 
				
				$(
					+ unsafe { cluConstData::ignore_feature::const_str_len(";") } + 
					<$b as crate::EscSeqLen>::LEN_ELEMENTS
				)*;
		}*/
		
		impl crate::EscSequency for $name {
			cluConstData::const_data! {
				const ESC_DATA: &'static str = "\x1b[",
					<$name as crate::EscSequency>::HEAD_DATA,
					"m";
				
				//const STR_DATA: &'static str = "";
				
				/*const R_ESC_DATA: &'static [u8] = b"\x1b[",
					<$name as crate::EscSequency>::R_ELEMENT_DATA,
					b"m";*/
				
				const HEAD_DATA: &'static str =
					<$a as crate::EscSequency>::HEAD_DATA
					$(,";", <$b as crate::EscSequency>::HEAD_DATA)*;
					
				/*const R_ELEMENT_DATA: &'static [u8] =
					<$a as crate::EscSequency>::R_ELEMENT_DATA
					$(,b";", <$b as crate::EscSequency>::R_ELEMENT_DATA)*;*/
			}
		}
		
		$crate::escseq_table!{ $($tt)* }
	};
	
	() => ()
}


