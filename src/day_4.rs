// use crate::read_file;

fn find_key(game_items: Vec<u32>, lot_items: Vec<u32>) -> Vec<u32> {
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

pub fn day_4_a(file_contents: String) -> u32 {
    let mut sum_key: u32 = 0;
    for line in file_contents.lines() {
        let (lot_line, game_line) = pred_game_string(&line.to_string());
        let key_list = find_key(game_line, lot_line);
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
        let found_items: Vec<u32> = find_key(lot_items, game_items);
        // assert
        assert_eq!(expected_found, found_items)
    }
}
