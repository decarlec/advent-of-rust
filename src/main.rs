mod utils;
use utils::Day;


mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let one = Day::new(1, 2023);
    //day1::pt1(one.get_input());
    //day1::pt2(one.get_input());
    let two = Day::new(2, 2023);
    //day2::pt2(two.get_input());
    let three = Day::new(3, 2023);
//    day3::pt1(three.get_input());
//   day3::pt2(three.get_input());
    let four = Day::new(4, 2023);
//    day4::pt2(four.get_input());

    let five = Day::new(5, 2023);
    //day5::pt1(five.get_input());
    //day5::pt2(five.get_input());

    let six = Day::new(6, 2023);
//    day6::pt1(six.get_input());
    let seven = Day::new(7, 2023);
//k    day7::pt1(seven.get_input());

    let eight = Day::new(8, 2023);
//    day8::pt1(eight.get_input());
    let nine = Day::new(9, 2023);
    day9::pt1(nine.get_input());
//
}

