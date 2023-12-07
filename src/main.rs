use std::fs;

fn main() {
    clean_string_to_list();
}

fn clean_string_to_list(){
    let mut result: Vec<&str> = Vec::new();
    for line in read_file().lines(){
        let line_trimmed: String = line.split(':').nth(1).unwrap_or("").trim().to_string();
        let line_splitted:Vec<&str>=line_trimmed.split("; ").collect();
        let mut numbers_per_game:Vec<i32> = Vec::new();
        numbers_per_game.push(0);
        numbers_per_game.push(0);
        numbers_per_game.push(0);
        for item in line_splitted {
            let sub_result: Vec<&str> = item.split(", ").collect();
            let mut highest_game_number:Vec<i32> = Vec::new();
            for blocks in sub_result{
                let collected_blocks: Vec<&str> =blocks.split(" ").collect();
                let string_number  = collected_blocks.get(0).unwrap().trim();
                if let Ok(parsed_number) = string_number.parse::<i32>() {
                    // Successfully parsed, now 'parsed_number' is an i32
                    highest_game_number.push(parsed_number);
                } else {
                    // Failed to parse, handle the error or provide a default value
                    println!("Failed to parse as i32");
                }
            }
            println!("{:?}",highest_game_number);
            for (index,item) in highest_game_number.iter().enumerate() {
                if highest_game_number.get(index) > numbers_per_game.get(index) {
                    if let Some(value) = highest_game_number.get(index) {
                        let extracted_value: i32 = *value;
                        println!("{:?} is higher then {:?}", highest_game_number.get(index), numbers_per_game.get(index));

                        //replace the item
                        if let Some(existing_value) = numbers_per_game.get_mut(index) {
                            *existing_value = extracted_value;
                            println!("Updated vector: {:?}", numbers_per_game);
                        } else {
                            println!("Index out of bounds");
                        }                    }
                }
            }
            println!("total : {:?}",numbers_per_game)
            // result.extend(sub_result);
        }

    }
}

fn read_file() ->String{
   let file_contents = fs::read_to_string("src/input_data/input_day_2_a.txt")
        .expect("LogRocket: Should have been able to read the file{}");
    return file_contents
}