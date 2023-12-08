use std::{fs::File, io::BufReader, io::prelude::*, default, borrow::BorrowMut};

struct Part {
    name: usize,
    range: (usize, usize),
}

struct Symbol {
    name: char,
    pos: usize
}

pub fn pt1(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let mut result : usize = 0;
    let mut linemat : Vec<String> = Vec::new();
    let mut partmat : Vec<Vec<Part>> = Vec::new();
    linemat.push("".to_string());
    partmat.push(Vec::new());
    for line in lines_iter {

        linemat.push(line.to_owned());
        partmat.push(get_parts(&line));
    }
    linemat.push("".to_string());
    partmat.push(Vec::new());
    for (i, parts) in partmat.iter().enumerate() {
        'parts: for part in parts {
            if let Some(line1) = linemat.get(i-1){
                if validate_part(part, line1) { result += part.name; continue 'parts};
            }
            if let Some(line2) = linemat.get(i){
                if validate_part(part, line2) { result += part.name; continue 'parts};
            }
            if let Some(line3) = linemat.get(i+1){
                if validate_part(part, line3) { result += part.name; continue 'parts};
            }

            println!("Part:{} invalid:\nLines:\n{}\n{}\n{}", part.name,  linemat.get(i-1).unwrap(), linemat.get(i).unwrap(), linemat.get(i+1).unwrap_or(&"".to_string()));
        }
    }


    println!("The result is {}.", result);
}

pub fn pt2(file_reader: BufReader<File>){
    let lines_iter = file_reader.lines().map(|l| l.unwrap());

    let mut result : usize = 0;
    let mut linemat : Vec<String> = Vec::new();
    let mut symmat : Vec<Vec<Symbol>> = Vec::new();
    linemat.push("".to_string());
    symmat.push(Vec::new());
    for line in lines_iter {
        linemat.push(line.to_owned());
        symmat.push(get_symbols(&line));
    }
    linemat.push("".to_string());
    symmat.push(Vec::new());

    for (i, symbols) in symmat.iter().enumerate() {
         for symbol in symbols {
            let lines : Vec<String> = linemat[i-1..=i+1].to_vec();
            if let Some(gear_ratio) = validate_gear(symbol, lines){
               result += gear_ratio;         
            }
        }
    }


    println!("The total gear ratio is {}.", result);
}

fn validate_gear(symbol: &Symbol, lines: Vec<String>) -> Option<usize> { 
    if symbol.name.ne(&'*') { return None };

    let mut valid_parts : Vec<Part> = Vec::new();
    for line in &lines {
        let parts = get_parts(&line);
        for part in parts {
            if symbol.pos + 1 >= part.range.0 && symbol.pos <= part.range.1 + 1 {
                valid_parts.push(part);
            }
        }
    }
    if valid_parts.len() == 2 { 
        println!("Parts:{},{} validate gear {}{}",valid_parts.get(0).unwrap().name,valid_parts.get(1).unwrap().name, symbol.name, symbol.pos);
        for line in lines {
            println!("{}", line);
        }
        return Some(valid_parts.get(0).unwrap().name*valid_parts.get(1).unwrap().name); 
    }
    return None;
}

fn get_parts(line: &String) -> Vec<Part> {
    let mut start_idx = 0;
    let mut recording = false;
    let mut parts : Vec<Part> = Vec::new();

    for (idx, char) in line.chars().enumerate() {
        if !recording && char.is_ascii_digit(){
            start_idx = idx;
            recording = true;
        }
        if recording {
            if !char.is_ascii_digit() {
                let part = Part { 
                    name: line[start_idx..=idx-1].parse::<usize>().unwrap(), 
                    range: (start_idx, idx-1), 
                };
                parts.push(part);
                recording = false;
                start_idx = 0
            }
            else if idx == line.len() - 1 {
                let part = Part {
                    name: line[start_idx..=idx].parse::<usize>().unwrap(),
                    range: (start_idx, idx)
                };
                parts.push(part);
                recording = false;
                start_idx = 0
            }
        }
    }
    parts
}

fn validate_part(part: &Part, line : &String) -> bool {
    let symbols = get_symbols(&line);
    for symbol in symbols {
        if symbol.pos + 1 >= part.range.0 && symbol.pos <= part.range.1 + 1 {
            println!("Symbol:{},{} validates: Part:{}", symbol.name, symbol.pos, part.name );
            return true;
        }
    }
    return false;
}



fn get_symbols(line: &String) -> Vec<Symbol> {
    let mut symbols : Vec<Symbol> = Vec::new();
    for (i, char) in line.chars().enumerate() {
        if !&char.is_ascii_digit() && !&char.eq(&'.'){
            symbols.push(Symbol { name: char, pos: i });
        }
    }
    symbols
}
