#![allow(dead_code)]

/*

// 1498. Number of Subsequences That Satisfy the Given Sum Condition
// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/
// https://leetcode.cn/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/
//
// Medium
//
// You are given an array of integers nums and an integer target.

Return the number of non-empty subsequences of nums such that the sum of the minimum and maximum element on it is less or equal to target. Since the answer may be too large, return it modulo 109 + 7.

Example 1:

Input: nums = [3,5,6,7], target = 9
Output: 4
Explanation: There are 4 subsequences that satisfy the condition.
[3] -> Min value + max value <= target (3 + 3 <= 9)
[3,5] -> (3 + 5 <= 9)
[3,5,6] -> (3 + 6 <= 9)
[3,6] -> (3 + 6 <= 9)

Example 2:

Input: nums = [3,3,6,8], target = 10
Output: 6
Explanation: There are 6 subsequences that satisfy the condition. (nums can have repeated numbers).
[3] , [3] , [3,3], [3,6] , [3,6] , [3,3,6]

Example 3:

Input: nums = [2,3,3,4,6,7], target = 12
Output: 61
Explanation: There are 63 non-empty subsequences, two of them do not satisfy the condition ([6,7], [7]).
Number of valid subsequences (63 - 2 = 61).

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^6
    1 <= target <= 10^6
*/

struct Solution;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        nums.sort();
        let mut res = 0;
        let n = nums.len();
        let mut l = 0;
        let mut r = n as i64 - 1;
        let i_mod = 1_000_000_007_i64;
        let mut pows = vec![1; n];
        for i in 1..n {
            pows[i] = pows[i - 1] * 2 % i_mod;
        }
        while l <= r {
            if nums[l as usize] + nums[r as usize] > target as i64 {
                r -= 1;
            } else {
                res = (res + pows[(r - l) as usize]) % i_mod;
                l += 1;
            }
        }
        res as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_subseq(vec![1], 1), 0);
    assert_eq!(Solution::num_subseq(vec![3, 5, 6, 7], 9), 4);
    assert_eq!(Solution::num_subseq(vec![3, 3, 6, 8], 10), 6);
    assert_eq!(Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
}
