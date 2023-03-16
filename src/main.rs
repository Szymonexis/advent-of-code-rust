mod days;
mod read_file_lines;

use days::{day_one::day_one, day_two::day_two, days::Days};
use read_file_lines::read_file_lines;

fn main() {
    // day 1.1 && 1.2
    day_one(&read_file_lines(&Days::get_day(1)));

    // day 2.1 && 2.2
    day_two(&read_file_lines(&Days::get_day(2)));
}
