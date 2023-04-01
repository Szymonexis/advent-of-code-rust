mod days;
mod read_file_lines;

use days::{
    day_five::day_five, day_four::day_four, day_one::day_one, day_three::day_three,
    day_two::day_two, day_six::day_six, days::Days,
};
use read_file_lines::read_file_lines;

fn main() {
    // day 1.1 && 1.2
    println!("day 1");
    day_one(&read_file_lines(&Days::get_day(1)));

    // day 2.1 && 2.2
    println!("day 2");
    day_two(&read_file_lines(&Days::get_day(2)));

    // day 3.1 && 3.2
    println!("day 3");
    day_three(&read_file_lines(&Days::get_day(3)));

    // day 4.1 && 4.2
    println!("day 4");
    day_four(&read_file_lines(&Days::get_day(4)));

    // day 5.1 && 5.2
    println!("day 5");
    day_five(&read_file_lines(&Days::get_day(5)));

    // day 6.1 && 6.2
    println!("day 6");
    day_six(&read_file_lines(&Days::get_day(6)));
}


