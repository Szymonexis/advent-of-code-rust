use std::process;

pub struct Days {}

impl Days {
    pub fn get_day(index: usize) -> String {
        let days: Vec<String> = vec![
            "day_one".to_string(),
            "day_two".to_string(),
            "day_three".to_string(),
            "day_four".to_string(),
            "day_five".to_string(),
            "day_six".to_string(),
            "day_seven".to_string(),
            "day_eight".to_string(),
            "day_nine".to_string(),
            "day_ten".to_string(),
            "day_eleven".to_string(),
            "day_twelve".to_string(),
            "day_thirteen".to_string(),
            "day_fourteen".to_string(),
            "day_fifteen".to_string(),
            "day_sixteen".to_string(),
            "day_seventeen".to_string(),
            "day_eighteen".to_string(),
            "day_nineteen".to_string(),
            "day_twenty".to_string(),
            "day_twentyone".to_string(),
            "day_twentytwo".to_string(),
            "day_twentythree".to_string(),
            "day_twentyfour".to_string(),
            "day_twentyfive".to_string(),
        ];

        let mut result = String::from("puzzles/");

        let day = match days.get(index - 1) {
            Some(some_day) => some_day,
            None => {
                eprintln!("Incorrect index!");
                process::exit(1);
            }
        };
        result.push_str(&day);
        result.push_str(".txt");

        return result;
    }
}
