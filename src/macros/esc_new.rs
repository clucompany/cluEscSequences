
#[macro_export]
macro_rules! escseq_data {
	[
		$( #[$($add_data:tt)*] )*
		@$type_t:tt ( $($n:ident $(+ $n2:ident)*)? ) { $($tt:tt)* }  $($unk_tt:tt)*
	] => {
		crate::escseq_data_element! {
			@[$($n $(, $n2)* )?][$type_t][ $([ $($add_data)* ])* ]: [$($tt)*]
		}
		crate::escseq_data! {
			$($unk_tt)*
		}
	};
	
	[ ] => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! escseq_data_element {
	// pub name = t;
	[ @[$($for_trait:ident),*][$($ttype:tt)*][$($add_ttype:tt)*]: [
		$( #[$($add_data:tt)*] )*
		$(pub $(( $pub_arg:tt ))? )+ $n:ident $(or @$n2:ident)* $(or $n3:ident)*  = $($str:expr),*; 	$($tt:tt)*
	] ] => {
		$crate::__escseq_data_element! {
			@[$($for_trait),*][$($ttype)*][$($add_ttype)* $([ $($add_data)* ])* ]: {
				[$(pub $(( $pub_arg ))?)+] [$n $(, @$n2)* $(, $n3)*] [$($str),*] 
			}
		}
		
		crate::escseq_data_element! {
			@[$($for_trait),*][$($ttype)*][$($add_ttype)*]: [$($tt)*]
		}
	};
	
	[ @[$($tt:tt)*][$($ttype:tt)*][$($add_ttype:tt)*]: []] => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __escseq_data_element {
	// or name
	[ @[$($for_trait:tt)*][$($ttype:tt)*][$($add_ttype:tt)*]: { [ $(pub $(( $pub_arg:tt ))? )+ ] [$n:ident , $n2:ident $($all_n:tt)*] [$($all_str:tt)*] }  $($tt:tt)* ] => {
		$crate::__escseq_data_element! {
			@[$($for_trait)*][$($ttype)*][$($add_ttype)*]: {
				[ $(pub $(( $pub_arg ))? )+ ] [$n $($all_n)* ] [$($all_str)*]
			}
			
			$($tt)*
		}
		
		$(pub $(($pub_arg))? )+ type $n2 = $n;
	};
	
	// or @test
	[ @[$($for_trait:tt)*][$($ttype:tt)*][$($add_ttype:tt)*]: { [ $(pub $(( $pub_arg:tt ))? )+ ] [$n:ident , @$n2:ident $($all_n:tt)*] [$($all_str:tt)*] } $($tt:tt)* ] => {
		$crate::__escseq_data_element! {
			@[$($for_trait)*][$($ttype)*][$($add_ttype)*]: {	[ $(pub $(( $pub_arg ))? )+ ] [$n2] [$($all_str)*]	}
			@[$($for_trait)*][$($ttype)*][$($add_ttype)*]: {	[ $(pub $(( $pub_arg ))? )+ ] [$n $($all_n)* ] [$($all_str)*]	}
			
			$($tt)*
		}
	};
	
	
	
	[ @[$($for_trait:ident),*][$($ttype:tt)*][$($add_ttype:tt)*]: {
		[ $(pub $(( $pub_arg:tt ))? )+ ] [$n:ident] [$($str:expr),*]
	} $($tt:tt)* ] => {
		$(pub $(($pub_arg))? )? enum $n {}
		
		impl $crate::data::EscSeqData for $n {
			$crate::cluConstData::const_data! {
				const ESC_DATA: &'static str = "\x1b[", $($str),* , "m";
				const HEAD_DATA: &'static str = $($str),*;
			}
		}
		crate::__escseq_data_element_decode_type! {
			@$($ttype)* [$n] {
				$($add_ttype)*
			}
		}
		
		$( impl $for_trait for $n {} )*
		
		$crate::__escseq_data_element! {
			$($tt)*
		}
	};
	
	[] => {}
}


#[doc(hidden)]
#[macro_export]
macro_rules! __escseq_data_element_decode_type {
	[@object[$n:ident] {}] => {
		impl $crate::elements::objects::EscObject for $n {}
	};
	[@position[$n:ident] {}] => {
		impl $crate::elements::position::EscPosition for $n {}
	};
	
	[@attr[$n:ident] {}] => {
		macro_tt_utils::throw_compile_error!(@root "Empty arguments, for 'attr' you need to specify '# [reset (...)]'");
	};
	[@attr[$n:ident] { [reset($($reset_path:tt)*)] }] => {
		impl $crate::elements::attributes::EscAttr for $n {
			type ResetAllAttr = crate::elements::reset::ResetAllAttr;
			type ResetAttr = $($reset_path)*;
		}
	};
	[@attr[$n:ident] {$($all_tt:tt)*}] => {
		macro_tt_utils::throw_compile_error!(@root "Undefined arguments, for 'attr' you need to specify '# [reset (...)]'");
	};
}

