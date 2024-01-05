use std::collections::HashMap;

static SEED_ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn is_number_in_range(
    items_dict: &HashMap<String, Vec<Vec<u32>>>,
    key: &str,
    number: u32,
) -> Result<u32, String> {
    if let Some(entry) = items_dict.get(key) {
        for (index, range) in entry.iter().enumerate() {
            let start = range[1];
            let end = range[1] + range[2] - 1;

            if number >= start && number <= end {
                return Ok(index as u32); // Number is within the range
            }
        }
    }
    Err("nothing found".to_string()) // Number is not in any range
}

fn retrieve_seeds(line: &str) -> Result<Vec<u32>, &'static str> {
    if let Some(first_line) = line.lines().next() {
        if first_line.contains("seeds") {
            let seed_list: Vec<u32> = first_line
                .split_whitespace()
                .skip(1)
                .map(|item| item.parse::<u32>().unwrap_or_default())
                .collect();
            Ok(seed_list)
        } else {
            Err("not found seeds")
        }
    } else {
        Err("empty string")
    }
}

fn split_mapping_from_line(multiline_string: &String) -> HashMap<String, Vec<Vec<u32>>> {
    let lines = multiline_string.lines();

    let mut block_list: Vec<Vec<u32>> = Vec::new();
    let mut header: String = "".to_string();
    let mut items_dict: HashMap<String, Vec<Vec<u32>>> = HashMap::new();

    for line in lines.skip(2) {
        if line == "        " {
            items_dict.insert(header.clone(), block_list.clone());
            header.clear();
            block_list.clear(); // Clear block_list for the next section
        } else if line.contains(" map:") {
            header = line.replace(" map:", "").replace(" ", "");
        } else {
            let temp_items: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            let block_item: Vec<u32> = temp_items
                .iter()
                .map(|item| item.parse::<u32>().unwrap_or_default())
                .collect();
            block_list.push(block_item);
        }
    }
    let mut updated_dict: HashMap<String, Vec<Vec<u32>>> = HashMap::new();
    for (key, values) in items_dict.clone().into_iter() {
        let mut sorted_values = values;
        sorted_values.sort_by_key(|inner_vec| inner_vec[1]);
        updated_dict.insert(key, sorted_values);
    }

    updated_dict
}

pub fn day_5_a(file_contents: String) {
    split_mapping_from_line(&file_contents);

    println!("hello dag 5 a")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::collections::HashMap;
    #[fixture]
    fn example_input_data() -> String {
        r#"seeds: 79 14 55 13
        
        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"#
            .to_string()
    }

    #[fixture]
    fn example_input_split() -> HashMap<String, Vec<Vec<u32>>> {
        let mut items_dict: HashMap<String, Vec<Vec<u32>>> = HashMap::new();
        items_dict.insert(
            "seed-to-soil".to_string(),
            vec![vec![52, 50, 48], vec![50, 98, 2]],
        );
        items_dict.insert(
            "soil-to-fertilizer".to_string(),
            vec![vec![39, 0, 15], vec![0, 15, 37], vec![37, 52, 2]],
        );
        items_dict.insert(
            "fertilizer-to-water".to_string(),
            vec![
                vec![42, 0, 7],
                vec![57, 7, 4],
                vec![0, 11, 42],
                vec![49, 53, 8],
            ],
        );
        items_dict.insert(
            "water-to-light".to_string(),
            vec![vec![88, 18, 7], vec![18, 25, 70]],
        );
        items_dict.insert(
            "light-to-temperature".to_string(),
            vec![vec![81, 45, 19], vec![68, 64, 13], vec![45, 77, 23]],
        );
        items_dict.insert(
            "temperature-to-humidity".to_string(),
            vec![vec![1, 0, 69], vec![0, 69, 1]],
        );
        items_dict.insert(
            "humidity-to-location".to_string(),
            vec![vec![60, 56, 37], vec![56, 93, 4]],
        );
        items_dict
    }

    #[rstest]
    fn test_split_text_map(
        #[from(example_input_data)] input_data: String,
        example_input_split: HashMap<String, Vec<Vec<u32>>>,
    ) {
        let result_split_map = split_mapping_from_line(&input_data);
        for key in result_split_map.keys() {
            assert_eq!(
                result_split_map.get(key),
                example_input_split.get(key),
                "wrong input with key: {:?}",
                key
            )
        }
    }

    #[rstest]
    fn test_if_number_is_in_range(example_input_split: HashMap<String, Vec<Vec<u32>>>) {
        let key_to_find = "seed-to-soil";
        let number: u32 = 60;
        let number_found = is_number_in_range(&example_input_split, key_to_find, number);
        assert_eq!(number_found.unwrap(), 0);
        let key_to_find = "fertilizer-to-water";
        let number: u32 = 60;
        let number_found = is_number_in_range(&example_input_split, key_to_find, number);
        assert_eq!(number_found.unwrap(), 3);
        let key_to_find = "light-to-temperature";
        let number: u32 = 64;
        let number_found = is_number_in_range(&example_input_split, key_to_find, number);
        assert_eq!(number_found.unwrap(), 1)
    }

    // #[fixture]
    // fn seed_locations() -> Vec<Vec<u32>> {
    //     let seed_locations = vec![vec![79, 81], vec![14, 14], vec![55, 57], vec![13, 13]];
    //     seed_locations
    // }

    // #[rstest]
    // fn test_find_seed_locations(
    //     seed_locations: Vec<Vec<u32>>,
    //     example_input_split: HashMap<String, Vec<Vec<u32>>>,
    // ) {
    //     for seed_combi in seed_locations {
    //         let seed_number = seed_combi[0];
    //         let result_seed_loctions = follow_seed(&example_input_split, &seed_number);
    //         assert_eq!(result_seed_loctions, seed_combi[1])
    //     }
    // }

    #[fixture]
    fn example_input_seeds() -> Vec<u32> {
        vec![79, 14, 55, 13]
    }

    #[rstest]
    fn test_retrieve_seeds(
        #[from(example_input_data)] input_data: String,
        example_input_seeds: Vec<u32>,
    ) {
        match retrieve_seeds(&input_data) {
            Ok(seed_list) => {
                let returned_seeds = seed_list;
                assert_eq!(example_input_seeds, returned_seeds)
            }
            Err(error_message) => {
                println!("test failed {}", error_message)
            }
        }
    }
}
