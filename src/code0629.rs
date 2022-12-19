#![allow(dead_code)]

// 629. K Inverse Pairs Array
// https://leetcode.com/problems/k-inverse-pairs-array/
// https://leetcode.cn/problems/k-inverse-pairs-array/
//
// For an integer array nums, an inverse pair is a pair of integers [i, j] where 0 <= i < j < nums.length and nums[i] > nums[j].
//
// Given two integers n and k, return the number of different arrays consist of numbers from 1 to n
// such that there are exactly k inverse pairs. Since the answer can be huge, return it modulo 10^9 + 7.
//
// Example 1:
//
// Input: n = 3, k = 0
// Output: 1
// Explanation: Only the array [1,2,3] which consists of numbers from 1 to 3 has exactly 0 inverse pairs.
//
// Example 2:
//
// Input: n = 3, k = 1
// Output: 2
// Explanation: The array [1,3,2] and [2,1,3] have exactly 1 inverse pair.
//
// Constraints:
//
// 1 <= n <= 1000
// 0 <= k <= 1000
//

struct Solution;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MODE: i32 = 1_000_000_007;
        let mut pre: Vec<i32> = vec![0; k as usize + 1];
        for i in 1..=n {
            let mut cur: Vec<i32> = vec![0; k as usize + 1];
            cur[0] = 1;
            for j in 1..=k {
                let cnt = (pre[j as usize] + MODE - if j - i >= 0 { pre[(j - i) as usize] } else { 0 }) % MODE;
                cur[j as usize] = (cnt + cur[j as usize - 1]) % MODE;
            }
            pre = cur;
        }
        (pre[k as usize] + MODE - if k > 0 { pre[k as usize - 1] } else { 0 }) % MODE
    }
}

#[test]
fn test() {
    assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
    assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
}
