#[macro_use]
extern crate lazy_static;
extern crate rand;

mod constants;
mod state;
mod types;

use rand::XorShiftRng;
use rand::{FromEntropy, Rng};

use state::GameState;

fn main() {
	let mut rng = XorShiftRng::from_entropy();
	let mut state = GameState::new(2);
	while !state.is_finished {
		println!("player {}", state.current_player);

		let actions = state.available_actions().clone();
		println!("  turn state: {}", state.turn_state);
		println!(
			"  available actions: {}",
			if actions.is_empty() { "none" } else { "" }
		);
		for action in actions.clone() {
			println!("    {}", action);
		}
		let action = rng.choose(&actions).unwrap().clone();
		println!("  action: {}", action);
		println!();
		state.do_action(action);
	}

	if state.winner.is_some() {
		let winner = state.winner.unwrap();
		println!("winner: {}", winner,);
	} else {
		println!("no winner!");
	}
	for (index, score) in state.player_scores.iter().enumerate() {
		println!("player {} with {} points", index, score);
	}
}
