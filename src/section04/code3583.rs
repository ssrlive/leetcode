#![allow(dead_code)]

// 3583. Count Special Triplets
// https://leetcode.com/problems/count-special-triplets/
// https://leetcode.cn/problems/count-special-triplets/
//
// Medium
//
// You are given an integer array nums.
//
// A special triplet is defined as a triplet of indices (i, j, k) such that:
//
//     0 <= i < j < k < n, where n = nums.length
//     nums[i] == nums[j] * 2
//     nums[k] == nums[j] * 2
//
// Return the total number of special triplets in the array.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [6,3,6]
//
// Output: 1
//
// Explanation:
//
// The only special triplet is (i, j, k) = (0, 1, 2), where:
//
//     nums[0] = 6, nums[1] = 3, nums[2] = 6
//     nums[0] = nums[1] * 2 = 3 * 2 = 6
//     nums[2] = nums[1] * 2 = 3 * 2 = 6
//
// Example 2:
//
// Input: nums = [0,1,0,0]
//
// Output: 1
//
// Explanation:
//
// The only special triplet is (i, j, k) = (0, 2, 3), where:
//
//     nums[0] = 0, nums[2] = 0, nums[3] = 0
//     nums[0] = nums[2] * 2 = 0 * 2 = 0
//     nums[3] = nums[2] * 2 = 0 * 2 = 0
//
// Example 3:
//
// Input: nums = [8,4,2,8,4]
//
// Output: 2
//
// Explanation:
//
// There are exactly two special triplets:
//
//     (i, j, k) = (0, 1, 3)
//         nums[0] = 8, nums[1] = 4, nums[3] = 8
//         nums[0] = nums[1] * 2 = 4 * 2 = 8
//         nums[3] = nums[1] * 2 = 4 * 2 = 8
//     (i, j, k) = (1, 2, 4)
//         nums[1] = 4, nums[2] = 2, nums[4] = 4
//         nums[1] = nums[2] * 2 = 2 * 2 = 4
//         nums[4] = nums[2] * 2 = 2 * 2 = 4
//
// Constraints:
//
//     3 <= n == nums.length <= 10^5
//     0 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = nums.len();
        let mut result = 0i64;
        let mut r = std::collections::HashMap::new();
        let mut l = std::collections::HashMap::new();

        for &val in &nums {
            *r.entry(val).or_insert(0) += 1;
        }

        for &nums_j in nums.iter().take(n) {
            let mid = nums_j;
            *r.entry(mid).or_insert(0) -= 1;
            let left = *l.get(&(2 * mid)).unwrap_or(&0);
            let right = *r.get(&(2 * mid)).unwrap_or(&0);
            result = (result + left as i64 * right as i64) % MOD as i64;
            *l.entry(mid).or_insert(0) += 1;
        }

        (result % MOD as i64) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::special_triplets(vec![6, 3, 6]), 1);
    assert_eq!(Solution::special_triplets(vec![0, 1, 0, 0]), 1);
    assert_eq!(Solution::special_triplets(vec![8, 4, 2, 8, 4]), 2);
}
