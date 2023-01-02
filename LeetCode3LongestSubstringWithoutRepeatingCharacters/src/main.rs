pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;
    let mut max_length: i32 = 0;
    let change_to_bytes = s.as_bytes();
    let mut temp_record: HashMap<u8, usize> = HashMap::new();
    for (index, &item) in change_to_bytes.iter().enumerate() {
        if temp_record.contains_key(&item) {
            let repeat_index = temp_record[&item];
            if temp_record.len() as i32 > max_length {
                max_length = temp_record.len() as i32;
            }
            temp_record.clear();
            for add_index_to_map in repeat_index + 1..index + 1 {
                temp_record.insert(change_to_bytes[add_index_to_map], add_index_to_map);
            }
        } else {
            if index == change_to_bytes.len() - 1 {
                temp_record.insert(item, index);
                if temp_record.len() as i32 > max_length {
                    max_length = temp_record.len() as i32;
                }
            }
        }
        temp_record.insert(item, index);
    }
    max_length
}

fn main() {
    let test_string = "anviaj".to_string();
    let max_length = length_of_longest_substring(test_string);
    println!("max_length: {}\n", max_length);
}
