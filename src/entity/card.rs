use std::cmp::Ordering;
use std::fmt::{Debug, format, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use Type::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Type {
    SPADE, CLUB, HEART, DIAMOND
}

#[derive(Debug, Eq, PartialEq)]
pub struct SolitaireCard {
    card_type : Type,
    number : i16
}

#[derive(Eq)]
pub struct PokerCard {
    card : SolitaireCard
}

impl SolitaireCard {
    pub fn new(card_type : Type, number : i16) -> Self {
        SolitaireCard{
            card_type,
            number
        }
    }

    pub fn get_all_card() -> Vec<SolitaireCard> {
        let mut result: Vec<SolitaireCard> = Vec::new();
        for card_type in Type::iter() {
            for i in 1..=13 {
                result.push(SolitaireCard::new(card_type, i));
            }
        }
        result
    }

    pub fn get_card_type(&self) -> Type {
        self.card_type
    }

    pub fn get_number(&self) -> i16 {
        self.number
    }
}

impl PokerCard {
    pub fn new_by_attribute(card_type : Type, number : i16) ->Self {
        PokerCard{
            card : SolitaireCard::new(card_type, number)
        }
    }

    pub fn new_by_solitaire_card(card : SolitaireCard) -> Self {
        PokerCard{
            card
        }
    }

    pub fn get_all_card() -> Vec<PokerCard>{
        let mut result:Vec<PokerCard> = Vec::new();
        for card in SolitaireCard::get_all_card() {
            result.push(PokerCard::new_by_solitaire_card(card));
        }
        result
    }

    pub fn get_card_type(&self) -> Type {
        self.card.card_type
    }

    pub fn get_number(&self) -> i16 {
        self.card.number
    }

    pub fn get_copy(&self) -> Self {
        Self::new_by_attribute(self.get_card_type(), self.get_number())
    }
}

impl PartialEq for PokerCard {
    fn eq(&self, other: &Self) -> bool {
        self.get_number() == other.get_number()
    }
}

impl Ord for PokerCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_number().cmp(&other.get_number())
    }
}

impl PartialOrd for PokerCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for PokerCard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_number().hash(state);
    }
}

impl Debug for PokerCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = match self.get_number() {
            1 | 14 => String::from("A"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            x => x.to_string()
        };
        f.write_str(&format!("{{{:?}, {:?}}}", number, self.get_card_type()))
    }
}

impl ToString for PokerCard {
    fn to_string(&self) -> String {
        let number = match self.get_number() {
            1 | 14 => String::from("A"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            x => x.to_string()
        };
        format!("{{{:?}, {:?}}}", number, self.get_card_type())
    }
}


