use std::fs;
use std::collections::HashMap;


fn main() {
    let predicted_blocks_dict: HashMap<&'static str, u32> = setup();
    let file_contents = read_file();
    let mut total_score:u32 = 0;
    for (index,line) in file_contents.lines().enumerate(){
        let clean_string = clean_string(line);
        if check_string(&predicted_blocks_dict,clean_string){
            total_score += (index+1) as u32;
            // println!("game is possible")
        }
        else{
            // println!("game is not possible")
        }
    }
    println!("score: {:?}",total_score)

}

fn check_string(predict_dict:&HashMap<&'static str, u32>,cleaned_string:String)->bool{
    let items: Vec<&str> = cleaned_string.split(" ").filter(|&s| !s.is_empty()).collect();
    for chunk in items.chunks(2) {
        if let [number, block] = chunk {
            // println!("this number{:?} and this block{:?}", number, block);
            if let (Ok(parsed_number), Some(&dict_number)) = (
                number.parse::<u32>(),
                predict_dict.get(block)
            ){
                // Successfully parsed, now 'parsed_number' is an i32
                if dict_number < parsed_number{
                    return false
                }
            } else {
                // Failed to parse, handle the error or provide a default value
                println!("Failed to parse as u32");
            }
        }
    }
    return true
}

fn setup()->HashMap<&'static str,u32>{
    let mut predicted_blocks_dict:HashMap<&'static str,u32> = HashMap::new();
    predicted_blocks_dict.insert("red", 12);
    predicted_blocks_dict.insert("blue", 14);
    predicted_blocks_dict.insert("green", 13);
    return predicted_blocks_dict
}

fn clean_string(lines: &str)->String{
    // removes the game session & quotes, so it returns a vector with pairs of 2
    let line_trimmed: String = lines.split(':').nth(1).unwrap_or("").trim().to_string();
    let clean_string = line_trimmed.replace(";","").replace(":","").replace(",","");
    // println!("cleaned string is : {clean_string}");
    return clean_string
}

fn read_file() ->String{
   let file_contents = fs::read_to_string("src/input_data/input_day_2_a.txt")
        .expect("LogRocket: Should have been able to read the file{}");
    return file_contents
}