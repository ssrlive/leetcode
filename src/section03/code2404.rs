#![allow(dead_code)]

// 2404. Most Frequent Even Element
// https://leetcode.com/problems/most-frequent-even-element/
// https://leetcode.cn/problems/most-frequent-even-element/
//
// Given an integer array nums, return the most frequent even element.
//
// If there is a tie, return the smallest one. If there is no such element, return -1.
//
// Example 1:
//
// Input: nums = [0,1,2,2,4,4,1]
// Output: 2
// Explanation:
// The even elements are 0, 2, and 4. Of these, 2 and 4 appear the most.
// We return the smallest one, which is 2.
//
// Example 2:
//
// Input: nums = [4,4,4,9,2,4]
// Output: 4
// Explanation: 4 is the even element appears the most.
//
// Example 3:
//
// Input: nums = [29,47,21,41,13,37,25,7]
// Output: -1
// Explanation: There is no even element.
//
// Constraints:
//
// - 1 <= nums.length <= 2000
// - 0 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::HashMap;

        let mut map = HashMap::new();
        nums.into_iter().filter(|x| x % 2 == 0).for_each(|x| {
            map.entry(x).and_modify(|x| *x += 1).or_insert(0);
        });

        map.into_iter().max_by_key(|&(k, v)| (v, Reverse(k))).unwrap_or((-1, 0)).0
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![0, 1, 2, 2, 4, 4, 1], 2),
        (vec![4, 4, 4, 9, 2, 4], 4),
        (vec![29, 47, 21, 41, 13, 37, 25, 7], -1),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::most_frequent_even(nums), expected);
    }
}
