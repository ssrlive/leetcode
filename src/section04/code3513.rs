#![allow(dead_code)]

// 3513. Number of Unique XOR Triplets I
// https://leetcode.com/problems/number-of-unique-xor-triplets-i/
// https://leetcode.cn/problems/number-of-unique-xor-triplets-i/
//
// Medium
//
// You are given an integer array nums of length n, where nums is a
//
// of the numbers in the range [1, n].
//
// A XOR triplet is defined as the XOR of three elements nums[i] XOR nums[j] XOR nums[k] where i <= j <= k.
//
// Return the number of unique XOR triplet values from all possible triplets (i, j, k).
//
// Example 1:
//
// Input: nums = [1,2]
//
// Output: 2
//
// Explanation:
//
// The possible XOR triplet values are:
//
//     (0, 0, 0) → 1 XOR 1 XOR 1 = 1
//     (0, 0, 1) → 1 XOR 1 XOR 2 = 2
//     (0, 1, 1) → 1 XOR 2 XOR 2 = 1
//     (1, 1, 1) → 2 XOR 2 XOR 2 = 2
//
// The unique XOR values are {1, 2}, so the output is 2.
//
// Example 2:
//
// Input: nums = [3,1,2]
//
// Output: 4
//
// Explanation:
//
// The possible XOR triplet values include:
//
//     (0, 0, 0) → 3 XOR 3 XOR 3 = 3
//     (0, 0, 1) → 3 XOR 3 XOR 1 = 1
//     (0, 0, 2) → 3 XOR 3 XOR 2 = 2
//     (0, 1, 2) → 3 XOR 1 XOR 2 = 0
//
// The unique XOR values are {0, 1, 2, 3}, so the output is 4.
//
// Constraints:
//
//     1 <= n == nums.length <= 10^5
//     1 <= nums[i] <= n
//     nums is a permutation of integers from 1 to n.
//

struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 4 {
            n.next_power_of_two() as i32
        } else {
            (n + 1).next_power_of_two() as i32
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::unique_xor_triplets(vec![1, 2]), 2);
    assert_eq!(Solution::unique_xor_triplets(vec![3, 1, 2]), 4);
}
