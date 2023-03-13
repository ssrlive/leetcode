#![allow(dead_code)]

/*

// 1673. Find the Most Competitive Subsequence
// https://leetcode.com/problems/find-the-most-competitive-subsequence/
// https://leetcode.cn/problems/find-the-most-competitive-subsequence/
//
// Medium
//
// Given an integer array nums and a positive integer k, return the most competitive subsequence of nums of size k.

An array's subsequence is a resulting sequence obtained by erasing some (possibly zero) elements from the array.

We define that a subsequence a is more competitive than a subsequence b (of the same length) if in the first position where a and b differ, subsequence a has a number less than the corresponding number in b. For example, [1,3,4] is more competitive than [1,3,5] because the first position they differ is at the final number, and 4 is less than 5.

Example 1:

Input: nums = [3,5,2,6], k = 2
Output: [2,6]
Explanation: Among the set of every possible subsequence: {[3,5], [3,2], [3,6], [5,2], [5,6], [2,6]}, [2,6] is the most competitive.

Example 2:

Input: nums = [2,4,3,3,5,4,9,6], k = 4
Output: [2,3,3,4]

Constraints:

    1 <= nums.length <= 10^5
    0 <= nums[i] <= 10^9
    1 <= k <= nums.length
*/

struct Solution;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut stack = vec![];
        let k = k as usize;
        let n = nums.len();
        for (i, &num) in nums.iter().enumerate() {
            while !stack.is_empty() && stack[stack.len() - 1] > num && stack.len() - 1 + n - i >= k {
                stack.pop();
            }
            if stack.len() < k {
                stack.push(num);
            }
        }
        stack
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 5, 2, 6], 2, vec![2, 6]),
        (vec![2, 4, 3, 3, 5, 4, 9, 6], 4, vec![2, 3, 3, 4]),
    ];
    for (nums, k, expect) in cases {
        assert_eq!(Solution::most_competitive(nums, k), expect);
    }
}
