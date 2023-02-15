#![allow(dead_code)]

/*

// 1658. Minimum Operations to Reduce X to Zero
// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/
// https://leetcode.cn/problems/minimum-operations-to-reduce-x-to-zero/
//
// Medium
//
// You are given an integer array nums and an integer x. In one operation, you can either remove the leftmost or the rightmost element from the array nums and subtract its value from x. Note that this modifies the array for future operations.

Return the minimum number of operations to reduce x to exactly 0 if it is possible, otherwise, return -1.

Example 1:

Input: nums = [1,1,4,2,3], x = 5
Output: 2
Explanation: The optimal solution is to remove the last two elements to reduce x to zero.

Example 2:

Input: nums = [5,6,7,8,9], x = 4
Output: -1

Example 3:

Input: nums = [3,2,20,1,1,3], x = 10
Output: 5
Explanation: The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^4
    1 <= x <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        match nums.iter().sum::<i32>() - x {
            target if target < 0 => -1,
            0 => nums.len() as i32,
            target => {
                match nums
                    .iter()
                    .enumerate()
                    .fold(
                        (nums.iter().enumerate().peekable(), 0, usize::MIN),
                        |(mut left, mut sum, mut max_len), (right_index, right_num)| {
                            sum += *right_num;
                            while sum > target {
                                let (_, left_num) = left.next().unwrap();
                                sum -= left_num;
                            }
                            if sum == target {
                                max_len = max_len.max(right_index - left.peek().unwrap().0 + 1);
                            }
                            (left, sum, max_len)
                        },
                    )
                    .2
                {
                    usize::MIN => -1,
                    max_len => (nums.len() - max_len) as i32,
                }
            }
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 1, 4, 2, 3], 5, 2),
        (vec![5, 6, 7, 8, 9], 4, -1),
        (vec![3, 2, 20, 1, 1, 3], 10, 5),
    ];
    for (nums, x, expected) in cases {
        assert_eq!(Solution::min_operations(nums, x), expected);
    }
}
