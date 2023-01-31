use std::cmp::Ordering;
// use std::collections::HashSet;
use std::rc::Rc;
// use entity::card::{PokerCard, SolitaireCard};
use crate::entity::card::Type;
use service::poker_service::{Match};
use crate::service::poker_service::{compare_score, score_by_two_deck};

mod entity;
mod service;

fn main() {
    println!("Hello, world!");

    let mut poker_match = Match::new();
    let deck_1 = poker_match.draw_player().unwrap().into_iter().map(|card| Rc::new(card)).collect::<Vec<_>>();
    let deck_2 = poker_match.draw_player().unwrap().into_iter().map(|card| Rc::new(card)).collect::<Vec<_>>();
    let deck_general = poker_match.draw_general().unwrap().into_iter().map(|card| Rc::new(card)).collect::<Vec<_>>();
    println!("Player 1: {:?}", deck_1);
    println!("Player 2: {:?}", deck_2);
    println!("General: {:?}", deck_general);

    let (score_1, chosen_1, type_1) = score_by_two_deck(&deck_1, &deck_general).unwrap();
    let (score_2, chosen_2, type_2) = score_by_two_deck(&deck_2, &deck_general).unwrap();

    match compare_score(&score_1, &score_2) {
        Ordering::Greater => {println!("Player 1 win with type {:?} and deck {:?} when player 2 have type {:?} and deck {:?}", type_1, chosen_1, type_2, chosen_2);}
        Ordering::Less => {println!("Player 2 win with type {:?} and deck {:?} when player 1 have type {:?} and deck {:?}", type_2, chosen_2, type_1, chosen_1);}
        Ordering::Equal => {println!("Draw, player 1 have type {:?} and deck {:?}, player 2 have type {:?} and deck {:?}", type_1, chosen_1, type_2, chosen_2);}
    }

    // let a = Rc::new(String::from("123"));
    // let rc1 = Rc::from(&a);
    // let rc2 = Rc::clone(&a);
    // println!("{:?}", *rc1);
    // println!("{:?}", *rc2);

    // let a : Vec<String> = vec![String::from("1"), String::from("2"), String::from("3")];
    // let a : Vec<Rc<String>> = a.iter().map(|s| Rc::new(String::from(s))).collect();
    // println!("{:?}", a);

    // let mut a = HashSet::from([Rc::new(PokerCard::new_by_attribute(Rc::new(Type::SPADE), 3)),
    //                  Rc::new(PokerCard::new_by_attribute(Rc::new(Type::HEART), 3)),
    //                  Rc::new(PokerCard::new_by_attribute(Rc::new(Type::DIAMOND), 4))]);
    // // a.sort();
    // println!("{:?}", a);
}
