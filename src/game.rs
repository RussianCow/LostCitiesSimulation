use rand::Rng;

use serde_derive::{Deserialize, Serialize};

use crate::state::GameState;
use crate::types::{Action, CardPile, Event};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct FinishedGame {
	pub starting_player_hands: Vec<CardPile>,
	pub deck: CardPile,
	pub color_position_events: Vec<Vec<Option<Event>>>,
	pub action_log: Vec<Action>,
}
impl FinishedGame {
	pub fn from_game_state(state: &GameState) -> FinishedGame {
		FinishedGame {
			starting_player_hands: (0..state.num_players)
				.map(|index| state.player_hands[index as usize].clone())
				.collect(),
			deck: state.deck.clone(),
			color_position_events: state.color_position_events.clone(),
			action_log: Vec::new(),
		}
	}
}

pub fn play_game<T: Rng>(rng: &mut T, num_players: u8) -> FinishedGame {
	let mut state = GameState::new(num_players);
	let mut finished_game = FinishedGame::from_game_state(&state);

	while !state.is_finished() {
		let available_actions = state.available_actions().clone();
		let action = rng.choose(&available_actions).unwrap().clone();
		state.do_action(action.clone());
		finished_game.action_log.push(action);
	}

	finished_game
}
