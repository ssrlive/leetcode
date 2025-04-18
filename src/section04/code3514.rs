#![allow(dead_code)]

// 3514. Number of Unique XOR Triplets II
// https://leetcode.com/problems/number-of-unique-xor-triplets-ii/
// https://leetcode.cn/problems/number-of-unique-xor-triplets-ii/
//
// Medium
//
// You are given an integer array nums.
//
// A XOR triplet is defined as the XOR of three elements nums[i] XOR nums[j] XOR nums[k] where i <= j <= k.
//
// Return the number of unique XOR triplet values from all possible triplets (i, j, k).
//
// Example 1:
//
// Input: nums = [1,3]
//
// Output: 2
//
// Explanation:
//
// The possible XOR triplet values are:
//
//     (0, 0, 0) → 1 XOR 1 XOR 1 = 1
//     (0, 0, 1) → 1 XOR 1 XOR 3 = 3
//     (0, 1, 1) → 1 XOR 3 XOR 3 = 1
//     (1, 1, 1) → 3 XOR 3 XOR 3 = 3
//
// The unique XOR values are {1, 3}. Thus, the output is 2.
//
// Example 2:
//
// Input: nums = [6,7,8,9]
//
// Output: 4
//
// Explanation:
//
// The possible XOR triplet values are {6, 7, 8, 9}. Thus, the output is 4.
//
// Constraints:
//
//     1 <= nums.length <= 1500
//     1 <= nums[i] <= 1500
//

struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let n = nums.len();
        let mut set = HashSet::with_capacity(n * n);
        let mut ans = nums.iter().copied().collect::<HashSet<_>>();

        for (i, &num1) in ans.iter().enumerate() {
            for &num2 in ans.iter().skip(i) {
                set.insert(num1 ^ num2);
            }
        }

        for num1 in nums {
            for num2 in set.iter() {
                ans.insert(num1 ^ num2);
            }
            if ans.len() == 2048 {
                break;
            };
        }

        ans.len() as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::unique_xor_triplets(vec![1, 3]), 2);
    assert_eq!(Solution::unique_xor_triplets(vec![6, 7, 8, 9]), 4);
}
