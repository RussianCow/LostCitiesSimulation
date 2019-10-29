use lazy_static::lazy_static;

use crate::types::Color;

lazy_static! {
	pub static ref COLORS: Vec<Color> = vec![
		Color::Yellow,
		Color::Green,
		Color::White,
		Color::Red,
		Color::Blue,
	];
}
