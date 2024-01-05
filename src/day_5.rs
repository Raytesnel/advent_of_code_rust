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
    items_dict: &HashMap<String, Vec<Vec<i64>>>,
    key: &str,
    number: i64,
) -> Result<usize, String> {
    if let Some(entry) = items_dict.get(key) {
        for (index, range) in entry.iter().enumerate() {
            let start = range[1];
            let end = range[1] + range[2] - 1;

            if number >= start && number <= end {
                return Ok(index); // Number is within the range
            }
        }
    }
    println!("nothing found");
    Err("nothing found".to_string()) // Number is not in any range
}
fn get_destination_number(
    items_dict: &HashMap<String, Vec<Vec<i64>>>,
    key: &str,
    number: &i64,
) -> i64 {
    match is_number_in_range(items_dict, key, *number) {
        Ok(number_returned) => {
            if let Some(entry) = items_dict.get(key) {
                let delta = entry[number_returned][0] - entry[number_returned][1];
                return number + delta;
            }
        }
        Err(_) => {
            println!("couldn't find number {:?} in range", number);
            return *number;
        }
    }
    *number
}
fn follow_seed(input_dict: &HashMap<String, Vec<Vec<i64>>>, seed_number: &i64) -> i64 {
    let mut previous_number = *seed_number;
    let mut next_number = 0;

    for seed_item in SEED_ORDER {
        next_number = get_destination_number(input_dict, &seed_item, &previous_number);
        previous_number = next_number;
    }

    println!("loop stopped {:?}", next_number);
    next_number
}

fn retrieve_seeds(line: &str) -> Result<Vec<i64>, &'static str> {
    if let Some(first_line) = line.lines().next() {
        if first_line.contains("seeds") {
            let seed_list: Vec<i64> = first_line
                .split_whitespace()
                .skip(1)
                .map(|item| item.parse::<i64>().unwrap_or_default())
                .collect();
            Ok(seed_list)
        } else {
            Err("not found seeds")
        }
    } else {
        Err("empty string")
    }
}

fn split_mapping_from_line(multiline_string: &String) -> HashMap<String, Vec<Vec<i64>>> {
    let lines = multiline_string.lines();

    let mut block_list: Vec<Vec<i64>> = Vec::new();
    let mut header: String = "".to_string();
    let mut items_dict: HashMap<String, Vec<Vec<i64>>> = HashMap::new();

    for line in lines.skip(2) {
        if line == "" {
            items_dict.insert(header.clone(), block_list.clone());
            header.clear();
            block_list.clear(); // Clear block_list for the next section
        } else if line.contains(" map:") {
            header = line.replace(" map:", "").replace(" ", "");
        } else {
            let temp_items: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            let block_item: Vec<i64> = temp_items
                .iter()
                .map(|item| item.parse::<i64>().unwrap_or_default())
                .collect();
            block_list.push(block_item);
        }
    }
    let mut updated_dict: HashMap<String, Vec<Vec<i64>>> = HashMap::new();
    for (key, values) in items_dict.clone().into_iter() {
        let mut sorted_values = values;
        sorted_values.sort_by_key(|inner_vec| inner_vec[1]);
        updated_dict.insert(key, sorted_values);
    }

    updated_dict
}

fn find_lowest_number(seed_locations: HashMap<i64, i64>) -> Result<i64, String> {
    if let Some(lowest_value) = seed_locations.values().cloned().min() {
        Ok(lowest_value)
    } else {
        Err("Seed locations map is empty.".to_string())
    }
}

pub fn day_5_a(file_contents: String) -> i64 {
    let mapped_input = split_mapping_from_line(&file_contents);

    match retrieve_seeds(&file_contents) {
        Ok(seeds) => {
            let mut seed_locations: HashMap<i64, i64> = HashMap::new();

            for seed in seeds {
                let location = follow_seed(&mapped_input, &seed);
                println!("with given seed:{:?} given location{:?}", seed, location);
                seed_locations.insert(seed, location);
            }

            match find_lowest_number(seed_locations) {
                Ok(value) => {
                    println!("5-A: total number = {:?}", value);
                    value
                }
                Err(_) => panic!("couldn't find the lowest number"),
            }
        }
        Err(e) => panic!("couldn't retrieve seeds, error: {}", e),
    }
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
    fn example_input_split() -> HashMap<String, Vec<Vec<i64>>> {
        let mut items_dict: HashMap<String, Vec<Vec<i64>>> = HashMap::new();
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
        example_input_split: HashMap<String, Vec<Vec<i64>>>,
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
    fn test_if_number_is_in_range(example_input_split: HashMap<String, Vec<Vec<i64>>>) {
        let key_to_find = "seed-to-soil";
        let number: i64 = 60;
        let _ = is_number_in_range(&example_input_split, key_to_find, number);
        let key_to_find = "seed-to-soil";
        let number: i64 = 5;
        match is_number_in_range(&example_input_split, key_to_find, number) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
        let key_to_find = "fertilizer-to-water";
        let number: i64 = 60;
        let number_found = is_number_in_range(&example_input_split, key_to_find, number);
        assert_eq!(number_found.unwrap(), 3);
        let key_to_find = "light-to-temperature";
        let number: i64 = 64;
        let number_found = is_number_in_range(&example_input_split, key_to_find, number);
        assert_eq!(number_found.unwrap(), 1)
    }

    #[rstest]
    fn test_return_destination(example_input_split: HashMap<String, Vec<Vec<i64>>>) {
        let key_to_find = "fertilizer-to-water";
        let number: i64 = 60;
        let new_number = get_destination_number(&example_input_split, key_to_find, &number);
        assert_eq!(new_number, 56);
        let number: i64 = 70;
        let new_number = get_destination_number(&example_input_split, key_to_find, &number);
        assert_eq!(new_number, 70)
    }

    #[fixture]
    fn seed_locations() -> Vec<Vec<i64>> {
        let seed_locations = vec![vec![79, 82], vec![14, 43], vec![55, 86], vec![13, 35]];
        seed_locations
    }

    #[rstest]
    fn test_find_seed_locations(
        seed_locations: Vec<Vec<i64>>,
        example_input_split: HashMap<String, Vec<Vec<i64>>>,
    ) {
        for seed_combi in seed_locations {
            let seed_number = seed_combi[0];
            let result_seed_loctions = follow_seed(&example_input_split, &seed_number);
            assert_eq!(result_seed_loctions, seed_combi[1])
        }
    }

    #[rstest]
    fn test_find_lowest_number() {
        let mut seed_dict: HashMap<i64, i64> = HashMap::new();
        seed_dict.insert(79, 82);
        seed_dict.insert(14, 43);
        seed_dict.insert(55, 86);
        seed_dict.insert(13, 35);
        match find_lowest_number(seed_dict) {
            Ok(value) => assert_eq!(value, 35),
            Err(_) => assert!(false),
        }
    }

    #[rstest]
    fn test_full_day_5_a(#[from(example_input_data)] input_data: String) {
        let lowest_number = day_5_a(input_data);
        assert_eq!(lowest_number, 35)
    }

    #[fixture]
    fn example_input_seeds() -> Vec<i64> {
        vec![79, 14, 55, 13]
    }

    #[rstest]
    fn test_retrieve_seeds(
        #[from(example_input_data)] input_data: String,
        example_input_seeds: Vec<i64>,
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
