#![allow(dead_code)]

/*

// 1770. Maximum Score from Performing Multiplication Operations
Hard
2.3K
504
Companies

You are given two 0-indexed integer arrays nums and multipliers of size n and m respectively, where n >= m.

You begin with a score of 0. You want to perform exactly m operations. On the ith operation (0-indexed) you will:

    Choose one integer x from either the start or the end of the array nums.
    Add multipliers[i] * x to your score.
        Note that multipliers[0] corresponds to the first operation, multipliers[1] to the second operation, and so on.
    Remove x from nums.

Return the maximum score after performing m operations.

Example 1:

Input: nums = [1,2,3], multipliers = [3,2,1]
Output: 14
Explanation: An optimal solution is as follows:
- Choose from the end, [1,2,3], adding 3 * 3 = 9 to the score.
- Choose from the end, [1,2], adding 2 * 2 = 4 to the score.
- Choose from the end, [1], adding 1 * 1 = 1 to the score.
The total score is 9 + 4 + 1 = 14.

Example 2:

Input: nums = [-5,-3,-3,-2,7,1], multipliers = [-10,-5,3,4,6]
Output: 102
Explanation: An optimal solution is as follows:
- Choose from the start, [-5,-3,-3,-2,7,1], adding -5 * -10 = 50 to the score.
- Choose from the start, [-3,-3,-2,7,1], adding -3 * -5 = 15 to the score.
- Choose from the start, [-3,-2,7,1], adding -3 * 3 = -9 to the score.
- Choose from the end, [-2,7,1], adding 1 * 4 = 4 to the score.
- Choose from the end, [-2,7], adding 7 * 6 = 42 to the score.
The total score is 50 + 15 - 9 + 4 + 42 = 102.

Constraints:

    n == nums.length
    m == multipliers.length
    1 <= m <= 300
    m <= n <= 10^5
    -1000 <= nums[i], multipliers[i] <= 1000
*/

struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp_prev = vec![0; m + 1];
        let mut dp_curr = dp_prev.clone();
        for left in (0..m).rev() {
            for right in (0..m - left).rev() {
                let multiplier = multipliers[left + right];
                dp_curr[right] = (dp_prev[right] + multiplier * nums[left])
                    .max(dp_curr[right + 1] + multiplier * nums[n - right - 1]);
            }
            std::mem::swap(&mut dp_prev, &mut dp_curr);
        }
        dp_prev[0]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3], vec![3, 2, 1], 14),
        (vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6], 102),
    ];
    for (nums, multipliers, expected) in cases {
        assert_eq!(Solution::maximum_score(nums, multipliers), expected);
    }
}
