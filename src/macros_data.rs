

#[macro_export]
macro_rules! escseq_data {
	//PUB
	[ $trait:ident { $( pub $data:ident = $($str:expr)*; )* } $($tt:tt)*] => {
		
		$crate::escseq_data! {
			$( pub $data = $($str),*; )*
			$($tt)*
		}
		$(impl $trait for $data {})*
	};
	
	//PUB
	[ $trait:ident { $( $data:ident = $($str:expr)*; )* } $($tt:tt)*] => {
		
		$crate::escseq_data! {
			$( $data = $($str),*; )*
			$($tt)*
		}
		$(impl $trait for $data {})*
	};
	
	
	
	[pub $data:ident = $($str:expr),*; $($tt:tt)*] => {
		pub enum $data {}
		
		$crate::escseq_data!{
			impl $data = $($str),*;
			$($tt)*
		}
	};
	
	[$data:ident = $($str:expr),*; $($tt:tt)*] => {
		enum $data {}
		
		$crate::escseq_data!{
			impl $data = $($str),*;
			$($tt)*
		}
	};
	
	[impl $data:ident = $($str:expr),*;  $($tt:tt)*] => {
		//"\x1b[", $str, "m"
		impl $crate::EscSequency for $data {
			cluConstConcat::const_data! {
				const ESC_DATA: &'static str = "\x1b[", $($str),*, "m";
				const HEAD_DATA: &'static str = $($str),*;
				//const STR_DATA: &'static str = "";
				
				
				/*const R_ESC_DATA: &'static [u8] = <Self as $crate::EscSequency>::ESC_DATA.as_bytes();
				const R_HEAD_DATA: &'static [u8] = <Self as $crate::EscSequency>::HEAD_DATA.as_bytes();
				const R_STR_DATA: &'static [u8] = <Self as $crate::EscSequency>::STR_DATA.as_bytes();*/
			}
		}
		
		$crate::escseq_data!{ $($tt)* }
	};
	
	() => ()
}
