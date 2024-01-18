#![allow(dead_code)]

// 3005. Count Elements With Maximum Frequency
// https://leetcode.com/problems/count-elements-with-maximum-frequency/
// https://leetcode.cn/problems/count-elements-with-maximum-frequency/
//
// Easy
//
// You are given an array nums consisting of positive integers.
//
// Return the total frequencies of elements in nums such that those elements all have the maximum frequency.
//
// The frequency of an element is the number of occurrences of that element in the array.
//
// Example 1:
//
// Input: nums = [1,2,2,3,1,4]
// Output: 4
// Explanation: The elements 1 and 2 have a frequency of 2 which is the maximum frequency in the array.
// So the number of elements in the array with maximum frequency is 4.
//
// Example 2:
//
// Input: nums = [1,2,3,4,5]
// Output: 5
// Explanation: All elements of the array have a frequency of 1 which is the maximum.
// So the number of elements in the array with maximum frequency is 5.
//
// Constraints:
//
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let map_nums = nums.iter().fold(std::collections::HashMap::new(), |mut acc, num| {
            acc.entry(num).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

        let max_frequency = *map_nums.values().max().unwrap();
        let max_frequency_count = map_nums.values().filter(|&&x| x == max_frequency).count() as i32;

        max_frequency * max_frequency_count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
    assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
}
