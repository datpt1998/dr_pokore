use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::entity::card;
use crate::entity::card::{PokerCard, Type};
use crate::service::poker_service::Flush::{RoyalFlush, StraightFlush};
use crate::service::poker_service::Normal::{HighCard, Straight};
use crate::service::poker_service::SameType::{FourOfAKind, FullHouse, Pair, ThreeOfAKind, TwoPair};
use std::string::ToString;
use strum_macros::Display;
// use rand::Rng;

pub struct Match {
    deck : Vec<PokerCard>
}

#[derive(Display, Debug)]
enum Flush {
    RoyalFlush,
    StraightFlush,
    Flush
}

#[derive(Display, Debug)]
enum SameType {
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair
}

#[derive(Display, Debug)]
enum Normal {
    Straight,
    HighCard
}

impl Match {
    pub fn new() -> Self{
        Match {
            deck : PokerCard::get_all_card()
        }
    }

    pub fn draw_player(&mut self) -> Result<Vec<PokerCard>, &str> {
        self.draw(2)
    }

    pub fn draw_general(&mut self) -> Result<Vec<PokerCard>, &str> {
        self.draw(5)
    }

    fn draw(&mut self, draw_number : i16) -> Result<Vec<PokerCard>, &str> {
        if draw_number as usize > self.deck.len() {
            return Err("Exceed deck number");
        }
        let mut rng = rand::thread_rng();
        let mut result : Vec<PokerCard> = Vec::new();
        for i in 0..draw_number {
            result.push(self.each_draw(&mut rng));
        }
        Ok(result)
    }

    fn each_draw(&mut self, rng: &mut ThreadRng) -> PokerCard {
        let index = rng.gen_range(0..self.deck.len());
        self.deck.remove(index)
    }
}

type PokerScore = [i16;18];

pub fn compare_score(s1 : &PokerScore, s2 : &PokerScore) -> Ordering {
    for i in 0..s1.len() {
        if s1[i] > s2[i] {
            return Ordering::Greater;
        } else if s1[i] < s2[i] {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

pub fn score_by_one_deck<'a>(deck: &Vec<Rc<PokerCard>>) -> Result<(PokerScore, [Rc<PokerCard>; 5], String), &'a str> {
    score(deck)
}

pub fn score_by_two_deck<'a>(player_deck : &Vec<Rc<PokerCard>>, general_deck: &Vec<Rc<PokerCard>>) -> Result<(PokerScore, [Rc<PokerCard>; 5], String), &'a str> {
    let deck = get_full_deck(vec!(player_deck, general_deck));
    score(&deck)
}

fn score<'a>(deck: &Vec<Rc<PokerCard>>) -> Result<(PokerScore, [Rc<PokerCard>; 5], String), &'a str> {
    let mut number_map : HashMap<i16, Vec<Rc<PokerCard>>> = HashMap::new();
    let mut type_map : HashMap<card::Type, Vec<Rc<PokerCard>>> = HashMap::new();
    let mut deck_set = deck.iter().map(|card| Rc::clone(card)).collect::<HashSet<_>>();
    for card in deck.iter() {
        push_value_to_map_vec(&mut type_map, card.get_card_type(), Rc::clone(card));
        if card.get_number() == 1 {
            let big_a = Rc::new(PokerCard::new_by_attribute(card.get_card_type(), 14));
            deck_set.insert(Rc::clone(&big_a));
            push_value_to_map_vec(&mut number_map, 14, Rc::clone(card));
            push_value_to_map_vec(&mut type_map, card.get_card_type(), Rc::clone(&big_a));
        } else {
            push_value_to_map_vec(&mut number_map, card.get_number(), Rc::clone(card));
        }
    }
    inner_score(&number_map, &mut type_map, &deck_set)
}

fn inner_score<'a>(number_map : &HashMap<i16, Vec<Rc<PokerCard>>>,
               type_map : &mut HashMap<card::Type, Vec<Rc<PokerCard>>>,
               deck_set : &HashSet<Rc<PokerCard>>) -> Result<(PokerScore, [Rc<PokerCard>; 5], String), &'a str>{
    let mut result = [0;18];

    let flush_result = flush(type_map);

    if let Some((flush_type, chosen)) = &flush_result {
        if let RoyalFlush | StraightFlush = flush_type {
            result[0] = chosen[0].get_number();
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, flush_type.to_string()));
        }
    }

    let same_type_result = four_three_two_case(number_map);

    if let Some((same_type_type, chosen)) = &same_type_result {
        if let FourOfAKind = same_type_type {
            result[1] = chosen[0].get_number();
            result[13] = chosen[4].get_number();
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, same_type_type.to_string()));
        } else if let FullHouse = same_type_type {
            result[2] = chosen[0].get_number();
            result[3] = chosen[3].get_number();
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, same_type_type.to_string()));
        }
    }

    if let Some((flush_type, chosen)) = &flush_result {
        if let Flush::Flush = flush_type {
            for i in 0..5 {
                result[i + 4] = chosen[i].get_number();
            }
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, flush_type.to_string()));
        }
    }

    let other_result = straight_and_other(deck_set);

    if let Some((other_type, chosen)) = &other_result {
        if let Straight = other_type {
            result[9] = chosen[0].get_number();
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, other_type.to_string()));
        }
    }

    if let Some((same_type_type, chosen)) = &same_type_result {
        if let ThreeOfAKind = same_type_type {
            result[10] = chosen[0].get_number();
            result[13] = chosen[3].get_number();
            result[14] = chosen[4].get_number();
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, same_type_type.to_string()));
        } else if let TwoPair = same_type_type {
            result[11] = chosen[0].get_number();
            result[12] = chosen[2].get_number();
            result[13] = chosen[4].get_number();
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, same_type_type.to_string()));
        } else if let Pair = same_type_type {
            result[12] = chosen[0].get_number();
            result[13] = chosen[2].get_number();
            result[14] = chosen[3].get_number();
            result[15] = chosen[4].get_number();
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, same_type_type.to_string()));
        }
    }

    if let Some((other_type, chosen)) = &other_result {
        if let HighCard = other_type {
            for i in 0..5 {
                result[i + 13] = chosen[i].get_number();
            }
            let mut chosen_copy : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&chosen[0]));
            for i in 0..chosen.len() {
                chosen_copy[i] = Rc::clone(&chosen[i]);
            }
            return Ok((result, chosen_copy, other_type.to_string()));
        }
    }
    Err("Non supported case")
}

fn straight_and_other(deck_set : &HashSet<Rc<PokerCard>>) -> Option<(Normal, [Rc<PokerCard>; 5])> {
    let mut sorted_vec = deck_set.iter().map(|card| Rc::clone(card)).collect::<Vec<_>>();
    sorted_vec.sort();
    if let Some(biggest) =  biggest_sequence(&mut sorted_vec) {
        let mut result : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&biggest[0]));
        for i in 0..5 {
            result[i] = Rc::clone(&biggest[4 - i]);
        }
        return Some((Straight, result));
    }
    sorted_vec.reverse();
    let mut result : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(&sorted_vec[0]));
    for i in 0..5 {
        result[i] = Rc::clone(&sorted_vec[i]);
    }
    Some((HighCard, result))
}

//four-of-a-kind, full-house, three-of-a-kind, two-pair, pair
fn four_three_two_case(number_map : &HashMap<i16, Vec<Rc<PokerCard>>>) -> Option<(SameType, [Rc<PokerCard>; 5])> {
    let mut is_four = false;
    let mut is_three = false;
    let mut is_two = false;
    let mut is_two_pair = false;
    let mut biggest_four = 0;
    let mut biggest_three = 0;
    let mut biggest_two_1 = 0;
    let mut biggest_two_2 = 0;
    for (key, value) in number_map.iter() {
        let number = *key;
        match value.len() {
            4 => {
                is_four = true;
                if number > biggest_four {
                    biggest_four = number;
                }
            }
            3 => {
                is_three = true;
                if number > biggest_three {
                    biggest_three = number;
                }
            }
            2 => {
                if is_two {
                    if is_two_pair {
                        let mut v = vec![number, biggest_two_1, biggest_two_2];
                        v.sort();
                        v.reverse();
                        biggest_two_1 = v[0];
                        biggest_two_2 = v[1];
                    } else {
                        is_two_pair = true;
                        let mut v = vec![number, biggest_two_2];
                        v.sort();
                        v.reverse();
                        biggest_two_1 = v[0];
                        biggest_two_2 = v[1];
                    }
                } else {
                    is_two = true;
                    biggest_two_2 = number;
                }
            }
            _ => {/*do nothing*/}
        }
    }

    return if is_four {
        let chosen_number_vec = number_map.get(&biggest_four).unwrap();
        let mut result : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(chosen_number_vec.first().unwrap()));
        for i in 0..chosen_number_vec.len() {
            result[i] = Rc::clone(&chosen_number_vec[i]);
        }
        let next_chosen_number = number_map.keys().filter(|&&key| key != biggest_four).max().unwrap();
        result[4] = Rc::clone(number_map.get(next_chosen_number).unwrap().first().unwrap());
        Some((FourOfAKind, result))
    } else if is_three && is_two {
        let chosen_three_vec = number_map.get(&biggest_three).unwrap();
        let chosen_two_vec = number_map.get(&biggest_two_2).unwrap();
        let mut result : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(chosen_three_vec.first().unwrap()));
        for i in 0..chosen_three_vec.len() {
            result[i] = Rc::clone(&chosen_three_vec[i]);
        }
        for i in 0..chosen_two_vec.len() {
            result[i + 3] = Rc::clone(&chosen_two_vec[i]);
        }
        Some((FullHouse, result))
    } else if is_three {
        let chosen_three_vec = number_map.get(&biggest_three).unwrap();
        let mut result : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(chosen_three_vec.first().unwrap()));
        for i in 0..chosen_three_vec.len() {
            result[i] = Rc::clone(&chosen_three_vec[i]);
        }
        let mut find_max = number_map.keys().filter(|&&key| key != biggest_three).collect::<Vec<_>>();
        for i in 0..2 {
            let next_chosen_number = find_max.iter().max().unwrap();
            result[i + 3] = Rc::clone(number_map.get(next_chosen_number).unwrap().first().unwrap());
            find_max = find_max.iter().filter(|&&&key| key != **next_chosen_number).map(|&key| key).collect::<Vec<_>>();
        }
        Some((ThreeOfAKind, result))
    } else if is_two_pair {
        let chosen_vec_1 = number_map.get(&biggest_two_1).unwrap();
        let chosen_vec_2 = number_map.get(&biggest_two_2).unwrap();
        let mut result : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(chosen_vec_1.first().unwrap()));
        for i in 0..chosen_vec_1.len() {
            result[i] = Rc::clone(&chosen_vec_1[i]);
        }
        for i in 0..chosen_vec_2.len() {
            result[i + 2] = Rc::clone(&chosen_vec_2[i]);
        }
        let next_chosen_number = number_map.keys()
            .filter(|&&key| key != biggest_two_1 && key != biggest_two_2)
            .max()
            .unwrap();
        result[4] = Rc::clone(number_map.get(next_chosen_number).unwrap().first().unwrap());
        Some((TwoPair, result))
    } else if is_two {
        let chosen_vec = number_map.get(&biggest_two_2).unwrap();
        let mut result : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::clone(chosen_vec.first().unwrap()));
        for i in 0..chosen_vec.len() {
            result[i] = Rc::clone(&chosen_vec[i]);
        }
        let mut find_max = number_map.keys().filter(|&&key| key != biggest_two_2).collect::<Vec<_>>();
        for i in 0..3 {
            let next_chosen_number = find_max.iter().max().unwrap();
            result[i + 2] = Rc::clone(number_map.get(next_chosen_number).unwrap().first().unwrap());
            find_max = find_max.iter().filter(|&&&key| key != **next_chosen_number).map(|&key| key).collect::<Vec<_>>();
        }
        Some((Pair, result))
    } else {
        None
    }
}

//straight flush, royal flush (which is just special case of straight flush) and flush
fn flush(type_map : &mut HashMap<card::Type, Vec<Rc<PokerCard>>>) -> Option<(Flush, [Rc<PokerCard>; 5])> {
    let mut biggest_top = 0 as i16;
    let mut biggest_normal = 0 as i16;
    let placeholder = PokerCard::new_by_attribute(Type::SPADE, 1);
    let mut chosen_top : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::new(placeholder.get_copy()));
    let mut chosen_normal : [Rc<PokerCard>; 5] = array_init::array_init(|_| Rc::new(placeholder.get_copy()));

    for (key, value) in type_map.iter() {
        let number_vec = value.iter().map(|card| card.get_number()).collect::<Vec<_>>();
        let len = if number_vec.contains(&1) { value.len() - 1 } else { value.len() };
        if len >= 5 {
            let mut cards = value.iter().map(|card| Rc::clone(card)).collect::<Vec<_>>();
            cards.sort();
            let new_big = biggest_sequence(&mut cards);
            if let Some(chosen_ones) = new_big {
                if chosen_ones[0].get_number() > biggest_top {
                    biggest_top = chosen_ones[0].get_number();
                    for i in 0..5 {
                        chosen_top[i] = Rc::clone(&chosen_ones[4 - i]);
                    }
                }
            }
            cards.reverse();
            let new_normal = cards.get(0).unwrap().get_number();
            if new_normal > biggest_normal {
                biggest_normal = new_normal;
                for i in 0..5 {
                    chosen_normal[i] = Rc::clone(cards.get(i).unwrap());
                }
            }
        }
    }
    if biggest_top > 0 {
        if chosen_top[0].get_number() == 10 {
            Some((RoyalFlush, chosen_top))
        } else {
            Some((StraightFlush, chosen_top))
        }
    } else if biggest_normal > 0 {
        Some((Flush::Flush, chosen_normal))
    } else {
        None
    }
}



fn biggest_sequence(cards : &mut Vec<Rc<PokerCard>>) -> Option<[Rc<PokerCard>; 5]>{
    if cards.len() < 5 { return None }
    cards.sort();
    let mut next = cards.get(0).unwrap().get_number() + 1;
    let mut result : [Rc<PokerCard>; 5] = array_init::array_init(|_|Rc::clone(cards.get(0).unwrap()));
    let mut count = 1;
    let mut biggest = 0 as i16;
    for i in 1..cards.len() {
        let card = cards.get(i).unwrap();
        if card.get_number() == next {
            result[count] = Rc::clone(card);
            count = count + 1;
        } else {
            result[1] = Rc::clone(card);
            count = 1;
        }
        if count >= 5 {
            if card.get_number() > biggest {
                biggest = card.get_number();
            }
            count = 1;
        }
        next = card.get_number() + 1;
    }
    if biggest > 0 { Some(result) } else { None }
}

fn get_full_deck(decks : Vec<&Vec<Rc<PokerCard>>>) -> Vec<Rc<PokerCard>>{
    let mut result: Vec<Rc<PokerCard>> = Vec::new();
    for deck in decks.iter() {
        for card in deck.iter() {
            result.push(Rc::clone(card));
        }
    }
    return result;
}

fn push_value_to_map_vec<K: PartialEq + Eq + Hash, V>(map : &mut HashMap<K, Vec<Rc<V>>>, key : K, value: Rc<V>) {
    let vec_opt = map.remove(&key);
    if let Some(mut vec) = vec_opt {
        vec.push(value);
        map.insert(key, vec);
    } else {
        let mut vec : Vec<Rc<V>> = Vec::new();
        vec.push(value);
        map.insert(key, vec);
    }
}