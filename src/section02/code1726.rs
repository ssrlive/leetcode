#![allow(dead_code)]

/*

// 1726. Tuple with Same Product
Medium
533
25
Companies

Given an array nums of distinct positive integers, return the number of tuples (a, b, c, d) such that a * b = c * d where a, b, c, and d are elements of nums, and a != b != c != d.

Example 1:

Input: nums = [2,3,4,6]
Output: 8
Explanation: There are 8 valid tuples:
(2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
(3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)

Example 2:

Input: nums = [1,2,4,5,10]
Output: 16
Explanation: There are 16 valid tuples:
(1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
(2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
(2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,5,4)
(4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)

Constraints:

    1 <= nums.length <= 1000
    1 <= nums[i] <= 10^4
    All elements in nums are distinct.
*/

struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let product = nums[i] * nums[j];
                let entry = map.entry(product).or_insert(0);
                *entry += 1;
            }
        }
        for (_, v) in map {
            count += v * (v - 1) / 2 * 8;
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6]), 8);
    assert_eq!(Solution::tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
}
