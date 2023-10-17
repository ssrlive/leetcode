#![allow(dead_code)]

// 2834. Find the Minimum Possible Sum of a Beautiful Array
// https://leetcode.com/problems/find-the-minimum-possible-sum-of-a-beautiful-array/
// https://leetcode.cn/problems/find-the-minimum-possible-sum-of-a-beautiful-array/
//
// Medium
//
// You are given positive integers n and target.
//
// An array nums is beautiful if it meets the following conditions:
//
// nums.length == n.
// nums consists of pairwise distinct positive integers.
// There doesn't exist two distinct indices, i and j, in the range [0, n - 1], such that nums[i] + nums[j] == target.
// Return the minimum possible sum that a beautiful array could have modulo 109 + 7.
//
// Example 1:
//
// Input: n = 2, target = 3
// Output: 4
// Explanation: We can see that nums = [1,3] is beautiful.
// - The array nums has length n = 2.
// - The array nums consists of pairwise distinct positive integers.
// - There doesn't exist two distinct indices, i and j, with nums[i] + nums[j] == 3.
// It can be proven that 4 is the minimum possible sum that a beautiful array could have.
//
// Example 2:
//
// Input: n = 3, target = 3
// Output: 8
// Explanation: We can see that nums = [1,3,4] is beautiful.
// - The array nums has length n = 3.
// - The array nums consists of pairwise distinct positive integers.
// - There doesn't exist two distinct indices, i and j, with nums[i] + nums[j] == 3.
// It can be proven that 8 is the minimum possible sum that a beautiful array could have.
//
// Example 3:
//
// Input: n = 1, target = 1
// Output: 1
// Explanation: We can see, that nums = [1] is beautiful.
//
// Constraints:
//
// 1 <= n <= 10^9
// 1 <= target <= 10^9
//

struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn minimum_possible_sum(n: i32, target: i32) -> i64 {
        let n = n as i64;
        let target = target as i64;
        let a = n.min(target / 2);
        let b = n - a;
        (Self::arith_sum(a) + Self::arith_sum(b) + b * (target - 1)) % Self::MOD
    }

    fn arith_sum(i: i64) -> i64 {
        i * (i + 1) / 2 % Self::MOD
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_possible_sum(2, 3), 4);
    assert_eq!(Solution::minimum_possible_sum(3, 3), 8);
    assert_eq!(Solution::minimum_possible_sum(1, 1), 1);
    assert_eq!(Solution::minimum_possible_sum(39636, 49035), 156198582);
}
