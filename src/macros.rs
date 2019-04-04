
use crate::FontSeqLen;
use cluExtIO::generic::WriteStr;
use crate::FontSeqEscape;
use std::io;
use std::fmt;
use std::fmt::Write;

#[macro_export]
macro_rules! fontseg_escape {
	[
		$([ $($i_life: lifetime),*  $($i2_name: ident),*  ])* $name: ident 
			$(< $($name_ty: ident),*  >)*
			
			$( where $($i3:ty : $i3_path: path),* )*
		{	
			//$($all_tt:tt)*
			#len[ $($len_tt:tt)* ]
			
			#write[ $($write_tt:tt)* ]
		}
		
		$($tt:tt)*
	] => {
		
		impl $(< $($i_life),* $($i2_name),* >)* FontSeqLen for $name $(< $($name_ty),* >)*
			$( where $( $i3 : $i3_path),* )*
		{
			$crate::raw_len_fontseg_escape! {
				/*@$([ $($i_life),*  $($i2_name),*  ])* $name ;
				@$(< $($name_ty),*  >)* ;
					
				@$( where $($i3 : $i3_path),* )* ;*/
				
				$($len_tt)*
			}
		}
		
		
		impl $(< $($i_life),* $($i2_name),* >)* FontSeqEscape for $name $(< $($name_ty),* >)*
			$( where $( $i3 : $i3_path),* )*
		{
			$crate::raw_fontseg_escape! {
				/*@$([ $($i_life),*  $($i2_name),*  ])* $name ;
				@$(< $($name_ty),*  >)* ;
					
				@$( where $($i3 : $i3_path),* )* ;*/
				
				$($write_tt)*
			}
		}
		
		$crate::fontseg_escape! {
			$($tt)*	
		}
	};
	
	[
		$([ $($i_life: lifetime),*  $($i2_name: ident),*  ])* pub enum $name: ident 
			$(< $($name_ty: ident),*  >)*
			
			$( where $($i3:ty : $i3_path: path),* )*
		{	
			//$($all_tt:tt)*
			#len[ $($len_tt:tt)* ]
			
			#write[ $($write_tt:tt)* ]
		}
		
		$($tt:tt)*
	] => {
		#[derive(Debug)]
		pub enum $name {}
		
		impl $(< $($i_life),* $($i2_name),* >)* $crate::FontSeqLen for $name $(< $($name_ty),* >)*
			$( where $( $i3 : $i3_path),* )*
		{
			$crate::raw_len_fontseg_escape! {
				/*@$([ $($i_life),*  $($i2_name),*  ])* $name ;
				@$(< $($name_ty),*  >)* ;
					
				@$( where $($i3 : $i3_path),* )* ;*/
				
				$($len_tt)*
			}
		}
		
		
		impl $(< $($i_life),* $($i2_name),* >)* $crate::FontSeqEscape for $name $(< $($name_ty),* >)*
			$( where $( $i3 : $i3_path),* )*
		{
			$crate::raw_fontseg_escape! {
				/*@$([ $($i_life),*  $($i2_name),*  ])* $name ;
				@$(< $($name_ty),*  >)* ;
					
				@$( where $($i3 : $i3_path),* )* ;*/
				
				$($write_tt)*
			}
		}
		
		$crate::fontseg_escape! {
			$($tt)*	
		}
	};
	
	() => ()
}



#[macro_export]
macro_rules! raw_len_fontseg_escape {
	(
		/*@$([ $($i_life: lifetime),*  $($i2_name: ident),*  ])* $name: ident ;
		@$(< $($name_ty: ident),*  >)* ;
			
		@$( where $($i3:ty : $i3_path: path),* )* ;*/
		
		LEN_ELEMENTS = $len_elements:expr;
		
		$($tt:tt)* 
	) => {
		
		const LEN_ELEMENTS: usize = $len_elements;
		
		$crate::raw_len_fontseg_escape! {
			$($tt)*	
		}
	};
	
	(
		/*@$([ $($i_life: lifetime),*  $($i2_name: ident),*  ])* $name: ident ;
		@$(< $($name_ty: ident),*  >)* ;
			
		@$( where $($i3:ty : $i3_path: path),* )* ;*/
		
		ELEMENT_LEN = $raw_elements:expr;
		
		$($tt:tt)*
	) => {
		
		const ELEMENT_LEN: usize = $raw_elements;
		
		$crate::raw_len_fontseg_escape! {
			$($tt)*	
		}
	};
	
	(
		$v:expr
	) => {
		const LEN_ELEMENTS: usize = 1;
		const ELEMENT_LEN: usize = $v;
				
		/*$crate::raw_len_fontseg_escape! {
			$($tt)*	
		}*/	
	};
	
	/*(
		@$([ $($i_life: lifetime),*  $($i2_name: ident),*  ])* $name: ident ;
		@$(< $($name_ty: ident),*  >)* ;
			
		@$( where $($i3:ty : $i3_path: path),* )* ;
		
		$($tt:tt)* 
	) => {};*/
	
	() => ()
}



#[macro_export]
macro_rules! raw_fontseg_escape {
	(
		/*@$([ $($i_life: lifetime),*  $($i2_name: ident),*  ])* $name: ident ;
		@$(< $($name_ty: ident),*  >)* ;
			
		@$( where $($i3:ty : $i3_path: path),* )* ;*/
		
		$str:expr, $byte:expr;
		$($tt:tt)* 
	) => {
		const ESC_DATA: &'static str = concat!("\x1b[", $str, "m");
		const R_ESC_DATA: &'static [u8] = concat_bytes!("\x1b[", $str, "m");
		
		const ELEMENT_DATA: &'static str = $str;
		const R_ELEMENT_DATA: &'static [u8] = $byte;
		
		
		
		/*#[inline]
		fn io_write<W: std::io::Write>(mut w: W) -> Result<(), std::io::Error> {
			if let Err(e) = w.write(concat_bytes!("\x1b[", $str, "m")) {
				return Err(e);	
			}
			Ok( () )
		}
		
		#[inline(always)]
		fn fmt_write<W: std::fmt::Write>(mut w: W) -> Result<(), std::fmt::Error> {
			w.write_str(concat!("\x1b[", $str, "m"))
		}
		
		#[inline(always)]
		fn clustr_write<'a, W: cluExtIO::generic::WriteStr<'a, (), E>, E>(mut w: W) -> Result<(), E> {
			w.write_str(concat!("\x1b[", $str, "m"))
		}*/
		
		$crate::raw_fontseg_escape! {
			/*@$([ $($i_life),*  $($i2_name),*  ])* $name ;
			@$(< $($name_ty),*  >)* ;
					
			@$( where $($i3 : $i3_path),* )* ;*/
			
			$($tt)*	
		}
	};
	
	
	/*(
		@$([ $($i_life: lifetime),*  $($i2_name: ident),*  ])* $name: ident ;
		@$(< $($name_ty: ident),*  >)* ;
			
		@$( where $($i3:ty : $i3_path: path),* )* ;
		
		$($tt:tt)* 
	) => {};*/
	
	
	() => ()
	
}



pub enum Test<T> {
	Test(T)
}

pub trait Ts {}


fontseg_escape! {	
	[T] Test<T> where T: Ts {
		#len		[1]
		#write	["1", b"1";]
	}
	
	
	pub enum Reset {
		#len		[1]
		#write	["1", b"1";]
	}
	
	/*impl<T> for Test<T>[T: Ts] {
		len_elements	||	1;
		len			||	10;
		
		~write		||	"\x1b[",;
		//clu_write		|w|	w.write_str("test");
	}*/
}


