#![allow(dead_code)]

// 3578. Count Partitions With Max-Min Difference at Most K
// https://leetcode.com/problems/count-partitions-with-max-min-difference-at-most-k/
// https://leetcode.cn/problems/count-partitions-with-max-min-difference-at-most-k/
//
// Medium
//
// You are given an integer array nums and an integer k. Your task is to partition nums into one or more non-empty contiguous segments such that in each segment, the difference between its maximum and minimum elements is at most k.
//
// Return the total number of ways to partition nums under this condition.
//
// Since the answer may be too large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [9,4,1,3,7], k = 4
//
// Output: 6
//
// Explanation:
//
// There are 6 valid partitions where the difference between the maximum and minimum elements in each segment is at most k = 4:
//
//     [[9], [4], [1], [3], [7]]
//     [[9], [4], [1], [3, 7]]
//     [[9], [4], [1, 3], [7]]
//     [[9], [4, 1], [3], [7]]
//     [[9], [4, 1], [3, 7]]
//     [[9], [4, 1, 3], [7]]
//
// Example 2:
//
// Input: nums = [3,3,4], k = 0
//
// Output: 2
//
// Explanation:
//
// There are 2 valid partitions that satisfy the given conditions:
//
//     [[3], [3], [4]]
//     [[3, 3], [4]]
//
// Constraints:
//
//     2 <= nums.length <= 5 * 104
//     1 <= nums[i] <= 109
//     0 <= k <= 109
//

struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = nums.len();
        let mut dp = vec![0; n + 1];
        let mut ps = vec![0; n + 1];
        dp[0] = 1;
        ps[0] = 1;

        let mut st = std::collections::BTreeMap::new();
        let mut j = 0;

        for i in 1..=n {
            *st.entry(nums[i - 1]).or_insert(0) += 1;

            // Shrink window from left if condition is violated
            while st.keys().last().unwrap() - st.keys().next().unwrap() > k {
                let count = st.get_mut(&nums[j]).unwrap();
                *count -= 1;
                if *count == 0 {
                    st.remove(&nums[j]);
                }
                j += 1;
            }

            let prev = if j > 0 { ps[j - 1] } else { 0 };
            dp[i] = (ps[i - 1] - prev + MOD) % MOD;
            ps[i] = (ps[i - 1] + dp[i]) % MOD;
        }

        dp[n]
    }
}

#[test]
fn test() {
    let nums = vec![9, 4, 1, 3, 7];
    let k = 4;
    assert_eq!(Solution::count_partitions(nums, k), 6);

    let nums = vec![3, 3, 4];
    let k = 0;
    assert_eq!(Solution::count_partitions(nums, k), 2);
}
