#![allow(dead_code)]

/*

// 1959. Minimum Total Space Wasted With K Resizing Operations
// https://leetcode.com/problems/minimum-total-space-wasted-with-k-resizing-operations/
// https://leetcode.cn/problems/minimum-total-space-wasted-with-k-resizing-operations/
//
// Medium
//
// You are currently designing a dynamic array. You are given a 0-indexed integer array nums, where nums[i] is the number of elements that will be in the array at time i. In addition, you are given an integer k, the maximum number of times you can resize the array (to any size).

The size of the array at time t, sizet, must be at least nums[t] because there needs to be enough space in the array to hold all the elements. The space wasted at time t is defined as sizet - nums[t], and the total space wasted is the sum of the space wasted across every time t where 0 <= t < nums.length.

Return the minimum total space wasted if you can resize the array at most k times.

Note: The array can have any size at the start and does not count towards the number of resizing operations.

Example 1:

Input: nums = [10,20], k = 0
Output: 10
Explanation: size = [20,20].
We can set the initial size to be 20.
The total wasted space is (20 - 10) + (20 - 20) = 10.

Example 2:

Input: nums = [10,20,30], k = 1
Output: 10
Explanation: size = [20,20,30].
We can set the initial size to be 20 and resize to 30 at time 2.
The total wasted space is (20 - 10) + (20 - 20) + (30 - 30) = 10.

Example 3:

Input: nums = [10,20,15,30,20], k = 2
Output: 15
Explanation: size = [10,20,20,30,30].
We can set the initial size to 10, resize to 20 at time 1, and resize to 30 at time 3.
The total wasted space is (10 - 10) + (20 - 20) + (20 - 15) + (30 - 30) + (30 - 20) = 15.

Constraints:

    1 <= nums.length <= 200
    1 <= nums[i] <= 10^6
    0 <= k <= nums.length - 1
*/

struct Solution;

impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut total = vec![vec![-1; n]; n];
        let mut memo = vec![vec![-1; k as usize + 1]; n];
        let mut actual_space_used = 0;
        for i in 0..n {
            let mut max_value_in_i_to_j = 0;
            actual_space_used += nums[i];
            for (j, &item) in nums.iter().enumerate().skip(i) {
                max_value_in_i_to_j = max_value_in_i_to_j.max(item);
                total[i][j] = max_value_in_i_to_j * (j as i32 - i as i32 + 1);
            }
        }
        Solution::cal_min_space_used(0, k, &nums, &total, &mut memo) - actual_space_used
    }

    pub fn cal_min_space_used(
        idx: usize,
        k: i32,
        nums: &Vec<i32>,
        total: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        let n = nums.len();
        if idx == n {
            return 0;
        }
        if k == 0 {
            return total[idx][n - 1];
        }
        if memo[idx][k as usize] != -1 {
            return memo[idx][k as usize];
        }
        let mut answer = i32::MAX;
        for i in idx..n {
            let min_space_used = Solution::cal_min_space_used(i + 1, k - 1, nums, total, memo);
            answer = answer.min(total[idx][i] + min_space_used);
        }
        memo[idx][k as usize] = answer;
        answer
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![10, 20], 0, 10),
        (vec![10, 20, 30], 1, 10),
        (vec![10, 20, 15, 30, 20], 2, 15),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::min_space_wasted_k_resizing(nums, k), expected);
    }
}
