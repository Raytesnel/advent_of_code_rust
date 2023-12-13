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
