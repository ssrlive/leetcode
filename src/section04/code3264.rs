#![allow(dead_code)]

// 3264. Final Array State After K Multiplication Operations I
// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/
// https://leetcode.cn/problems/final-array-state-after-k-multiplication-operations-i/
//
// Easy
//
// You are given an integer array nums, an integer k, and an integer multiplier.
//
// You need to perform k operations on nums. In each operation:
//
//     Find the minimum value x in nums. If there are multiple occurrences of the minimum value, select the one that appears first.
//     Replace the selected minimum value x with x * multiplier.
//
// Return an integer array denoting the final state of nums after performing all k operations.
//
// Example 1:
//
// Input: nums = [2,1,3,5,6], k = 5, multiplier = 2
//
// Output: [8,4,6,5,6]
//
// Explanation:
// Operation	Result
// After operation 1	[2, 2, 3, 5, 6]
// After operation 2	[4, 2, 3, 5, 6]
// After operation 3	[4, 4, 3, 5, 6]
// After operation 4	[4, 4, 6, 5, 6]
// After operation 5	[8, 4, 6, 5, 6]
//
// Example 2:
//
// Input: nums = [1,2], k = 3, multiplier = 4
//
// Output: [16,8]
//
// Explanation:
// Operation	Result
// After operation 1	[4, 2]
// After operation 2	[4, 8]
// After operation 3	[16, 8]
//
// Constraints:
//
//     1 <= nums.length <= 100
//     1 <= nums[i] <= 100
//     1 <= k <= 10
//     1 <= multiplier <= 5
//

struct Solution;

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        let len = nums.len();
        (0..k)
            .fold(
                nums.into_iter()
                    .zip(0..)
                    .map(Reverse)
                    .collect::<std::collections::BinaryHeap<Reverse<(i32, usize)>>>(),
                |mut indexed, _| {
                    if let Some(mut inner) = indexed.peek_mut() {
                        inner.0.0 *= multiplier;
                    }
                    indexed
                },
            )
            .into_iter()
            .fold(vec![0i32; len], |mut muls, Reverse((v, i))| {
                muls[i] = v;
                muls
            })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2), vec![8, 4, 6, 5, 6]);
    assert_eq!(Solution::get_final_state(vec![1, 2], 3, 4), vec![16, 8]);
}
