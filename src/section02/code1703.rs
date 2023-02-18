#![allow(dead_code)]

/*

// 1703. Minimum Adjacent Swaps for K Consecutive Ones
Hard
573
21
Companies

You are given an integer array, nums, and an integer k. nums comprises of only 0's and 1's. In one move, you can choose two adjacent indices and swap their values.

Return the minimum number of moves required so that nums has k consecutive 1's.

Example 1:

Input: nums = [1,0,0,1,0,1], k = 2
Output: 1
Explanation: In 1 move, nums could be [1,0,0,0,1,1] and have 2 consecutive 1's.

Example 2:

Input: nums = [1,0,0,0,0,0,1,1], k = 3
Output: 5
Explanation: In 5 moves, the leftmost 1 can be shifted right until nums = [0,0,0,0,0,1,1,1].

Example 3:

Input: nums = [1,1,0,1], k = 2
Output: 0
Explanation: nums already has 2 consecutive 1's.

Constraints:

    1 <= nums.length <= 10^5
    nums[i] is 0 or 1.
    1 <= k <= sum(nums)
*/

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut a = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if num == 1 {
                a.push(i as i64);
            }
        }
        let mut b = vec![0];
        for &num in a.iter() {
            b.push(b.last().unwrap() + num);
        }
        let mut res = 2e9 as i64;
        for i in 0..a.len() - k + 1 {
            res = res.min(b[i + k] - b[k / 2 + i] - b[(k + 1) / 2 + i] + b[i]);
        }
        res -= ((k / 2) * ((k + 1) / 2)) as i64;
        res as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 0, 0, 1, 0, 1], 2, 1),
        (vec![1, 0, 0, 0, 0, 0, 1, 1], 3, 5),
        (vec![1, 1, 0, 1], 2, 0),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::min_moves(nums, k), expected);
    }
}
