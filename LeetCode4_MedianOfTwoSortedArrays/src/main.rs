struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut a = nums1.clone();
        let mut b = nums2.clone();
        a.append(&mut b);
        a.sort();
        print!("a = {:?} \n", a);
        if a.len() % 2 != 0 {
            a[a.len() / 2] as f64
        } else {
            let v1 = (a[a.len() / 2] + a[a.len() / 2 - 1]) as f64;
            v1 / 2.0
        }
    }
}

fn main() {
    let nums1: Vec<i32> = vec![1, 2, 3, 4];
    let nums2: Vec<i32> = vec![3, 4, 5, 6];
    let get_median_sorted_arrays = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("get_median_sorted_arrays {} \n", get_median_sorted_arrays);
}
