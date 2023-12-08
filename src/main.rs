mod utils;
use utils::Day;


mod day1;
mod day2;
mod day3;

fn main() {
    let one = Day::new(1, 2023);
    day1::pt1(one.get_input());
    day1::pt2(one.get_input());
    let two = Day::new(2, 2023);
    day2::pt2(two.get_input());
    let three = Day::new(3, 2023);
    day3::pt1(three.get_input());
    day3::pt2(three.get_input());
    //let idx = "nine91threepdcthjkmrthreeeightwonsg".find("three");
    //let idx = "nine91threepdcthjkmrthreeeightwonsg".find("three");
    //if let Some(a) = idx {
    //println!("{}", a);
    //}
    //day2::day2(utils::get_input(2, 2023));
}

