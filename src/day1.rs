use std::{fs::File, io::BufReader, io::prelude::*};

pub fn pt1(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let numeric = |i: char| i.is_numeric();

    let mut total: u32 = 0;
    for line in lines_iter {
        let first = line.chars().find(|x| numeric(*x)).unwrap();
        let last = line.chars().rev().find(|x| numeric(*x)).unwrap();
        total += String::from_iter([first, last]).parse::<u32>().unwrap(); 
    }

    println!("The total calibration value for part 1 is {}.", total);
}


pub fn pt2(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let numbers: [(char, &str); 9] = [('1', "one"), ('2',"two"), ('3',"three"), ('4',"four"), ('5',"five"), ('6',"six"), ('7', "seven"), ('8', "eight"), ('9', "nine")];
    let mut total: usize = 0;
    let mut first_char : Option<char> = None;
    let mut last_char : Option<char> = None;

    for line in lines_iter {
        let mut first_idx: usize = line.len();
        let mut last_idx: usize = 0;

        for (idx, char) in line.chars().enumerate(){
            for num in numbers {
                if num.0.eq(&char) {
                    if idx <= first_idx {
                        first_idx = idx;
                        first_char = Some(char);
                    }
                    if idx >= last_idx {
                        last_idx = idx;
                        last_char = Some(char);
                    }
                }
            }
        }
        for number in numbers {
            let mut s_idx: usize = 0;
            while let Some(idx) = &line[s_idx..].find(number.1){ 
                let real_idx = idx + s_idx;
                if real_idx <= first_idx {
                    first_idx = real_idx; 
                    first_char = Some(number.0);
                }
                if real_idx >= last_idx {
                    last_idx = real_idx;
                    last_char = Some(number.0);
                }
                s_idx = real_idx + 1; 
            } 
        }

        match (first_char, last_char) {
            (Some(a), Some(b)) => {
                total += String::from_iter([a,b]).parse::<usize>().unwrap();
            },
            _ => panic!("{} {}", first_char.is_some(), last_char.is_some())
        }
    }
    println!("The total calibration value for part 2 is {}.", total);
}
