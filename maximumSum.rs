use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter().fold((i32::MIN, 0), |(max_sum, cur_sum), &num| {
            let new_cur_sum = max(cur_sum + num, num);
            (max(max_sum, new_cur_sum), new_cur_sum)
        }).0
    }
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = Solution::max_sub_array(nums);
    println!("The maximum sum of a contiguous subarray is {}", result);
}
