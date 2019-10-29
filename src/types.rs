use std::fmt::{Display, Formatter, Result};

use serde_derive::{Deserialize, Serialize};

pub type PlayerIndex = usize;
pub type PawnIndex = usize;
pub type PathPosition = u8;
pub type Score = i16;
pub type CardPile = Vec<Card>;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Color {
	Yellow,
	Green,
	White,
	Red,
	Blue,
}
impl Display for Color {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let string = match self {
			Color::Yellow => "Yellow",
			Color::Green => "Green",
			Color::White => "White",
			Color::Red => "Red",
			Color::Blue => "Blue",
		};
		f.write_str(string)
	}
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct Card {
	pub color: Color,
	pub number: i8,
}
impl Card {
	pub fn new(color: Color, number: i8) -> Card {
		Card { color, number }
	}
}
impl Display for Card {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Card({} {})", self.color, self.number)
	}
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum PawnType {
	Adventurer,
	Researcher,
}
impl Display for PawnType {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let string = match self {
			PawnType::Adventurer => "Adventurer",
			PawnType::Researcher => "Researcher",
		};
		f.write_str(string)
	}
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Event {
	Points(Score),
	Artifact,
	Arrow,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Action {
	PlayCard(Card),
	DiscardCard(Card),
	DrawFromDeck,
	DrawFromDiscard(Color),
	ChoosePawnType(PawnType),
	MoveColor(Color),
}
impl Display for Action {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let string = match self {
			Action::PlayCard(card) => {
				format!("PlayCard({} {})", card.color, card.number)
			}
			Action::DiscardCard(card) => {
				format!("DiscardCard({} {})", card.color, card.number)
			}
			Action::DrawFromDeck => String::from("DrawFromDeck"),
			Action::DrawFromDiscard(color) => {
				format!("DrawFromDiscard({})", color)
			}
			Action::ChoosePawnType(pawn_type) => {
				format!("ChoosePawnType({})", pawn_type)
			}
			Action::MoveColor(color) => format!("MoveColor({})", color),
		};
		f.write_str(&*string)
	}
}
