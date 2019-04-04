

#[macro_export]
macro_rules! maybeseg_data {
	[ $($tt:tt)*] => {
		
		
		
		$crate::maybeseg_data!{ $($tt)* }
	};
	
	() => ()
}