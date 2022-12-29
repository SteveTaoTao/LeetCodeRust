use std::collections::HashMap;

pub fn two_sum_two_for_cycle(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (idx, value) in nums.iter().enumerate() {
        for (idx_inside, value_inside) in nums.iter().enumerate() {
            if (idx != idx_inside) {
                if ((value + value_inside) == target) {
                    return vec![idx as i32, idx_inside as i32];
                }
            }
        }
    }
    vec![]
}

pub fn two_sum_two_for_solution2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let size = nums.len();
    for z in 0..size {
        for y in z + 1..size {
            if nums[z] == target - nums[y] {
                return vec![z as i32, y as i32];
            }
        }
    }
    vec![]
}

pub fn others_smart_opinion_solution3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (index, num) in nums.into_iter().enumerate() {
        if map.contains_key(&num) {
            return vec![map[&num] as i32, index as i32];
        } else {
            map.insert(target - num, index);
        }
    }
    unreachable!();
}

fn main() {
    let input = vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let target = 9;
    // let output = two_sum_two_for_cycle(input, target);
    // let output = two_sum_two_for_cycle(input, target);
    let output = others_smart_opinion_solution3(input, target);

    println!("output: {:?}", output);
}
