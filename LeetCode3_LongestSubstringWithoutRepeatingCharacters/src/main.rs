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

pub fn length_of_longest_substring_good(s: String) -> i32 {
    {
        let mut max_len: usize = 0;

        // [1] longest substring is the one with the largest
        //    difference between positions of repeated characters;
        //    thus, we should create a storage for such positions
        let mut pos: [usize; 128] = [0; 128];

        // [2] while iterating through the string (i.e., moving
        //    the end of the sliding window), we should also
        //    update the start of the window
        let mut start: usize = 0;

        for (end, ch) in s.chars().enumerate() {
            // [3] get the position for the start of sliding window
            //    with no other occurences of 'ch' in it
            start = start.max(pos[ch as usize]);

            // [4] update maximum length
            max_len = max_len.max(end - start + 1);

            // [5] set the position to be used in [3] on next iterations
            pos[ch as usize] = end + 1;
        }

        return max_len as i32;
    }
}

fn main() {
    let test_string = "anviaj".to_string();
    let max_length = length_of_longest_substring(test_string);
    println!("max_length: {}\n", max_length);
}
