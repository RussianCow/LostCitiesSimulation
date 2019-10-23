extern crate bincode;
extern crate chrono;
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate rand;
#[macro_use]
extern crate serde_derive;

mod constants;
mod game;
mod state;
mod types;

use std::fs;
use std::path::Path;

use bincode::serialize;
use chrono::{Utc};
use clap::{App, Arg};
use rand::{FromEntropy, XorShiftRng};

fn get_filename() -> String {
	let now = Utc::now();
	now.timestamp_millis().to_string() + ".game"
}

fn main() {
	let matches =
		App::new("Lost Cities Simulator")
		.version("0.0.1")
		.author("Sasha Chedygov <sasha@chedygov.com>")
		.about(
			"Simulates random matches of the board game Lost Cities, and saves those games to a file."
		)
		.arg(
			Arg::with_name("num_games")
			.short("n")
			.long("num_games")
			.value_name("n")
			.help("Determines how many games to simulate")
			.default_value("1")
		)
		.arg(
			Arg::with_name("out_dir")
			.short("o")
			.long("out_dir")
			.value_name("dir")
			.help("Where to save the game files")
			.default_value("out")
		)
		.get_matches();
	let num_games: u32 = matches.value_of("num_games").unwrap().parse().unwrap();
	let out_dir = Path::new(matches.value_of("out_dir").unwrap());
	fs::create_dir_all(out_dir).expect("Could not create output directory!");

	let mut rng = XorShiftRng::from_entropy();
	for _ in 0..num_games {
		let finished_game = game::play_game(&mut rng, 2);
		let out_path = out_dir.join(get_filename());
		fs::write(out_path, serialize(&finished_game).unwrap())
			.expect("Could not write");
	}
}
