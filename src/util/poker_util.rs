use std::ops::Deref;
use std::rc::Rc;
use crate::entity::card::PokerCard;

pub fn to_string_decks(decks : Vec<impl Deref<Target = PokerCard>>) -> String {
    let mut result = String::new();
    for (index, card) in decks.iter().enumerate() {
        result.push_str(card.to_string().as_str());
        if index != (decks.len() - 1) {
            result.push_str(", ");
        }
    }
    result
}