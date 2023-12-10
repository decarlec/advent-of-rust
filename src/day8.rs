use std::{fs::File, io::BufReader, io::prelude::*, time::SystemTime, collections::HashMap};

struct Mapping {
    from: [char; 3],
    left: [char; 3],
    right: [char; 3]
}

type Mappings<'a> = HashMap<String, Mapping>;

pub fn pt1(file_reader: BufReader<File>){
    let start = SystemTime::now();
    let lines_iter = file_reader.lines().map(|l| l.unwrap());
    
    let mut result: usize = 0;
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
            from: from.chars().collect::<Vec<char>>()[0..=2].try_into().unwrap(),
            left: left.chars().collect::<Vec<char>>()[2..=4].try_into().unwrap(),
            right: right.chars().collect::<Vec<char>>()[1..=3].try_into().unwrap(),
        };
        //println!("from: {}, left:{}, right:{}", mapping.from.iter().collect::<String>(), mapping.left.iter().collect::<String>(), mapping.right.iter().collect::<String>());
        mappings.insert(from.to_string(), mapping);
    }
    //println!("from: {}, left:{}, right:{}", start.from.iter().collect::<String>(), start.left.iter().collect::<String>(), start.right.iter().collect::<String>());

    println!("{}",moves.iter().collect::<String>());
    //let mut cur: [char;3] = ['A','A','A'];
    let mut paths = mappings.iter().filter_map(|m| {
        if m.1.from[2] == 'A' { return Some(m.1.from); } 
        return None;}).collect::<Vec<[char;3]>>();

    'outer: loop {
        for mv in &moves {
            for path in &mut paths {
                let mapping = mappings.get(path).unwrap();
                //println!("from: {}, left:{}, right:{}", mapping.from.iter().collect::<String>(), mapping.left.iter().collect::<String>(), mapping.right.iter().collect::<String>());
                match mv {
                    'L' => *path = mapping.left.clone(),
                    'R' => *path = mapping.right.clone(),
                    _ => panic!()
                }
                //println!("Path:{}", path.iter().collect::<String>());
            }
            result += 1; 
            if paths.iter().all(|p| p[2] == 'Z') { println!("Breaking"); break 'outer;};
        }
    }


    println!("The result is {}. Elapsed: {:?}", result, start.elapsed().unwrap());
}

