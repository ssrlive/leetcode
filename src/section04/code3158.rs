#![allow(dead_code)]

// 3158. Find the XOR of Numbers Which Appear Twice
// https://leetcode.com/problems/find-the-xor-of-numbers-which-appear-twice/
// https://leetcode.cn/problems/find-the-xor-of-numbers-which-appear-twice/
//
// Easy
//
// You are given an array nums, where each number in the array appears either once or twice.
//
// Return the bitwise XOR of all the numbers that appear twice in the array, or 0 if no number appears twice.
//
// Example 1:
//
// Input: nums = [1,2,1,3]
//
// Output: 1
//
// Explanation:
//
// The only number that appears twice in nums is 1.
//
// Example 2:
//
// Input: nums = [1,2,3]
//
// Output: 0
//
// Explanation:
//
// No number appears twice in nums.
//
// Example 3:
//
// Input: nums = [1,2,2,1]
//
// Output: 3
//
// Explanation:
//
// Numbers 1 and 2 appeared twice. 1 XOR 2 == 3.
//
// Constraints:
//
// 1 <= nums.length <= 50
// 1 <= nums[i] <= 50
// Each number in nums appears either once or twice.
//

struct Solution;

impl Solution {
    fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(std::collections::HashMap::with_capacity(50), |mut map, x| {
                *map.entry(x).or_insert(0) += 1;
                map
            })
            .iter()
            .filter(|(_, x)| x.eq(&&2))
            .fold(0x0, |ret, (x, _)| ret ^ *x)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
}
