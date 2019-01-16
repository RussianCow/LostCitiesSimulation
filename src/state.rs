use std::fmt::{Display, Formatter, Result};

use rand::prng::XorShiftRng;
use rand::{Rng, SeedableRng};

use constants::COLORS;
use types::*;

#[derive(Clone, PartialEq)]
pub enum TurnState {
	PlayingCard,
	DrawingCard,
	PlayingPawn(Color),
	MovingPawn,
	GameFinished,
}
impl Display for TurnState {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let string = match self {
			TurnState::PlayingCard => String::from("PlayingCard"),
			TurnState::DrawingCard => String::from("DrawingCard"),
			TurnState::PlayingPawn(pawn) => format!("PlayingPawn({})", pawn),
			TurnState::MovingPawn => String::from("MovingPawn"),
			TurnState::GameFinished => String::from("GameFinished"),
		};
		f.write_str(&string)
	}
}

fn create_deck(_is_short: bool) -> CardPile {
	let mut cards: Vec<Card> = Vec::new();
	COLORS.iter().for_each(|color| {
		for number in 0..=10 {
			for _ in 0..2 {
				let card = Card::new(*color, number);
				cards.push(card);
			}
		}
	});

	// Shuffle
	let mut rng = XorShiftRng::from_seed([0; 16]);
	rng.shuffle(&mut cards);

	cards
}

fn create_events() -> Vec<Event> {
	let mut events: Vec<Event> = Vec::new();
	events.extend((0..9).map(|_| Event::Artifact));
	events.extend((0..9).map(|_| Event::Arrow));
	events.extend((0..2).map(|_| Event::Points(5)));
	events.extend((0..3).map(|_| Event::Points(10)));
	events.extend((0..2).map(|_| Event::Points(15)));
	events
}

pub struct GameState {
	// TODO: Turn some of these vectors into statically sized slices.
	pub player_hands: Vec<CardPile>,
	pub player_scores: Vec<Score>,
	pub player_artifacts: Vec<u8>,
	pub player_color_expeditions: Vec<Vec<CardPile>>,
	pub player_pawns: Vec<Vec<PawnIndex>>,

	pub pawn_types: Vec<PawnType>,
	pub pawn_colors: Vec<Option<Color>>,
	pub pawn_positions: Vec<Option<PathPosition>>,

	pub color_position_events: Vec<Vec<Option<Event>>>,
	pub color_discard_piles: Vec<CardPile>,

	pub deck: CardPile,
	pub turn_state: TurnState,
	pub current_player: PlayerIndex,
	pub winner: Option<PlayerIndex>,
	// TODO: Implement long (3-round) games.
}
impl GameState {
	pub fn new(num_players: i8) -> GameState {
		if num_players > 4 || num_players < 2 {
			panic!("Games can only have between 2 and 4 players.");
		}

		let player_indexes = 0..num_players;

		let mut deck = create_deck(num_players == 2);

		let player_hands = player_indexes
			.clone()
			.map(|_| (0..8).map(|_| deck.pop().unwrap()).collect())
			.collect();
		let player_scores: Vec<Score> =
			player_indexes.clone().map(|_| 0).collect();
		let player_artifacts = player_scores.iter().map(|x| *x as u8).collect();
		let player_color_expeditions: Vec<Vec<CardPile>> = player_indexes
			.clone()
			.map(|_| COLORS.iter().map(|_| CardPile::new()).collect())
			.collect();

		let pawn_indexes = 0..(num_players * 5);
		let mut pawn_types: Vec<PawnType> = Vec::new();
		let mut player_pawns: Vec<Vec<PawnIndex>> = Vec::new();
		let pawn_colors: Vec<Option<Color>> =
			pawn_indexes.clone().map(|_| None).collect();
		let pawn_positions: Vec<Option<PathPosition>> =
			pawn_indexes.clone().map(|_| None).collect();
		for player_index in player_indexes {
			let mut pawns: Vec<PawnIndex> = Vec::new();
			for pawn_index in 0..5 {
				let pawn_type = if pawn_index == 4 {
					PawnType::Researcher
				} else {
					PawnType::Adventurer
				};
				pawn_types.push(pawn_type);
				pawns.push((player_index * 5 + pawn_index) as PawnIndex);
			}
			player_pawns.push(pawns);
		}

		let mut events = create_events();
		let color_discard_piles: Vec<CardPile> =
			COLORS.iter().map(|_| CardPile::new()).collect();
		let color_position_events: Vec<Vec<Option<Event>>> = vec![
			vec![
				None,
				events.pop(),
				events.pop(),
				None,
				events.pop(),
				None,
				events.pop(),
				None,
				events.pop(),
			],
			vec![
				None,
				events.pop(),
				None,
				events.pop(),
				events.pop(),
				None,
				events.pop(),
				None,
				events.pop(),
			],
			vec![
				None,
				None,
				events.pop(),
				events.pop(),
				None,
				events.pop(),
				events.pop(),
				None,
				events.pop(),
			],
			vec![
				None,
				None,
				events.pop(),
				None,
				events.pop(),
				events.pop(),
				events.pop(),
				None,
				events.pop(),
			],
			vec![
				None,
				events.pop(),
				None,
				events.pop(),
				None,
				events.pop(),
				events.pop(),
				None,
				events.pop(),
			],
		];

		GameState {
			player_hands,
			player_scores,
			player_artifacts,
			player_color_expeditions,
			player_pawns,

			pawn_types,
			pawn_colors,
			pawn_positions,

			color_discard_piles,
			color_position_events,

			deck,
			turn_state: TurnState::PlayingCard,
			current_player: 0,
			winner: None,
		}
	}

	pub fn is_finished(&self) -> bool {
		self.turn_state == TurnState::GameFinished
	}

	pub fn available_actions(&self) -> Vec<Action> {
		let player = self.current_player;
		match self.turn_state {
			TurnState::PlayingCard => {
				let mut actions: Vec<Action> = Vec::new();
				actions.extend(
					self.player_hands[player]
						.iter()
						.filter(|card| {
							self.can_play_card(player, (*card).clone())
						})
						.map(|card| Action::PlayCard(card.clone())),
				);
				actions.extend(
					self.player_hands[player]
						.iter()
						.map(|card| Action::DiscardCard(card.clone())),
				);
				actions
			}
			TurnState::DrawingCard => {
				let mut actions: Vec<Action> = vec![Action::DrawFromDeck];
				actions.extend(
					COLORS
						.iter()
						.filter(|color| {
							!self.color_discard_piles[**color as usize]
								.is_empty()
						})
						.map(|color| Action::DrawFromDiscard(color.clone())),
				);
				actions
			}
			TurnState::PlayingPawn(_) => self.player_pawns[player]
				.iter()
				.filter(|pawn| self.pawn_colors[**pawn].is_none())
				.map(|pawn| Action::ChoosePawnType(self.pawn_types[*pawn]))
				.collect(),
			TurnState::MovingPawn => self.player_pawns[player]
				.iter()
				.filter(|pawn| self.can_move_pawn(**pawn))
				.map(|pawn| Action::MoveColor(self.pawn_colors[*pawn].unwrap()))
				.collect(),
			TurnState::GameFinished => Vec::new(),
		}
	}

	pub fn do_action(&mut self, action: Action) {
		if self.is_finished() {
			panic!("Cannot do_action when the game has already finished.");
		}

		let player = self.current_player;
		self.turn_state = match action {
			Action::PlayCard(card) => self.play_card(player, card),
			Action::DiscardCard(card) => self.discard_card(player, card),
			Action::DrawFromDeck => self.draw_from_deck(player),
			Action::DrawFromDiscard(color) => {
				self.draw_from_discard(player, color)
			}
			Action::ChoosePawnType(pawn_type) => {
				self.choose_pawn_type(player, pawn_type)
			}
			Action::MoveColor(color) => self.move_color(player, color),
		}
	}

	fn can_play_card(&self, player: PlayerIndex, card: Card) -> bool {
		let maybe_last_card =
			self.player_color_expeditions[player][card.color as usize].last();
		match maybe_last_card {
			Some(last_card) => last_card.number <= card.number,
			None => true,
		}
	}

	fn can_move_pawn(&self, pawn: PawnIndex) -> bool {
		let position = self.pawn_positions[pawn];
		position.is_some() && position != Some(8)
	}

	fn player_can_move_any(&self, player: PlayerIndex) -> bool {
		self.player_pawns[player]
			.iter()
			.filter(|pawn| self.can_move_pawn(**pawn))
			.next()
			.is_some()
	}

	fn play_card(&mut self, player: PlayerIndex, card: Card) -> TurnState {
		if !self.can_play_card(player, card) {
			panic!("Can't play card: {}", card);
		}
		if self.player_hands[player].len() != 8 {
			panic!("Incorrect hand size: {}", self.player_hands[player].len());
		}
		self.remove_card_from_hand(player, card);
		let is_expedition_empty = self.player_color_expeditions[player]
			[card.color as usize]
			.is_empty();
		self.player_color_expeditions[player][card.color as usize].push(card);
		if is_expedition_empty {
			TurnState::PlayingPawn(card.color)
		} else {
			self.move_color(player, card.color)
		}
	}

	fn discard_card(&mut self, player: PlayerIndex, card: Card) -> TurnState {
		self.remove_card_from_hand(player, card);
		self.color_discard_piles[card.color as usize].push(card);
		TurnState::DrawingCard
	}

	fn draw_from_deck(&mut self, player: PlayerIndex) -> TurnState {
		self.player_hands[player].push(self.deck.pop().unwrap());
		if self.deck.is_empty() {
			self.end_game()
		} else {
			self.end_turn()
		}
	}

	fn draw_from_discard(
		&mut self,
		player: PlayerIndex,
		color: Color,
	) -> TurnState {
		self.player_hands[player].push(
			self.color_discard_piles[color as usize]
				.pop()
				.expect(&format!(
                "Attempting to draw from an empty discard pile of color {}!",
                color
            )),
		);
		self.end_turn()
	}

	fn choose_pawn_type(
		&mut self,
		player: PlayerIndex,
		pawn_type: PawnType,
	) -> TurnState {
		let color = match self.turn_state {
			TurnState::PlayingPawn(color) => color,
			_ => panic!("Invalid turn state for choosing a pawn."),
		};
		let pawns = self.player_pawns[player].clone();
		let pawn = *pawns
			.iter()
			.filter(|pawn| {
				self.pawn_types[**pawn] == pawn_type
					&& self.pawn_colors[**pawn].is_none()
			})
			.nth(0)
			.unwrap();
		self.pawn_colors[pawn] = Some(color);
		self.move_color(player, color)
	}

	fn move_color(&mut self, player: PlayerIndex, color: Color) -> TurnState {
		let pawns = self.player_pawns[player].clone();
		let pawn = *pawns
			.iter()
			.find(|pawn| self.pawn_colors[**pawn] == Some(color))
			.expect(&format!("No pawn at {} exists for the player.", color));
		let current_position = self.pawn_positions[pawn];
		if current_position == Some(8) {
			return TurnState::MovingPawn;
		}
		let new_position = match current_position {
			Some(position) => position + 1,
			// If it's None, that means we just played the pawn but haven't
			// moved it yet.
			None => 0,
		};
		self.pawn_positions[pawn] = Some(new_position);
		let event =
			self.color_position_events[color as usize][new_position as usize];
		match event {
			Some(event) => match event {
				Event::Points(points) => {
					self.player_scores[player] += points;
					self.end_turn()
				}
				Event::Artifact => {
					self.player_artifacts[player] += 1;
					self.end_turn()
				}
				Event::Arrow => {
					if self.player_can_move_any(player) {
						TurnState::MovingPawn
					} else {
						self.end_turn()
					}
				}
			},
			None => self.end_turn(),
		}

		// TODO: End the game when 5 pawns have crossed the bridges.
	}

	fn remove_card_from_hand(&mut self, player: PlayerIndex, card: Card) {
		let hand = &mut self.player_hands[player];
		let index = hand.iter().position(|player_card| *player_card == card);
		hand.remove(index.unwrap());
	}

	fn player_count(&self) -> usize {
		self.player_scores.len()
	}

	fn end_turn(&mut self) -> TurnState {
		let player = self.current_player;
		if self.player_hands[player].len() < 8 {
			return TurnState::DrawingCard;
		}

		self.current_player = if self.current_player == self.player_count() - 1
		{
			0 as PlayerIndex
		} else {
			self.current_player + 1
		};
		TurnState::PlayingCard
	}

	fn end_game(&mut self) -> TurnState {
		self.calculate_scores();

		let max_score =
			self.player_scores.iter().fold(0, |max_score, score| {
				if *score > max_score {
					*score
				} else {
					max_score
				}
			});
		let potential_winner1: Option<PlayerIndex> = self
			.player_scores
			.iter()
			.position(|score| *score == max_score);
		let potential_winner2: Option<PlayerIndex> = self
			.player_scores
			.iter()
			.rposition(|score| *score == max_score);
		self.winner = if potential_winner1.is_some()
			&& potential_winner1 == potential_winner2
		{
			potential_winner1
		} else {
			None
		};

		TurnState::GameFinished
	}

	fn calculate_scores(&mut self) {
		for player in 0..self.player_count() {
			self.player_scores[player] +=
				score_for_artifact_count(self.player_artifacts[player]);
			for pawn in self.player_pawns[player].clone() {
				let position = self.pawn_positions[pawn];
				if position.is_some() {
					self.player_scores[player] +=
						score_for_position(position.unwrap());
				}
			}
		}
	}
}

fn score_for_artifact_count(count: u8) -> Score {
	match count {
		0 => -20,
		1 => -15,
		2 => -10,
		3 => 15,
		4 => 30,
		_ => 50,
	}
}

fn score_for_position(position: PathPosition) -> Score {
	match position {
		0 => -20,
		1 => -15,
		2 => -10,
		3 => 5,
		4 => 10,
		5 => 15,
		6 => 30,
		7 => 35,
		8 => 50,
		_ => panic!("Invalid position {}", position),
	}
}
