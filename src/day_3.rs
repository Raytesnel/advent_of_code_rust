use std::iter::{Enumerate, Peekable};
use std::str::Chars;

fn check_symbol_around(
    file_contents: &String,
    index_char: usize,
    index_number: usize,
    index_row: usize,
    numbers_around_symbol: String,
) -> Result<u32, String> {
    if let Some(_symbol_index) =
        find_symbol_around_numbers(&file_contents, index_char, index_number, index_row)
    {
        if let Ok(combined_integer) = numbers_around_symbol.parse::<u32>() {
            return Ok(combined_integer);
        } else {
            return Err("Failed to convert to u32".to_string());
        }
    }
    Err("Symbol not found".to_string())
}

fn get_all_number_series(
    index_char: usize,
    file_item: char,
    chars_iter: &mut Peekable<Enumerate<Chars<'_>>>,
) -> (String, u32) {
    let mut numbers_around_symbol = String::new();
    let mut index_number: u32 = index_char as u32;
    numbers_around_symbol.push(file_item);
    while let Some((next_index, next_item)) = chars_iter.peek() {
        if next_item.is_digit(10) {
            numbers_around_symbol.push(*next_item);
            index_number = *next_index as u32;
            chars_iter.next(); // Move the iterator forward
        } else {
            break;
        }
    }
    return (numbers_around_symbol, index_number);
}

fn find_symbol_around_numbers(
    file_list: &str,
    start_index: usize,
    end_index: usize,
    index_row: usize,
) -> Option<usize> {
    // Define the range to check for symbols (e.g., row 0 till 2 with index 2 till 6)
    let symbol_start = start_index.saturating_sub(1);
    let symbol_end = end_index.saturating_add(1);
    let start_row: usize;
    let end_row: usize;
    if index_row == 0 {
        start_row = 0;
        end_row = 1;
    } else {
        start_row = index_row - 1;
        end_row = index_row + 1;
    }
    for (_index_row, file_line) in file_list
        .lines()
        .skip(start_row)
        .take(end_row - start_row + 1)
        .enumerate()
    {
        for (index, symbol_candidate) in file_line
            .chars()
            .skip(symbol_start)
            .take(symbol_end - symbol_start + 1)
            .enumerate()
        {
            if is_symbol(&symbol_candidate) {
                // println!("found symbol next to number at {:?} : {:?}",index_row,index);
                return Some(symbol_start + index);
            }
        }
    }

    // Iterate over the specified range to find a symbol

    None
}

fn is_symbol(c: &char) -> bool {
    if c.to_string() == "." {
        return false;
    }
    !c.is_alphanumeric()
}

fn is_digit(c: char) -> bool {
    c.is_digit(10) && c.is_ascii_digit()
}

pub fn day_3_a(file_contents: String) {
    let file_list = file_contents.lines();
    let mut total_sum: u32 = 0;
    for (index_row, file_line) in file_list.clone().enumerate() {
        let mut chars_iter = file_line.chars().enumerate().peekable();
        while let Some((index_char, file_item)) = chars_iter.next() {
            if is_digit(file_item) {
                let (numbers_around_symbol, index_number) =
                    get_all_number_series(index_char, file_item, &mut chars_iter);
                // now we need to check if there is a symbol around the number series
                match check_symbol_around(
                    &file_contents,
                    index_char,
                    index_number as usize,
                    index_row,
                    numbers_around_symbol,
                ) {
                    Ok(result) => total_sum += result,
                    Err(error) => {
                        if !error.contains("Symbol not found") {
                            println!("Error: {}", error);
                        }
                    }
                }
            }
        }
    }
    println!("3-A: total number = {:?}", total_sum)
}
