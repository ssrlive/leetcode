#![allow(dead_code)]

// 3660. Jump Game IX
// https://leetcode.com/problems/jump-game-ix/
// https://leetcode.cn/problems/jump-game-ix/
//
// Medium
//
// You are given an integer array nums.
//
// From any index i, you can jump to another index j under the following rules:
//
// Jump to index j where j > i is allowed only if nums[j] < nums[i].
// Jump to index j where j < i is allowed only if nums[j] > nums[i].
// For each index i, find the maximum value in nums that can be reached by following any sequence of valid jumps starting at i.
//
// Return an array ans where ans[i] is the maximum value reachable starting from index i.
//
// Example 1:
//
// Input: nums = [2,1,3]
//
// Output: [2,2,3]
//
// Explanation:
//
// For i = 0: No jump increases the value.
// For i = 1: Jump to j = 0 as nums[j] = 2 is greater than nums[i].
// For i = 2: Since nums[2] = 3 is the maximum value in nums, no jump increases the value.
// Thus, ans = [2, 2, 3].
//
// Example 2:
//
// Input: nums = [2,3,1]
//
// Output: [3,3,3]
//
// Explanation:
//
// For i = 0: Jump forward to j = 2 as nums[j] = 1 is less than nums[i] = 2, then from i = 2 jump to j = 1 as nums[j] = 3 is greater than nums[2].
// For i = 1: Since nums[1] = 3 is the maximum value in nums, no jump increases the value.
// For i = 2: Jump to j = 1 as nums[j] = 3 is greater than nums[2] = 1.
// Thus, ans = [3, 3, 3].
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut pre = vec![0; n];
        let mut suff = vec![0; n];
        let mut res = vec![0; n];

        pre[0] = nums[0];
        suff[n - 1] = nums[n - 1];
        for i in 1..n {
            pre[i] = pre[i - 1].max(nums[i]);
        }

        for i in (0..n - 1).rev() {
            suff[i] = suff[i + 1].min(nums[i]);
        }

        res[n - 1] = pre[n - 1];
        for i in (0..n - 1).rev() {
            res[i] = pre[i];
            if pre[i] > suff[i + 1] {
                res[i] = res[i + 1];
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 1, 3];
    let result = Solution::max_value(nums);
    assert_eq!(result, vec![2, 2, 3]);

    let nums = vec![2, 3, 1];
    let result = Solution::max_value(nums);
    assert_eq!(result, vec![3, 3, 3]);
}
