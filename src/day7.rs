use std::{fs::File, io::BufReader, io::prelude::*, cmp::Ordering};


struct Hand {
    cards: [char; 5],
    bid: usize,
    hand_type: HandType
}

#[derive(PartialEq, Copy, Clone)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

const CARDS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

impl Hand {
    // returns true if this hand is better than input
    fn greater_than(&self, hand: &Hand) -> Ordering
    {
        if self.hand_type == hand.hand_type {
            for (idx,this_card) in self.cards.iter().enumerate() {
                let that_card = &hand.cards[idx];

                let (this_idx, _) = CARDS.iter().enumerate().find(|c| c.1 == this_card).unwrap();
                let (that_idx, _) = CARDS.iter().enumerate().find(|c| c.1 == that_card).unwrap();
                if this_idx == that_idx { continue; }
                if this_idx < that_idx {
                    return Ordering::Greater;
                }
                else if this_idx > that_idx {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
        else if self.hand_type.index() < hand.hand_type.index() {
            return Ordering::Greater;
        }
        else if self.hand_type.index() > hand.hand_type.index() {
            return Ordering::Less;
        }
        Ordering::Equal
    }

}

fn get_hand_type(cards: [char;5]) -> HandType {
    let mut uniques: Vec<(char, usize)> = Vec::new(); 
    let mut jokers = 0;
    for card in cards {
        if card.eq(&'J') { jokers += 1; continue }
        if let Some(existing) = uniques.iter_mut().find(|c| c.0 == card) {
            existing.1 += 1; 
        }
        else {
            uniques.push((card, 1));
        }
    }

    
    println!("hand {}", cards.iter().collect::<String>());
    if jokers == 5 { return HandType::FiveOfKind; }
    let largest = uniques.iter().max_by(|a, b| { 
        if a.1 > b.1 { return Ordering::Greater;}
        if a.1 < b.1 { return Ordering::Less;}
        return Ordering::Equal;
    }).unwrap();

    match largest.1 + jokers {
        5 => return HandType::FiveOfKind,
        4 => return HandType::FourOfKind,
        3 => {
           if uniques.len() == 2 { return HandType::FullHouse }
           if uniques.len() == 3 { return HandType::ThreeOfKind }
            panic!("unexpected result on hand {}, largest:{}", cards.iter().collect::<String>(), largest.1);
        }
        2 => {
            if uniques.len() == 3 { return HandType::TwoPair }
            if uniques.len() == 4 { return HandType::OnePair }
            panic!("unexpected result on hand {}, largest:{}", cards.iter().collect::<String>(), largest.1);
        }
        _ => HandType::HighCard
        
    };

    match uniques.len(){
        1 => return HandType::FiveOfKind,
        2 => {
            if uniques.iter().find(|u| u.1 == 4).is_some() {
                return HandType::FourOfKind;
            }
            return HandType::FullHouse;
        },
        3 => {
            if uniques.iter().find(|u| u.1 == 3).is_some() {
                return HandType::ThreeOfKind;
            }
            return HandType::TwoPair;
        }
        4 => HandType::OnePair, 
        _ => HandType::HighCard
    }
}

pub fn pt1(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let mut hands: Vec<Hand> = Vec::new();
    let mut result: usize = 0;
    for line in lines_iter {
        let (cards, bid) = line.split_once(' ').unwrap();        
        let card_arr = cards.chars().collect::<Vec<char>>()[0..=4].try_into().unwrap();
        let hand = Hand { 
            cards: card_arr, 
            bid: bid.parse::<usize>().unwrap(),
            hand_type: get_hand_type(card_arr)
        };
        hands.push(hand);
    }
    hands.sort_by(|a, b| a.greater_than(b));

    for (idx, hand) in hands.iter().enumerate() {
        result += (idx+1)*hand.bid;
    }

    println!("The result is {}.", result);
}

