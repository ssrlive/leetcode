#![allow(dead_code)]

/*

// 2289. Steps to Make Array Non-decreasing
// https://leetcode.com/problems/steps-to-make-array-non-decreasing/
// https://leetcode.cn/problems/steps-to-make-array-non-decreasing/
//
// Medium
//
// You are given a 0-indexed integer array nums. In one step,
// remove all elements nums[i] where nums[i - 1] > nums[i] for all 0 < i < nums.length.

Return the number of steps performed until nums becomes a non-decreasing array.

Example 1:

Input: nums = [5,3,4,4,7,3,6,11,8,5,11]
Output: 3
Explanation: The following are the steps performed:
- Step 1: [5,3,4,4,7,3,6,11,8,5,11] becomes [5,4,4,7,6,11,11]
- Step 2: [5,4,4,7,6,11,11] becomes [5,4,7,11,11]
- Step 3: [5,4,7,11,11] becomes [5,7,11,11]
[5,7,11,11] is a non-decreasing array. Therefore, we return 3.

Example 2:

Input: nums = [4,5,7,7,13]
Output: 0
Explanation: nums is already a non-decreasing array. Therefore, we return 0.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut s = vec![(nums[0], 0)];
        let mut ans = 0;
        for &v in &nums[1..] {
            let mut cnt = 0;
            while !s.is_empty() && s[s.len() - 1].0 <= v {
                let curr = s.pop().unwrap_or((0, 0)).1;
                cnt = cnt.max(curr);
            }
            if !s.is_empty() {
                cnt += 1;
            }
            s.push((v, cnt));
            ans = cnt.max(ans);
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]), 3);
    assert_eq!(Solution::total_steps(vec![4, 5, 7, 7, 13]), 0);
}
