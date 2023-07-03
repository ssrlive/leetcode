#![allow(dead_code)]

/*

// 2059. Minimum Operations to Convert Number
// https://leetcode.com/problems/minimum-operations-to-convert-number/
// https://leetcode.cn/problems/minimum-operations-to-convert-number/
//
// Medium
//
// You are given a 0-indexed integer array nums containing distinct numbers, an integer start, and an integer goal.
// There is an integer x that is initially set to start, and you want to perform operations on x such that it is converted to goal.
// You can perform the following operation repeatedly on the number x:

If 0 <= x <= 1000, then for any index i in the array (0 <= i < nums.length), you can set x to any of the following:

    x + nums[i]
    x - nums[i]
    x ^ nums[i] (bitwise-XOR)

Note that you can use each nums[i] any number of times in any order. Operations that set x to be out of the range 0 <= x <= 1000 are valid, but no more operations can be done afterward.

Return the minimum number of operations needed to convert x = start into goal, and -1 if it is not possible.

Example 1:

Input: nums = [2,4,12], start = 2, goal = 12
Output: 2
Explanation: We can go from 2 → 14 → 12 with the following 2 operations.
- 2 + 12 = 14
- 14 - 2 = 12

Example 2:

Input: nums = [3,5,7], start = 0, goal = -4
Output: 2
Explanation: We can go from 0 → 3 → -4 with the following 2 operations.
- 0 + 3 = 3
- 3 - 7 = -4
Note that the last operation sets x out of the range 0 <= x <= 1000, which is valid.

Example 3:

Input: nums = [2,8,16], start = 0, goal = 1
Output: -1
Explanation: There is no way to convert 0 into 1.

Constraints:

    1 <= nums.length <= 1000
    -10^9 <= nums[i], goal <= 10^9
    0 <= start <= 1000
    start != goal
    All the integers in nums are distinct.
*/

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut seen = vec![false; 1001];
        let mut cnt = 0;
        let mut q = vec![goal];
        while !q.is_empty() {
            cnt += 1;
            let mut q1 = vec![];
            for &x in q.iter() {
                for &n in nums.iter() {
                    for &xn in [x + n, x - n, x ^ n].iter() {
                        if (0..=1000).contains(&xn) && !seen[xn as usize] {
                            if xn == start {
                                return cnt;
                            }
                            seen[xn as usize] = true;
                            q1.push(xn);
                        }
                    }
                }
            }
            q = q1;
        }
        -1
    }
}

#[test]
fn test() {
    let cases = vec![(vec![2, 4, 12], 2, 12, 2), (vec![3, 5, 7], 0, -4, 2), (vec![2, 8, 16], 0, 1, -1)];
    for (nums, start, goal, expected) in cases {
        assert_eq!(Solution::minimum_operations(nums, start, goal), expected);
    }
}
