#![allow(dead_code)]

/*

// 1785. Minimum Elements to Add to Form a Given Sum
Medium
207
169
Companies

You are given an integer array nums and two integers limit and goal. The array nums has an interesting property that abs(nums[i]) <= limit.

Return the minimum number of elements you need to add to make the sum of the array equal to goal. The array must maintain its property that abs(nums[i]) <= limit.

Note that abs(x) equals x if x >= 0, and -x otherwise.

Example 1:

Input: nums = [1,-1,1], limit = 3, goal = -4
Output: 2
Explanation: You can add -2 and -3, then the sum of the array will be 1 - 1 + 1 - 2 - 3 = -4.

Example 2:

Input: nums = [1,-10,9,1], limit = 100, goal = 0
Output: 1

Constraints:

    1 <= nums.length <= 10^5
    1 <= limit <= 10^6
    -limit <= nums[i] <= limit
    -10^9 <= goal <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let mut temp = 0isize;
        for v in nums {
            temp += v as isize;
        }
        let diff = temp - goal as isize;
        let result = diff / limit as isize;
        (if diff % limit as isize != 0 {
            result.abs() + 1
        } else {
            result.abs()
        }) as i32
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![1, -1, 1], 3, -4, 2),
        (vec![1, -10, 9, 1], 100, 0, 1),
    ];
    for (nums, limit, goal, expected) in cases {
        assert_eq!(Solution::min_elements(nums, limit, goal), expected);
    }
}
