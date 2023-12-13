mod day_1;
mod day_2;
mod day_3;

use std::fs;

fn main() {
    day_1::assignment_a(read_file("src/input_data/input_day_1.txt"));
    day_1::assignment_b(read_file("src/input_data/input_day_1.txt"));
    day_2::day_2_a(read_file("src/input_data/input_day_2.txt"));
    day_2::day_2_b(read_file("src/input_data/input_day_2.txt"));
    day_3::day_3_a(read_file("src/input_data/input_day_3.txt"));
    day_3::day_3_b(read_file("src/input_data/input_day_3.txt"));
}

fn read_file(input_path: &str) -> String {
    //read the file returns a really long string
    let file_contents = fs::read_to_string(input_path)
        .expect("LogRocket: Should have been able to read the file{}");
    return file_contents;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        let day1a = day_1::assignment_a(read_file("src/input_data/input_day_1.txt"));
        assert_eq!(day1a, 56042);
        let day1b = day_1::assignment_b(read_file("src/input_data/input_day_1.txt"));
        assert_eq!(day1b, 55358);
        let day2a = day_2::day_2_a(read_file("src/input_data/input_day_2.txt"));
        assert_eq!(day2a, 2720);
        let day2b = day_2::day_2_b(read_file("src/input_data/input_day_2.txt"));
        assert_eq!(day2b, 71535);
        let day3a = day_3::day_3_a(read_file("src/input_data/input_day_3.txt"));
        assert_eq!(day3a, 514969);
        let day3b = day_3::day_3_b(read_file("src/input_data/input_day_3.txt"));
        assert_eq!(day3b, 78915902);
    }
}
