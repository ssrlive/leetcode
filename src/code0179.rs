#![allow(dead_code)]

// 179. Largest Number
// https://leetcode.com/problems/largest-number/
// https://leetcode.cn/problems/largest-number/
//
// Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.
//
// Since the result may be very large, so you need to return a string instead of an integer.
//

struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_by(|a, b| {
            let a = a.to_string();
            let b = b.to_string();
            let ab = a.clone() + &b;
            let ba = b + &a;
            ba.cmp(&ab)
        });
        let mut result = nums.iter().map(|x| x.to_string()).collect::<String>();
        if result.starts_with('0') {
            result = "0".to_string();
        }
        result
    }
}

#[test]
fn test_largest_number() {
    assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
    assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330".to_string());
    assert_eq!(Solution::largest_number(vec![1]), "1".to_string());
    assert_eq!(Solution::largest_number(vec![10]), "10".to_string());
    assert_eq!(Solution::largest_number(vec![0, 0]), "0".to_string());
}
