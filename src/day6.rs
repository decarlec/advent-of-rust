use std::{iter::zip, fs::File, io::BufReader, io::prelude::*};

struct Race {
    time: usize,
    record: usize
}

impl Race {
    fn winning_ways(&self) -> usize {
        let mut count = 0;
        for hold in 0..=self.time {
            if hold*(self.time-hold) > self.record {
                count += 1;
            }
        }
        count
    }
}

pub fn pt1(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());
    
    let mut time: usize = 0;
    let mut distance: usize = 0;

    for line in lines_iter {
        let (name, items) = line.split_once(':').unwrap();
        match name {
            "Time" => {
                let mut timestr = items.to_string();
                timestr.retain(|c| !c.is_whitespace());
                time = timestr.parse::<usize>().unwrap();            
            println!("{}",time);
            },
            "Distance" => {
                let mut disstr = items.to_string();
                disstr.retain(|c| !c.is_whitespace());
                distance = disstr.parse::<usize>().unwrap();                         
                println!("{}",distance);
                //distances = items.to_string().retain(|c| !c.is_whitespace()).map(|s| s.parse::<usize>().unwrap()).collect();
            }
            _ => panic!("No!")
        }
    }

    //let races = zip(time, distances).map(|(t, d)| Race{ time: t, record: d }); 
    let result = Race { time, record: distance }.winning_ways();
    //let result : usize = races.map(|r| r.winning_ways()).product();


    println!("The result is {}.", result);
}

