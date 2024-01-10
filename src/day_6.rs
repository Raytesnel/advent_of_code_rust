use maplit::hashmap;
use std::collections::HashMap;
use std::iter::zip;

fn split_input(input_data: String) -> HashMap<u32, u32> {
    let mut lines = input_data.lines();
    let mut dict_input = HashMap::new();
    let time: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|item| item.parse::<u32>().unwrap_or_default())
        .collect();
    let distance: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|item| item.parse::<u32>().unwrap_or_default())
        .collect();
    for (time_item, distance_item) in zip(time, distance) {
        dict_input.insert(time_item, distance_item);
    }
    dict_input
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::collections::HashMap;

    #[fixture]
    fn sample_data() -> String {
        r#"Time:      7  15   30
Distance:  9  40  200"#
            .to_string()
    }
    #[fixture]
    fn sample_input_data() -> HashMap<u32, u32> {
        let input_values: HashMap<u32, u32> = hashmap! {
            7=>9,
            15=>40,
            30=>200

        };
        input_values
    }

    #[rstest]
    fn test_split_to_sets(
        sample_data: String,
        #[from(sample_input_data)] expected_result: HashMap<u32, u32>,
    ) {
        let result = split_input(sample_data);
        assert_eq!(result, expected_result);
    }
}
