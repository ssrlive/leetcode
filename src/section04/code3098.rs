#![allow(dead_code)]

// 3098. Find the Sum of Subsequence Powers
// https://leetcode.com/problems/find-the-sum-of-subsequence-powers/
// https://leetcode.cn/problems/find-the-sum-of-subsequence-powers/
//
// Hard
//
// You are given an integer array nums of length n, and a positive integer k.
//
// The power of a subsequence is defined as the minimum absolute difference between any two elements in the subsequence.
//
// Return the sum of powers of all subsequences of nums which have length equal to k.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [1,2,3,4], k = 3
//
// Output: 4
//
// Explanation:
//
// There are 4 subsequences in nums which have length 3: [1,2,3], [1,3,4], [1,2,4], and [2,3,4].
// The sum of powers is |2 - 3| + |3 - 4| + |2 - 1| + |3 - 4| = 4.
//
// Example 2:
//
// Input: nums = [2,2], k = 2
//
// Output: 0
//
// Explanation:
//
// The only subsequence in nums which has length 2 is [2,2]. The sum of powers is |2 - 2| = 0.
//
// Example 3:
//
// Input: nums = [4,3,-1], k = 2
//
// Output: 10
//
// Explanation:
//
// There are 3 subsequences in nums which have length 2: [4,3], [4,-1], and [3,-1].
// The sum of powers is |4 - 3| + |4 - (-1)| + |3 - (-1)| = 10.
//
// Constraints:
//
//     2 <= n == nums.length <= 50
//     -10^8 <= nums[i] <= 10^8
//     2 <= k <= n
//

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn sum_of_powers(nums: Vec<i32>, k: i32) -> i32 {
        let mod_val = 1_000_000_007;
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut map = HashMap::new();
        Self::dfs(0, k, i32::MAX, -1, &nums, n, &mut map, mod_val)
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(start: usize, k: i32, energy: i32, last: i32, nums: &Vec<i32>, n: usize, map: &mut HashMap<i64, i32>, mod_val: i32) -> i32 {
        if k == 0 {
            return energy;
        }
        let key = ((energy as i64) << 18) | ((start as i64) << 12) | ((k as i64) << 6) | ((last + 1) as i64);
        if let Some(&val) = map.get(&key) {
            return val;
        }
        let mut ans = 0;
        for i in start..=n - k as usize {
            let new_energy = if last == -1 {
                energy
            } else {
                energy.min(nums[i] - nums[last as usize])
            };
            ans = (ans + Self::dfs(i + 1, k - 1, new_energy, i as i32, nums, n, map, mod_val)) % mod_val;
        }
        map.insert(key, ans);
        ans
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let k = 3;
    let res = 4;
    assert_eq!(Solution::sum_of_powers(nums, k), res);

    let nums = vec![2, 2];
    let k = 2;
    let res = 0;
    assert_eq!(Solution::sum_of_powers(nums, k), res);

    let nums = vec![4, 3, -1];
    let k = 2;
    let res = 10;
    assert_eq!(Solution::sum_of_powers(nums, k), res);

    let nums = vec![4, 3, 2, 3];
    let k = 2;
    let res = 6;
    assert_eq!(Solution::sum_of_powers(nums, k), res);
}
