
use cluExtIO::generic::WriteStr;
use crate::FontSeqEscape;
use std::io;
use std::fmt::Write;

#[macro_export]
macro_rules! new_fontseg_esc {
	[
		impl $(< $($i_life: lifetime),*  $($i2_name: ident),*  >)*
			FontSeqEscape for $name: ident $(< $($name_ty: ident),* >)*
			
			$( where $( $i3:ty : $i3_path: path),* )*
		{
			
			$($all_tt:tt)*
		}
		$($tt:tt)*
	] => {
		impl $(< $($i_life),* $($i2_name),* >)* FontSeqEscape for $name $(< $($name_ty),* >)*
			$( where $( $i3 : $i3_path),* )*
		{
			raw_fontseg_esc! {
				$($all_tt)*
			}
		}
		
		new_fontseg_esc! {
			$($tt)*	
		}
	};
	
	() => ()
}


#[macro_export]
macro_rules! raw_fontseg_esc {
	/*[ #len: $v:expr; $($tt:tt)* ] => {
		#[inline(always)]
		fn len() -> usize {
			$v
		}
		
		raw_fontseg_esc! {
			$($tt)*
		}
	};
	
	[ #len_elements: $v:expr; $($tt:tt)* ] => {
		#[inline(always)]
		fn len_elements() -> usize {
			$v
		}
		
		raw_fontseg_esc! {
			$($tt)*
		}
	};*/
	
	( len || $v:expr; $($tt:tt)* ) => {
		#[inline(always)]
		fn len() -> usize {
			$v
		}
		
		raw_fontseg_esc! {
			$($tt)*
		}
	};
	
	( len_elements || $v:expr; $($tt:tt)* ) => {
		#[inline(always)]
		fn len_elements() -> usize {
			$v
		}
		
		raw_fontseg_esc! {
			$($tt)*
		}
	};
	
	( ~write || $v:expr; $($tt:tt)* ) => {
		fn io_write<W: io::Write>(w: W) -> Result<(), io::Error> {
			w.write()
		}
		
		raw_fontseg_esc! {
			$($tt)*
		}
	};
	
	/*( clu_write |$w: ident| $v:expr; $($tt:tt)* ) => {
		fn clustr_write<'a, W: WriteStr<'a, (), E>, E>(w: W) -> Result<(), E> {
			let mut $w = w;
			$v
		}
		
		raw_fontseg_esc! {
			$($tt)*
		}
	};*/
	
	
	() => ()
	
}



pub enum Test<T> {
	Test(T)
}

pub trait Ts {}
new_fontseg_esc! {
	impl<T> FontSeqEscape for Test<T> where T: Ts {
		len_elements	||	1;
		len			||	10;
		
		~write		||	"\x1b[",;
		//clu_write		|w|	w.write_str("test");
	}
}

