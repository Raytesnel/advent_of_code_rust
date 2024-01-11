use maplit::hashmap;
use std::collections::HashMap;
use std::f64;
use std::iter::zip;

fn calculate_min_max_button(time: &f64, distance: f64) -> Result<(f64, f64), String> {
    let discriminant = time * time - 4.0 * distance;
    if discriminant >= 0.0 {
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = ((time) + sqrt_discriminant) / 2.0;
        let root2 = ((time) - sqrt_discriminant) / 2.0;
        Ok((root1, root2))
    } else {
        Err("failed to find the roots".to_string())
    }
}

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

pub fn day_6_a(file_contents: String) -> u32 {
    let input_data = split_input(file_contents);
    let mut total_sum = 1;
    for (time, distance) in input_data {
        let time_input = time as f64;
        let distance_input = distance as f64;
        let (max_result, min_result) =
            calculate_min_max_button(&time_input, distance_input + 1.0).unwrap();
        let delta = (max_result.floor() - min_result.ceil() + 1.0) as u32;
        total_sum = total_sum * delta;
    }
    println!("6-A: total number = {:?}", total_sum);
    total_sum
}

pub fn day_6_b() -> u64 {
    let time = 51699878.0;
    let distance = 377117112241505.0;
    let (max_result, min_result) = calculate_min_max_button(&time, distance + 1.0).unwrap();
    let delta = (max_result.floor() - min_result.ceil() + 1.0) as u64;
    println!("6-B: total number = {:?}", delta);
    delta
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

    #[rstest]
    fn test_min_max_button() {
        let time = 7.0;
        let distance = 9 + 1;
        let min_button = 2;
        let max_button = 5;
        let (max_result, min_result) = calculate_min_max_button(&time, distance as f64).unwrap();

        assert_eq!(min_button, min_result as u32);
        assert_eq!(max_button, max_result as u32);
    }

    #[rstest]
    fn test_sample_input_delta(sample_input_data: HashMap<u32, u32>) {
        let mut results: HashMap<u32, u32> = hashmap! {
            4=>7,
            8=>15,
            9=>30
        };
        let mut total_sum = 1;
        for (result, time_input) in results {
            let distance_input = sample_input_data.get(&time_input).unwrap();
            let distance = *distance_input as f64;
            let time = time_input as f64;
            let (max_result, min_result) = calculate_min_max_button(&time, distance + 1.0).unwrap();
            let delta = (max_result.floor() - min_result.ceil() + 1.0) as u32;
            total_sum = total_sum * delta;
            assert_eq!(delta, result, "testing the delta");
        }
        assert_eq!(total_sum, 288, "testing total sum")
    }

    #[rstest]
    fn test_day_6_b() {
        let time = 71530.0;
        let distance = 940200.0;
        let wins = 71503;
        let (max_result, min_result) = calculate_min_max_button(&time, distance + 1.0).unwrap();
        let delta = (max_result.floor() - min_result.ceil() + 1.0) as u32;
        assert_eq!(wins, delta)
    }
}
