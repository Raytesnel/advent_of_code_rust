// use crate::read_file;

use std::collections::HashMap;

fn find_key(game_items: &Vec<u32>, lot_items: &Vec<u32>) -> Vec<u32> {
    let mut winner_items: Vec<u32> = Vec::new();
    for lot_item in lot_items {
        if game_items.contains(&lot_item) {
            winner_items.push(lot_item.clone())
        }
    }
    winner_items
}

fn pred_game_string(file_line: &String) -> (Vec<u32>, Vec<u32>) {
    // Split the file_line into parts using "|"
    let parts: Vec<&str> = file_line.split("|").collect();

    // Extract the first part as lot_line
    let lot_line: Vec<u32> = parts
        .get(0)
        .unwrap_or(&"")
        .split_whitespace()
        .skip(1) // Skip the initial "Card"
        .flat_map(|s| s.parse::<u32>())
        .collect();

    let game_line: Vec<u32> = parts
        .get(1)
        .unwrap()
        .split_whitespace()
        .flat_map(|s| s.parse::<u32>())
        .collect();

    // Return the tuple with both vectors
    (lot_line, game_line)
}

fn count_list(item_list: &Vec<u32>) -> u32 {
    let length = item_list.len() as u32;
    let number: u32 = 2;
    if length > 1 {
        number.pow(length - 1)
    } else if length == 1 {
        1
    } else {
        0
    }
}
enum Thing {
    Vec(Vec<u32>),
    Number(u32),
}

fn prepare_card_data(file_lines: String) -> HashMap<u32, Vec<Thing>> {
    let mut found_number_index: HashMap<u32, Vec<Thing>> = HashMap::new();
    for (index, line) in file_lines.lines().enumerate() {
        let (lot_line, game_line) = pred_game_string(&line.to_string());
        let multiplier = 0;
        let line_vec: Vec<Thing> = vec![
            Thing::Number(multiplier),
            Thing::Vec(lot_line),
            Thing::Vec(game_line),
        ];
        found_number_index.insert(index as u32 + 1, line_vec);
    }
    found_number_index
}

pub fn day_4_b(file_contents: String) -> u32 {
    let mut hasmap_items = prepare_card_data(file_contents);
    for card_number in 1..=hasmap_items.len() {
        if let Some(card_item) = hasmap_items.get_mut(&(card_number as u32)) {
            if let (
                Thing::Number(actual_multiplier),
                Thing::Vec(actual_lot_items),
                Thing::Vec(actual_game_items),
            ) = (&card_item[0], &card_item[1], &card_item[2])
            {
                let key_list = find_key(actual_game_items, actual_lot_items);
                // println!("with card {:?} found number: {:?}", card_number, key_list);
                // println!("found {:?} numbers", key_list.len());
                for _ in 0..=*actual_multiplier {
                    for index_keys in 1..=key_list.len() {
                        if let Some(next_card_item) =
                            hasmap_items.get_mut(&(card_number as u32 + index_keys as u32))
                        {
                            if let Some(Thing::Number(next_multiplier)) = next_card_item.get_mut(0)
                            {
                                *next_multiplier += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    let total_multiply: u32 = hasmap_items
        .iter()
        .map(|(_, item)| {
            if let Thing::Number(num) = &item[0] {
                *num
            } else {
                0
            }
        })
        .sum();
    let total_sum = total_multiply + hasmap_items.len() as u32;
    println!("4-A: total number = {:?}", total_sum);
    total_sum
}

pub fn day_4_a(file_contents: String) -> u32 {
    let mut sum_key: u32 = 0;
    for line in file_contents.lines() {
        let (lot_line, game_line) = pred_game_string(&line.to_string());
        let key_list = find_key(&game_line, &lot_line);
        let number_added = count_list(&key_list);
        sum_key += number_added;
    }
    println!("4-A: total number = {:?}", sum_key);
    sum_key
}

#[cfg(test)]
mod tests {
    use super::*;

    // use crate::read_file;

    #[test]
    fn test_example_4b() {
        let file_contents: String = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .to_string();
        let total_sum = 30;

        let returned_sum = day_4_b(file_contents);

        assert_eq!(returned_sum, total_sum)
    }

    #[test]
    fn test_prepare_lines_vectors() {
        let file_contents: String = "Card   1:  2 15 17 11 64 59 45 41 61 19 |  4 36 62 43 94 41 24 25 13 83 97 86 61 90 67  7 15 58 18 19 38 17 49 52 37".to_string();
        let card_number: u32 = 1;
        let expected_lot_items = vec![2, 15, 17, 11, 64, 59, 45, 41, 61, 19];
        let expected_game_items = vec![
            4, 36, 62, 43, 94, 41, 24, 25, 13, 83, 97, 86, 61, 90, 67, 7, 15, 58, 18, 19, 38, 17,
            49, 52, 37,
        ];
        let expected_multiplier = 0;

        let hashmap = prepare_card_data(file_contents);
        if let Thing::Vec(actual_lot_items) = &hashmap.get(&card_number).unwrap()[1] {
            assert_eq!(&expected_lot_items, actual_lot_items);
        } else {
            panic!("Expected Thing::Vec for lot items");
        }
        if let Thing::Vec(actual_game_items) = &hashmap.get(&card_number).unwrap()[2] {
            assert_eq!(&expected_game_items, actual_game_items);
        } else {
            panic!("Expected Thing::Vec for game items");
        }
        if let Thing::Number(actual_multiplier) = hashmap.get(&card_number).unwrap()[0] {
            assert_eq!(expected_multiplier, actual_multiplier);
        } else {
            panic!("Expected Thing::Number for multiplier");
        }
    }

    #[test]
    fn test_day_4_multiple_lines() {
        let file_contents: String = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .to_string();
        let expected_sum: u32 = 13;

        let sum = day_4_a(file_contents);

        assert_eq!(expected_sum, sum)
    }

    #[test]
    fn test_day_4_a() {
        let file_contents: String = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .to_string();
        let expected_values = [8, 2, 2, 1, 0, 0];
        for (index, file_line) in file_contents.lines().enumerate() {
            if let Some(&value) = expected_values.get(index) {
                let sum: u32 = day_4_a(file_line.to_string());

                assert_eq!(sum, value)
            } else {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn test_count_list() {
        let item_list = vec![41, 61, 15, 19, 17];
        let expected_result: u32 = 16;

        let result = count_list(&item_list);

        assert_eq!(expected_result, result)
    }

    #[test]
    fn test_prep_data_line() {
        let file_contents: String = "Card   1:  2 15 17 11 64 59 45 41 61 19 |  4 36 62 43 94 41 24 25 13 83 97 86 61 90 67  7 15 58 18 19 38 17 49 52 37".to_string();
        let expected_lot_items = vec![2, 15, 17, 11, 64, 59, 45, 41, 61, 19];
        let expected_game_items = vec![
            4, 36, 62, 43, 94, 41, 24, 25, 13, 83, 97, 86, 61, 90, 67, 7, 15, 58, 18, 19, 38, 17,
            49, 52, 37,
        ];

        let (lot_items, game_items) = pred_game_string(&file_contents);

        assert_eq!(lot_items, expected_lot_items);
        assert_eq!(game_items, expected_game_items);
    }
    #[test]
    fn test_find_item_in_list() {
        // arrange
        let lot_items: Vec<u32> = vec![2, 15, 17, 11, 64, 59, 45, 41, 61, 19];
        let game_items: Vec<u32> = vec![
            4, 36, 62, 43, 94, 41, 24, 25, 13, 83, 97, 86, 61, 90, 67, 7, 15, 58, 18, 19, 38, 17,
            49, 52, 37,
        ];
        let expected_found: Vec<u32> = vec![41, 61, 15, 19, 17];
        // act
        let found_items: Vec<u32> = find_key(&lot_items, &game_items);
        // assert
        assert_eq!(expected_found, found_items)
    }
}
