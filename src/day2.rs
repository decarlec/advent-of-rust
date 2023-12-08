use std::{fs::File, io::BufReader, io::prelude::*, str::FromStr};

struct Game {
    num: u32,
    max_red : u32,
    max_blue : u32,
    max_green:  u32
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32)
}

impl Color {
    fn is_valid(&self) -> bool {
        match self {
            Color::Red(a) => a <= &12,
            Color::Blue(a) => a <= &14,
            Color::Green(a) => a <= &13,
        }

    } 
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split_once(' ').unwrap() {
            (n, "red") => Ok(Color::Red(u32::from_str(n).unwrap())),
            (n, "blue") => Ok(Color::Blue(u32::from_str(n).unwrap())),
            (n, "green") => Ok(Color::Green(u32::from_str(n).unwrap())),
            _ => Err("no good color".to_string())
        }
    }
}

pub fn pt2(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let mut result : u32 = 0;
    for line in lines_iter {
        let mut game = 
            Game { 
                num: u32::from_str(line.split_at(4).1.split_once(':').unwrap().0.trim()).unwrap(),
                max_red: 0,
                max_blue: 0,
                max_green: 0
            };
        let rounds : Vec<&str>  = line.split_once(':').unwrap().1.split(';').collect();
        for round in rounds.iter() {
            let sets = round.split(',').into_iter();         
            for set in sets {
                let color = Color::from_str(set).unwrap(); 
                match color {
                    Color::Red(c) => { game.max_red = std::cmp::max(game.max_red,c)},
                    Color::Green(c) => { game.max_green = std::cmp::max(game.max_green,c)},
                    Color::Blue(c) => { game.max_blue = std::cmp::max(game.max_blue,c)}  
                }
            }
        }
        result += game.max_blue*game.max_green*game.max_red;
    } 

    println!("The result is {}.", result);
}
