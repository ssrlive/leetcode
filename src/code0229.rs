#![allow(dead_code)]

// 229. Majority Element II
// https://leetcode.com/problems/majority-element-ii/
// https://leetcode.cn/problems/majority-element-ii/
//
// Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.
//
// Note: The algorithm should run in linear time and in O(1) space.
//
// Example 1:
//
// Input: [3,2,3]
// Output: [3]
//
// Example 2:
//
// Input: [1,1,1,3,3,2,2,2]
// Output: [1,2]
//

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let mut result = Vec::<i32>::new();
        for num in nums.iter() {
            let count = map.entry(*num).or_insert(0);
            *count += 1;
        }
        for (key, value) in map {
            if value > nums.len() / 3 {
                result.push(key);
            }
        }
        result
    }
}

#[test]
fn test_majority_element() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
    let mut v = Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]);
    v.sort();
    assert_eq!(v, vec![1, 2]);
}
