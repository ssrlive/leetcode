#![allow(dead_code)]

// 525. Contiguous Array
// https://leetcode.com/problems/contiguous-array/
//
// Given a binary array nums, return the maximum length of a contiguous subarray with an equal number of 0 and 1.
//
// Example 1:
//
// Input: nums = [0,1]
// Output: 2
// Explanation: [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.
//
// Example 2:
//
// Input: nums = [0,1,0]
// Output: 2
// Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - nums[i] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut hm = std::collections::HashMap::<i32, i32>::new();
        hm.insert(0, 0);
        let mut answer = 0;
        let mut n = 0;
        for (i, &num) in (0..).zip(nums.iter()) {
            n += if num == 1 { 1 } else { -1 };
            if let Some(j) = hm.get(&n) {
                answer = std::cmp::max(answer, i - j + 1);
            } else {
                hm.insert(n, i + 1);
            }
        }
        answer
    }
}

#[test]
fn test_find_max_length() {
    assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
}
