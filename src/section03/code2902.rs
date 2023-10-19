#![allow(dead_code)]

// 2902. Count of Sub-Multisets With Bounded Sum
// https://leetcode.com/problems/count-of-sub-multisets-with-bounded-sum/
// https://leetcode.cn/problems/count-of-sub-multisets-with-bounded-sum/
//
// Hard
//
// You are given a 0-indexed array nums of non-negative integers, and two integers l and r.
//
// Return the count of sub-multisets within nums where the sum of elements in each subset falls within the inclusive range of [l, r].
//
// Since the answer may be large, return it modulo 109 + 7.
//
// A sub-multiset is an unordered collection of elements of the array in which a given value x can
// occur 0, 1, ..., occ[x] times, where occ[x] is the number of occurrences of x in the array.
//
// Note that:
//
// Two sub-multisets are the same if sorting both sub-multisets results in identical multisets.
// The sum of an empty multiset is 0.
//
// Example 1:
//
// Input: nums = [1,2,2,3], l = 6, r = 6
// Output: 1
// Explanation: The only subset of nums that has a sum of 6 is {1, 2, 3}.
//
// Example 2:
//
// Input: nums = [2,1,4,2,7], l = 1, r = 5
// Output: 7
// Explanation: The subsets of nums that have a sum within the range [1, 5] are {1}, {2}, {4}, {2, 2}, {1, 2}, {1, 4}, and {1, 2, 2}.
//
// Example 3:
//
// Input: nums = [1,2,1,3,5,2], l = 3, r = 5
// Output: 9
// Explanation: The subsets of nums that have a sum within the range [3, 5]
// are {3}, {5}, {1, 2}, {1, 3}, {2, 2}, {2, 3}, {1, 1, 2}, {1, 1, 3}, and {1, 2, 2}.
//
// Constraints:
//
// 1 <= nums.length <= 2 * 10^4
// 0 <= nums[i] <= 2 * 10^4
// Sum of nums does not exceed 2 * 10^4.
// 0 <= l <= r <= 2 * 10^4
//

struct Solution;

impl Solution {
    pub fn count_sub_multisets(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut mp = std::collections::BTreeMap::<i32, i32>::new();
        let mut count0 = 0i64;
        for a in nums {
            if a == 0 {
                count0 += 1;
            } else {
                *mp.entry(a).or_insert(0) += 1;
            }
        }

        let mut dp = vec![0; 20001];
        let mut sum = 0;
        const MOD: i32 = 1_000_000_007;

        dp[0] = 1;
        for (val, cnt) in &mp {
            for j in (0..=sum).rev() {
                for k in 1..=(*cnt) as usize {
                    let it = j + k * (*val) as usize;
                    dp[it] = (dp[it] + dp[j]) % MOD;
                }
            }
            sum += (*val) as usize * (*cnt) as usize;
        }

        let mut ret = 0;
        for &dp_i in dp.iter().take(r as usize + 1).skip(l as usize) {
            ret = (ret + dp_i) % MOD;
        }

        (ret as i64 * (count0 + 1) % MOD as i64) as _
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 2, 3];
    let l = 6;
    let r = 6;
    let ans = 1;
    assert_eq!(Solution::count_sub_multisets(nums, l, r), ans);

    let nums = vec![2, 1, 4, 2, 7];
    let l = 1;
    let r = 5;
    let ans = 7;
    assert_eq!(Solution::count_sub_multisets(nums, l, r), ans);

    let nums = vec![1, 2, 1, 3, 5, 2];
    let l = 3;
    let r = 5;
    let ans = 9;
    assert_eq!(Solution::count_sub_multisets(nums, l, r), ans);
}
