use std::{fs::File, io::BufReader, io::prelude::*, time::SystemTime, collections::HashMap};
use num::integer;

struct Mapping{
    left: String,
    right: String
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Path {
    val: String,
    last: char
}

type Mappings<'a> = HashMap<String, Mapping>;

pub fn pt1(file_reader: BufReader<File>){
    let start = SystemTime::now();
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let mut result: u64 = 0;
    let mut moves: Vec<char> = Vec::new(); 
    let mut mappings: Mappings = HashMap::new();
    for line in lines_iter {
        if line.trim().is_empty() { continue; }
        if !line.contains('='){
            moves = line.trim().chars().collect();
            continue;
        }
        let (from, to) = line.split_once('=').unwrap();
        let (left, right) = to.split_once(',').unwrap();
        let mapping = Mapping {
            left: left.trim().replace('(',"").to_string(),
            right: right.trim().replace(')',"").to_string()
        };
        mappings.insert(from.trim().to_string(), mapping);
    }

    let mut paths: Vec<String> = mappings.iter().filter_map(|(k,m)| {
        if k.chars().nth(2).unwrap() == 'A' { return Some(k.to_string()) }
        return None;
    }).collect();

    println!("moves: {}", moves.len()); 

    let mut results: Vec<usize> = Vec::new();

    'outer: for mut path in &mut paths {
        let mut result = 0;
        while !(path.chars().nth(2).unwrap() == 'Z'){
            for mv in &moves {
                let mapping = mappings.get(path).unwrap();
                match mv {
                    'L' => *path = mapping.left.to_string(),
                    'R' => *path = mapping.right.to_string(),
                    _ => panic!()
                }
                result += 1;

                if path.chars().nth(2).unwrap() == 'Z' {
                    results.push(result);
                    continue 'outer;
                }

            }
        }
    }
    let mut num = 1;
    for r in results { 
        num = integer::lcm(num, r);
        println!("{}",num);
    } 

    println!("The result is {}. Elapsed: {:?}", num, start.elapsed().unwrap());
}

