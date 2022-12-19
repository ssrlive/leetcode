#![allow(dead_code)]

// 169. Majority Element
// https://leetcode.com/problems/majority-element/
// https://leetcode.cn/problems/majority-element/
//
// Given an array of size n, find the majority element.
//
// The majority element is the element that appears more than ⌊ n/2 ⌋ times.
// You may assume that the array is non-empty and the majority element always exist in the array.
//

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
            }
            if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}

#[test]
fn test_majority_element() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
