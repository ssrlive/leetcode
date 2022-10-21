#![allow(dead_code)]

// 137. Single Number II
// https://leetcode.com/problems/single-number-ii/
//
// Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
//
// You must implement a solution with a linear runtime complexity and use only constant extra space.
//

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        for &x in nums.iter() {
            ones = (ones ^ x) & !twos;
            twos = (twos ^ x) & !ones;
        }
        ones
    }
}

#[test]
fn test_single_number() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}
