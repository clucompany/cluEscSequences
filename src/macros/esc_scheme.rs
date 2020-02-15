


#[macro_export]
macro_rules! escseq_scheme {
	//NO STR
	[pub $name:ident ($($t:tt)*)  $($tt:tt)*] => {
		pub enum $name {}
		
		$crate::escseq_scheme! {
			impl $name ($($t)*)
			
			$($tt)* 
		}
	};
	
	[$name:ident ($($t:tt)*)  $($tt:tt)*] => {
		enum $name {}
		
		$crate::escseq_scheme! {
			impl $name ($($t)*)
			
			$($tt)* 
		}
	};
	
	
	//@
	[pub $name:ident	$( ($($t:tt)*)[$($str:expr),*] )*  ;$($tt:tt)*] => {
		pub enum $name {}
		
		$crate::escseq_scheme! {
			impl $name $(	($($t)*)[$($str),*]	)* ;
			
			$($tt)* 
		}
	};
	
	[$name:ident	$( ($($t:tt)*)[$($str:expr),*] )*  ;$($tt:tt)*] => {
		enum $name {}
		
		$crate::escseq_scheme! {
			impl $name $(	($($t)*)[$($str),*]	)* ;
			
			$($tt)* 
		}
	};
	
	
	
	
	
	
	
	[impl $name:ident $( ( $a:ty $(, $b:ty)* ) [$($str:expr),*] )* ;$($tt:tt)*] => {
		impl crate::EscSequence for $name {
			cluConstData::const_data! {
				const ESC_DATA: &'static str = 
					$(
						"\x1b[",
						
						<$a as crate::EscSequence>::HEAD_DATA
						
						$(, ";", <$b as crate::EscSequence>::HEAD_DATA)*
						
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
						<$a as crate::EscSequence>::ELEMENT_DATA
						
						$(, ";", <$b as crate::EscSequence>::ELEMENT_DATA)*
						
						, "m" $(, $str)* 
						
					)* 
					, $crate::EscSeqReset::ESC_DATA ;
				*/
				
				//const ELEMENT_DATA: &'static str = "";
					/*<$a as crate::EscSequence>::ELEMENT_DATA
					$(,";", <$b as crate::EscSequence>::ELEMENT_DATA)**/;
					
				/*	<$a as crate::EscSequence>::ELEMENT_DATA
					$(,";", <$b as crate::EscSequence>::ELEMENT_DATA)*;*/
					
				/*const R_ELEMENT_DATA: &'static [u8] =
					<$a as crate::EscSequence>::R_ELEMENT_DATA
					$(,b";", <$b as crate::EscSequence>::R_ELEMENT_DATA)*;*/
			//}*/
		
		$crate::escseq_scheme!{ $($tt)* }
	};
	
	
	
	
	[impl $name:ident ($a:ty $(, $b:ty)*)  $($tt:tt)*] => {
		/*impl crate::EscLenSeq for $name {
			const ELEMENT_LEN: usize = 
				<$a as crate::EscLenSeq>::ELEMENT_LEN 
				
				$(
					+ unsafe { cluConstData::ignore_feature::const_str_len(";") } + 
					<$b as crate::EscLenSeq>::ELEMENT_LEN
				)*;
				
			const LEN_ELEMENTS: usize = 
				<$a as crate::EscLenSeq>::LEN_ELEMENTS 
				
				$(
					+ unsafe { cluConstData::ignore_feature::const_str_len(";") } + 
					<$b as crate::EscLenSeq>::LEN_ELEMENTS
				)*;
		}*/
		
		impl crate::EscSeq for $name {
			$crate::cluConstData::const_data! {
				const ESC_DATA: &'static str = "\x1b[",
					<$a as crate::EscSeq>::HEAD_DATA,
					"m";
				
				//const STR_DATA: &'static str = "";
				
				/*const R_ESC_DATA: &'static [u8] = b"\x1b[",
					<$name as crate::EscSequence>::R_ELEMENT_DATA,
					b"m";*/
				
				const HEAD_DATA: &'static str =
					<$a as crate::EscSeq>::HEAD_DATA
					$(,";", <$b as crate::EscSeq>::HEAD_DATA)*;
					
				/*const R_ELEMENT_DATA: &'static [u8] =
					<$a as crate::EscSequence>::R_ELEMENT_DATA
					$(,b";", <$b as crate::EscSequence>::R_ELEMENT_DATA)*;*/
			}
		}
		
		$crate::escseq_scheme!{ $($tt)* }
	};
	
	() => ()
}


