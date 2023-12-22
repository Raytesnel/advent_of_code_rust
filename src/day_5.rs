use std::collections::HashMap;

fn retrieve_seeds(line: &str) -> Result<Vec<&str>, &'static str> {
    if let Some(first_line) = line.lines().next() {
        if first_line.contains("seeds") {
            let seed_list: Vec<&str> = first_line.split_whitespace().skip(1).collect();
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
    items_dict
}

// fn mapping_numbers_hasmap(lines: &String, mapping: &str) {}
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
            vec![vec![50, 98, 2], vec![52, 50, 48]],
        );
        items_dict.insert(
            "soil-to-fertilizer".to_string(),
            vec![vec![0, 15, 37], vec![37, 52, 2], vec![39, 0, 15]],
        );
        items_dict.insert(
            "fertilizer-to-water".to_string(),
            vec![
                vec![49, 53, 8],
                vec![0, 11, 42],
                vec![42, 0, 7],
                vec![57, 7, 4],
            ],
        );
        items_dict.insert(
            "water-to-light".to_string(),
            vec![vec![88, 18, 7], vec![18, 25, 70]],
        );
        items_dict.insert(
            "light-to-temperature".to_string(),
            vec![vec![45, 77, 23], vec![81, 45, 19], vec![68, 64, 13]],
        );
        items_dict.insert(
            "temperature-to-humidity".to_string(),
            vec![vec![0, 69, 1], vec![1, 0, 69]],
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

    #[fixture]
    fn example_input_seed_to_soil() -> HashMap<u32, u32> {
        let mut from_seed_to_soil: HashMap<u32, u32> = HashMap::new();
        for index in 0..=49 {
            from_seed_to_soil.insert(index, index);
        }
        for index in 50..=97 {
            from_seed_to_soil.insert(index, index + 2);
        }
        from_seed_to_soil.insert(98, 50);
        from_seed_to_soil.insert(99, 51);
        from_seed_to_soil
    }

    // #[rstest]
    // fn test_setup_map_seed_to_soil(
    //     #[from(example_input_data)] input_data: String,
    //     example_input_seed_to_soil: HashMap<u32, u32>,
    // ) {
    //     match mapping_numbers_hasmap(&input_data, "seed-to-soil") {
    //         Ok(mapping) => {
    //             assert_eq!(mapping, example_input_seed_to_soil)
    //         }
    //         Err(error_message) => {
    //             panic!("oepsie{}", error_message)
    //         }
    //     }
    // }
    #[fixture]
    fn example_input_seeds() -> Vec<&'static str> {
        vec!["79", "14", "55", "13"]
    }

    #[rstest]
    fn test_retrieve_seeds(
        #[from(example_input_data)] input_data: String,
        example_input_seeds: Vec<&str>,
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
