use std::collections::HashMap;

use crate::card::{Card, Suit, self};

pub struct Hand<'a> {
    pub str_hand: &'a str,
    cards: Vec<Card>,
}

impl<'a> Hand<'a> {
    pub fn new(data: &'a str) -> Self {
        let hand = Hand {
            str_hand: data,
            cards: data.split(" ").map(|c| Card::new(c)).collect()
        };

        if hand.cards.len() != 5 {
            panic!("not enough cards");
        }

        return hand;
    }

    pub fn hand_rating(self: &Self) -> u32 {
        // todo: implement
        let mut values: HashMap<u8, u8> = HashMap::new();
        let mut suits: HashMap<&Suit, u8> = HashMap::new();
        for c in &self.cards {
            values.entry(c.value).and_modify(|c| *c += 1).or_insert(1);
            suits.entry(&c.suit).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut singles: Vec<u32> = vec![];
        let mut doubles: Vec<u32> = vec![];
        let mut triples: Vec<u32> = vec![];
        let mut quads: Vec<u32> = vec![];
        for v in &values {
            let card_value = *v.0 as u32;
            match v.1 {
                1 => singles.push(card_value),
                2 => doubles.push(card_value * 10),
                3 => triples.push(card_value * 100),
                4 => quads.push(card_value * 1_000),
                _ => continue,
            }
        }

        if suits.len() == 1 {
            if values.len() == 5 {
                // Straight Flush (5 x same suits, sequential values) | 90000
                // todo check for sequential
            }

            // Flush (5 x same suit) | 60000
        }


        // Four of a kind (4 x same value) | 80000
        if quads.len() == 1 && singles.len() == 1 {
            // todo give weight to quad
            return 80_000 + quads[0] + singles[0]
        }


        // Full House (3 x same value + 2 x same value) | 70000
        if triples.len() == 1 && doubles.len() == 1 {
            // todo give more weight to triple than double
            return 70_000 + triples[0] + doubles[0]
        }


        if values.len() == 5 {
            // Straight (sequential values) | 50000
            // todo check for sequencial

        }

        // Three of a kind (3 x same value) | 40000
        if triples.len() == 1 {
            return 40_000 + triples[0]
        }


        // Two pair (2 x same value + 2 x same value) | 30000
        if doubles.len() == 2 {
            return 30_000 + doubles[0] + doubles[1]
        }


        // One pair (2 x same value) | 2000
        if doubles.len() == 1 {
            return 20_000 + doubles[0]
        }


        // High card (highest card) | 1000
        return singles.iter().sum()
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_rating() == other.hand_rating()
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand_rating().partial_cmp(&other.hand_rating())
    }
}