use std::io::BufReader;
use std::fs::File;


pub struct Day {
    pub day: u32,
    pub year: u32
}

impl Day {
    pub fn get_input(&self) -> BufReader<File> {
        let file = File::open(format!("inputs/{0}/day{1}", self.year.to_string(), self.day.to_string())).unwrap();
        BufReader::new(file)
    } 

    pub fn new(day: u32, year: u32) -> Day {
        Day { day, year }
    }
}

