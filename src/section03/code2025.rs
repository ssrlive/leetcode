#![allow(dead_code)]

/*

// 2025. Maximum Number of Ways to Partition an Array
// https://leetcode.com/problems/maximum-number-of-ways-to-partition-an-array/
// https://leetcode.cn/problems/maximum-number-of-ways-to-partition-an-array/
//
// Hard
//
// You are given a 0-indexed integer array nums of length n. The number of ways to partition nums is the number of pivot indices that satisfy both conditions:

    1 <= pivot < n
    nums[0] + nums[1] + ... + nums[pivot - 1] == nums[pivot] + nums[pivot + 1] + ... + nums[n - 1]

You are also given an integer k. You can choose to change the value of one element of nums to k, or to leave the array unchanged.

Return the maximum possible number of ways to partition nums to satisfy both conditions after changing at most one element.

Example 1:

Input: nums = [2,-1,2], k = 3
Output: 1
Explanation: One optimal approach is to change nums[0] to k. The array becomes [3,-1,2].
There is one way to partition the array:
- For pivot = 2, we have the partition [3,-1 | 2]: 3 + -1 == 2.

Example 2:

Input: nums = [0,0,0], k = 1
Output: 2
Explanation: The optimal approach is to leave the array unchanged.
There are two ways to partition the array:
- For pivot = 1, we have the partition [0 | 0,0]: 0 == 0 + 0.
- For pivot = 2, we have the partition [0,0 | 0]: 0 + 0 == 0.

Example 3:

Input: nums = [22,4,-25,-20,-15,15,-16,7,19,-10,0,-13,-14], k = -33
Output: 4
Explanation: One optimal approach is to change nums[2] to k. The array becomes [22,4,-33,-20,-15,15,-16,7,19,-10,0,-13,-14].
There are four ways to partition the array.

Constraints:

    n == nums.length
    2 <= n <= 10^5
    -10^5 <= k, nums[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let k = k as i64;
        let n = nums.len();
        let mut pref = vec![0; n];
        let mut suff = vec![0; n];
        pref[0] = nums[0];
        suff[n - 1] = nums[n - 1];
        for i in 1..n {
            pref[i] = pref[i - 1] + nums[i];
            suff[n - 1 - i] = suff[n - i] + nums[n - 1 - i];
        }
        let mut ans = 0;
        let mut left = std::collections::HashMap::new();
        let mut right = std::collections::HashMap::new();
        for i in 0..n - 1 {
            *right.entry(pref[i] - suff[i + 1]).or_insert(0) += 1;
        }
        if right.contains_key(&0) {
            ans = *right.get(&0).unwrap();
        }
        for i in 0..n {
            let mut curr = 0;
            let diff = k - nums[i];
            if left.contains_key(&diff) {
                curr += *left.get(&diff).unwrap();
            }
            if right.contains_key(&-diff) {
                curr += *right.get(&-diff).unwrap();
            }
            ans = ans.max(curr);
            if i < n - 1 {
                let dd = pref[i] - suff[i + 1];
                *left.entry(dd).or_insert(0) += 1;
                *right.entry(dd).or_insert(0) -= 1;
                if *right.get(&dd).unwrap() == 0 {
                    right.remove(&dd);
                }
            }
        }
        ans as _
    }
}

#[test]
fn test() {
    let nums = vec![2, -1, 2];
    let k = 3;
    let res = 1;
    assert_eq!(Solution::ways_to_partition(nums, k), res);
    let nums = vec![0, 0, 0];
    let k = 1;
    let res = 2;
    assert_eq!(Solution::ways_to_partition(nums, k), res);
    let nums = vec![22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14];
    let k = -33;
    let res = 4;
    assert_eq!(Solution::ways_to_partition(nums, k), res);
}
