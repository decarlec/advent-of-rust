use std::{fs::File, io::BufReader, io::prelude::*};

type History = Vec<Vec<i64>>;

fn pop_histories(history: &mut History){
    println!("Populating history: {}", history.first().unwrap().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
    'pop: loop {
        let current = history.last().unwrap();
        let new: &mut Vec<i64> = &mut Vec::new();
        for i in 0..current.len() - 1 {
            new.push(current.get(i+1).unwrap()-current.get(i).unwrap());  
        }
        history.push(new.to_vec());
        println!("{}", new.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
        if new.iter().all(|a| a==&0_i64){ break 'pop; }
    }
}

fn process_history(history: &mut History) -> i64 {
        println!("Processing history: {}", history.first().unwrap().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
        
        let mut h_res: i64 = 0;

        for i in (1..history.len()-1).rev() {
            let a = history.get(i).unwrap().clone();
            let b : &mut Vec<i64> = history.get_mut(i-1).unwrap();
            let c = a.last().unwrap() + b.last().unwrap();
            println!("A: {}",  a.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
            println!("B: {}",  b.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
            b.push(c);
            if i == 1 { h_res = c; }
        }
        println!("Hres: {}\n", h_res);
        h_res
}

fn process_history_backwards(history: &mut History) -> i64 {
        println!("Populating history backwards: {}", history.first().unwrap().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
        let mut h_res: i64 = 0;

        println!("Processing history: {}", history.first().unwrap().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
        for i in (1..history.len()-1).rev() {
            let a = history.get(i).unwrap().clone();
            let b : &mut Vec<i64> = history.get_mut(i-1).unwrap();
            let c = b.first().unwrap() - a.first().unwrap();
            println!("A: {}",  a.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
            println!("B: {}",  b.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
            b.insert(0, c);
            if i == 1 { h_res = c; }
        }
        println!("Hres: {}\n", h_res);
        h_res
}

pub fn pt1(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let mut result: i64 = 0;

    let mut histories: Vec<History> = Vec::new();
    for line in lines_iter {
        let history = vec![line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect()];
        histories.push(history);
    }

    histories.iter_mut().for_each(|h| pop_histories(h));  

    //histories.iter_mut().for_each(|h| result += process_history(h));
    histories.iter_mut().for_each(|h| result += process_history_backwards(h));


    println!("The result is {}.", result);
}

