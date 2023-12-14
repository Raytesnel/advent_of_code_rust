static NUMBER_CHARS: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

pub fn assignment_a(file_contents: String) -> u32 {
    let mut found_numbers = vec![];
    for line in file_contents.lines() {
        let mut line_list = vec![];
        for character in line.chars() {
            if character.is_numeric() {
                line_list.push(character)
            }
        }
        if let (Some(first), Some(last)) = (line_list.first(), line_list.last()) {
            if let (Some(first_digit), Some(last_digit)) = (first.to_digit(10), last.to_digit(10)) {
                let combined_integer = first_digit * 10 + last_digit;
                found_numbers.push(combined_integer);
            } else {
                println!("Invalid characters in the vector");
            }
        } else {
            println!("Vector is empty");
        }
    }
    let sum: u32 = found_numbers.iter().sum();
    println!("1-A: total number = {sum}");
    sum
}

fn convert_string_to_u32(s: &str) -> Option<u32> {
    if let Some(index) = NUMBER_CHARS.iter().position(|&item| item == s) {
        // Index found, add 1 to convert to u32
        Some((index + 1) as u32)
    } else {
        None
    }
}

pub fn assignment_b(file_contents: String) -> u32 {
    let mut found_numbers: Vec<u32> = vec![];
    for line in file_contents.lines() {
        let first = find_first_number(line);
        let second = find_last_number(line);
        if let (Some(first_digit), Some(second_digit)) = (first, second) {
            let combined_integer_str = format!("{}{}", first_digit, second_digit);
            // Attempt to parse the string back to u32
            if let Ok(combined_integer) = combined_integer_str.parse::<u32>() {
                found_numbers.push(combined_integer);
            } else {
                println!("Failed to convert to u32");
                break;
            }
        }
    }
    let sum = count_numbers_from_list(found_numbers);
    sum
}

fn count_numbers_from_list(found_numbers: Vec<u32>) -> u32 {
    let sum: u32 = found_numbers.iter().sum();
    println!("1-B: total number = {sum}");
    sum
}

fn find_first_number(line: &str) -> Option<u32> {
    let mut collected_chars = String::new();
    for character in line.chars() {
        collected_chars.push(character);
        if character.is_numeric() {
            return character.to_digit(10);
        } else {
            for char_number in NUMBER_CHARS {
                if collected_chars.contains(char_number) {
                    return convert_string_to_u32(char_number);
                }
            }
        }
    }
    None
}
fn find_last_number(line: &str) -> Option<u32> {
    let mut collected_chars = String::new();
    for character in line.chars().rev() {
        collected_chars.insert(0, character);
        if character.is_numeric() {
            return character.to_digit(10);
        } else {
            for char_number in NUMBER_CHARS {
                if collected_chars.contains(char_number) {
                    return convert_string_to_u32(char_number);
                }
            }
        }
    }
    None
}
