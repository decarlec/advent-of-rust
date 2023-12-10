use std::{fs::File, io::BufReader, io::prelude::*, time::SystemTime, collections::HashMap};

struct Mapping{
    left: Path,
    right: Path 
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
            left: Path { val: left.trim().replace('(',"").to_string(), last: left.chars().nth(2).unwrap().to_owned()},
            right: Path { val: right.trim().replace('(',"").to_string(), last: right.chars().nth(2).unwrap()},
        };
        //println!("from: {}, left:{}, right:{}", mapping.from.iter().collect::<String>(), mapping.left.iter().collect::<String>(), mapping.right.iter().collect::<String>());
        mappings.insert(from.trim().to_string(), mapping);
    }
    //println!("from: {}, left:{}, right:{}", start.from.iter().collect::<String>(), start.left.iter().collect::<String>(), start.right.iter().collect::<String>());

    //println!("{}",moves.iter().collect::<String>());
    //let mut cur: [char;3] = ['A','A','A'];

    let mut paths: Vec<Path> = mappings.iter().filter_map(|(k,m)| {
        if k.chars().nth(2).unwrap() == 'A' { return Some( Path { val: k.to_string(), last: k.chars().nth(2).unwrap()}); }
        return None;
        }).collect();

    'outer: loop {
        for mv in &moves {
            for mut path in &mut paths {
                let mapping = mappings.get(&*path.val).unwrap();
                //println!("left:{}, right:{}", mapping.left, mapping.right);
                match mv {
                    'L' => path = mapping.left,
                    'R' => path = mapping.right,
                    _ => panic!()
                }
                //println!("Path:{}", path.iter().collect::<String>());
            }
            result += 1; 
            if paths.iter().all(|p| p.last == 'Z') { println!("Breaking"); break 'outer;};
        }
        let e = start.elapsed().unwrap(); 
        if e.as_secs() > 0 {
        println!("Results {}. Elapsed: {:?}. Result/s:{}", result, e, result/e.as_secs());
        }
    }


    println!("The result is {}. Elapsed: {:?}", result, start.elapsed().unwrap());
}

