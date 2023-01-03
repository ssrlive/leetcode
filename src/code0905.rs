#![allow(dead_code)]

// 905. Sort Array By Parity
// https://leetcode.com/problems/sort-array-by-parity/
// https://leetcode.cn/problems/sort-array-by-parity/
//
// Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.
//
// Return any array that satisfies this condition.
//
// Example 1:
//
// Input: nums = [3,1,2,4]
// Output: [2,4,3,1]
// Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
//
// Example 2:
//
// Input: nums = [0]
// Output: [0]
//
// Constraints:
//
// - 1 <= nums.length <= 5000
// - 0 <= nums[i] <= 5000
//

struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .filter(|&x| x % 2 == 0)
            .chain(nums.iter().filter(|&x| x % 2 != 0))
            .copied()
            .collect::<Vec<i32>>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sort_array_by_parity(vec![3, 1, 2, 4]), vec![2, 4, 3, 1]);
    assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
}
