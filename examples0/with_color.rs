
extern crate ColorGeneric;
use ColorGeneric::colors::BrightBlue;
use ColorGeneric::colors::BrightRed;
use ColorGeneric::colors::BrightGreen;
use ColorGeneric::ColorGeneric;

pub fn main() {
	//Experimental possibility of using Rust Arguments for ColorGenerics.

	let mut item = Items::default();

	for a in 0..15 {
		BrightBlue::with_color_fmt(format_args!("ITEM #{:?}", item), |fmt| {
			println!("NUM #{}, {}; COUNT: {}", a, fmt, item.count());
		});
		
		item.0 += 1;
		item.1 += 2;
	}


	for a in 0..15 {
		BrightGreen::with_color_fmt(format_args!("NUM #{}", a), |fmt_num| {
			BrightBlue::with_color_fmt(format_args!("ITEM #{:?}", item), |fmt_item| {
				BrightRed::with_color_fmt(format_args!("ALL_COUNT {}", item.count()), |fmt_count| {
					println!("{}, {}; {};", fmt_num, fmt_item, fmt_count);
				});
			});
		});

		item.0 += 1;
		item.1 += 2;
	}
}

#[derive(Debug, Default)]
pub struct Items(usize, usize);

impl Items {
	#[inline]
	pub fn count(&self) -> usize {
		self.0 + self.1
	}
}