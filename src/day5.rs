use std::{fs::File, io::BufReader, io::prelude::*};
use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
struct Mapping {
    destination: usize,
    source: usize,
    range: usize,
    m_type: MappingType
}

#[derive(Clone)]
enum MappingType {
    S2s,
    S2f,
    F2w,
    W2l,
    L2t,
    T2h,
    H2l,
    None
}

impl fmt::Display for MappingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MappingType::S2s => write!(f, "S2s"),
            MappingType::S2f => write!(f, "S2f"),
            MappingType::F2w => write!(f, "F2w"),
            MappingType::W2l => write!(f, "W2l"),
            MappingType::L2t => write!(f, "L2t"),
            MappingType::T2h => write!(f, "T2h"),
            MappingType::H2l => write!(f, "H2l"),
            MappingType::None => write!(f, "None"),
        }
    }
}

impl Mapping {
    fn map(&self, input: usize) -> Option<usize>{
        if input >= self.source && input <= self.source + self.range - 1 {
            return Some(self.destination + (input - self.source))
        }
        return None
    }
}

impl FromStr for MappingType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed-to-soil map" => Ok(MappingType::S2s),
            "soil-to-fertilizer map" => Ok(MappingType::S2f),
            "fertilizer-to-water map" => Ok(MappingType::F2w),
            "water-to-light map" => Ok(MappingType::W2l),
            "light-to-temperature map" => Ok(MappingType::L2t),
            "temperature-to-humidity map" => Ok(MappingType::T2h),
            "humidity-to-location map" => Ok(MappingType::H2l),
            _ => Err(()) 
        }
    }
}


pub fn pt1(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let mut result : usize = usize::MAX;
    let mut seeds : Vec<usize> = Vec::new();
    let mut mapping_groups: Vec<Vec<Mapping>> = Vec::<Vec<Mapping>>::new();
    let mut current_type: MappingType = MappingType::None;
    let mut current_group : Vec<Mapping> = Vec::new();


    for line in lines_iter {
        if let Some(label) = line.split_once(':') {
            //seeds
            if label.0 == "seeds" {
                seeds = label.1.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            }
            //mapping def
            else {
                println!("{}", label.0);
                current_type = MappingType::from_str(label.0.trim()).unwrap();
                mapping_groups.push(current_group.clone());
                current_group = Vec::new();
            }
        } 
        //mapping
        else {
            let items: Vec<usize> = line.split_whitespace().map(|i| i.parse::<usize>().unwrap()).collect();
            if items.len() > 0 {
                current_group.push(
                    Mapping { 
                        destination: *items.get(0).unwrap(),  
                        source: *items.get(1).unwrap(), 
                        range: *items.get(2).unwrap(),
                        m_type: current_type.clone()
                    }
                    );
            }
        }

    }
    //add last mapping group
    mapping_groups.push(current_group.clone());

    for seed in seeds {
        let mut val = seed.clone();
        for mapping_group in &mapping_groups {
            'group: for mapping in mapping_group {
                if let Some(new) = mapping.map(val){
                    val = new;
                    break 'group;
                }
            }
        }
        result = std::cmp::min(result, val); 
    }

    println!("The closest location for part 1 is {}.", result);
}

pub fn pt2(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let mut result : usize = usize::MAX;
    let mut seed_inputs : Vec<usize> = Vec::new();
    let mut mapping_groups: Vec<Vec<Mapping>> = Vec::<Vec<Mapping>>::new();
    let mut current_type: MappingType = MappingType::None;
    let mut current_group : Vec<Mapping> = Vec::new();


    for line in lines_iter {
        if let Some(label) = line.split_once(':') {
            //seeds
            if label.0 == "seeds" {
                seed_inputs = label.1.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            }
            //mapping def
            else {
                println!("{}", label.0);
                current_type = MappingType::from_str(label.0.trim()).unwrap();
                mapping_groups.push(current_group.clone());
                current_group = Vec::new();
            }
        } 
        //mapping
        else {
            let items: Vec<usize> = line.split_whitespace().map(|i| i.parse::<usize>().unwrap()).collect();
            if items.len() > 0 {
                current_group.push(
                    Mapping { 
                        destination: *items.get(0).unwrap(),  
                        source: *items.get(1).unwrap(), 
                        range: *items.get(2).unwrap(),
                        m_type: current_type.clone()
                    }
                    );
            }
        }

    }
    //add last mapping group
    mapping_groups.push(current_group.clone());

    let mut start : usize = 0;
    for (idx, seed_input) in seed_inputs.into_iter().enumerate() {
        match idx%2 {
            0 => start = seed_input,
            _ => {
                let end = start+seed_input-1;
                println!("Ranging: {}->{}", start,end );
                for item in start..=end {
                    let mut val = item.clone();
                    for mapping_group in &mapping_groups {
                        'group: for mapping in mapping_group {
                            if let Some(new) = mapping.map(val){
                                // if item == 82 {
                                //     println!("Mapping: des:{},src:{},rng:{},type:{}", mapping.destination, mapping.source, mapping.range, mapping.m_type);
                                // }
                                val = new;
                                break 'group;
                            }
                        } 
                    }
                    result = std::cmp::min(result, val);
                }
            }
        }
    }

    // for seed in seeds {
    //     let mut val = seed.clone();
    //     for mapping_group in &mapping_groups {
    //         'group: for mapping in mapping_group {
    //             if let Some(new) = mapping.map(val){
    //                 val = new;
    //                 break 'group;
    //             }
    //         }
    //     }
    //     result = std::cmp::min(result, val); 
    // }
    // for (idx, start) in starts.into_iter().enumerate() {
    //     println!("Seed start: {} range: {}", start, ranges.get(idx).unwrap());
    //     for item in start..start+ranges.get(idx).unwrap() {
    //         seeds.push(item);
    //     }
    // }
    // for seed in seeds {
    //     let mut val = seed.clone();
    //     for mapping_group in &mapping_groups {
    //         'group: for mapping in mapping_group {
    //             if let Some(new) = mapping.map(val){
    //                 val = new;
    //                 break 'group;
    //             }
    //         }
    //     }
    //     result = std::cmp::min(result, val); 
    // }

    println!("The closest location for part 2 is {}.", result);
}
