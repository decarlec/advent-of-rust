use std::{fs::File, io::BufReader, io::prelude::*};
use std::str::FromStr;

struct Card {
    number: usize,
    winners: Vec<u8>,
    numbers: Vec<u8>
}

impl Card {
    fn score(&self) -> u32 {
        let mut mat: u32 = 0;
        for winner in self.winners.iter() {
            if self.numbers.contains(winner){
                mat += 1;
            }
        }
        return mat
    }
    
    fn get_card_score(&self, list: &Vec<Card>) -> usize {
        let mut count = 0;
        for winner in self.winners.iter() {
            if self.numbers.contains(winner){
                count += 1
            }
        }
        let mut score = 1;

        if count > 0 {
            let cards = &list[self.number..=self.number+count-1];
            for card in cards {
                score += card.get_card_score(list); 
            }
        }
        score
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.split_once(':').unwrap().0.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
        let winners = s.split_once(':').unwrap().1.split_once('|').unwrap().0.split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect();
        let numbers = s.split_once('|').unwrap().1.split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect();
        Ok(Card {
            number,
            winners,
            numbers
        })
    }
}

pub fn pt2(file_reader: BufReader<File>){
    let lines : Vec<String> = file_reader.lines().map(|l| l.unwrap()).collect();
    
    let mut result: usize = 0;

    let master = &lines.iter().map(|line| Card::from_str(&line).unwrap()).collect();
    for line in lines {
        let card = Card::from_str(&line).unwrap();
        result += card.get_card_score(master);
    }

    println!("The result is {}.", result);
}


