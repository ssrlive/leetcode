#![allow(dead_code)]

// 3357. Minimize the Maximum Adjacent Element Difference
// https://leetcode.com/problems/minimize-the-maximum-adjacent-element-difference/
// https://leetcode.cn/problems/minimize-the-maximum-adjacent-element-difference/
//
// Hard
//
// You are given an array of integers nums. Some values in nums are missing and are denoted by -1.
//
// You can choose a pair of positive integers (x, y) exactly once and replace each missing element with either x or y.
//
// You need to minimize the maximum absolute difference between adjacent elements of nums after replacements.
//
// Return the minimum possible difference.
//
// Example 1:
//
// Input: nums = [1,2,-1,10,8]
//
// Output: 4
//
// Explanation:
//
// By choosing the pair as (6, 7), nums can be changed to [1, 2, 6, 10, 8].
//
// The absolute differences between adjacent elements are:
//
// |1 - 2| == 1
// |2 - 6| == 4
// |6 - 10| == 4
// |10 - 8| == 2
//
// Example 2:
//
// Input: nums = [-1,-1,-1]
//
// Output: 0
//
// Explanation:
//
// By choosing the pair as (4, 4), nums can be changed to [4, 4, 4].
//
// Example 3:
//
// Input: nums = [-1,10,-1,8]
//
// Output: 1
//
// Explanation:
//
// By choosing the pair as (11, 9), nums can be changed to [11, 10, 9, 8].
//
// Constraints:
//
// 2 <= nums.length <= 10^5
// nums[i] is either -1 or in the range [1, 10^9].
//

struct Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_adj = 0;
        let mut mina = i32::MAX;
        let mut maxb = i32::MIN;
        for i in 0..n - 1 {
            let a = nums[i];
            let b = nums[i + 1];
            if a > 0 && b > 0 {
                max_adj = max_adj.max((a - b).abs());
            } else if a > 0 || b > 0 {
                mina = mina.min(a.max(b));
                maxb = maxb.max(a.max(b));
            }
        }

        let mut res = 0;
        for i in 0..n {
            if (i > 0 && nums[i - 1] == -1) || nums[i] > 0 {
                continue;
            }
            let mut j = i;
            while j < n && nums[j] == -1 {
                j += 1;
            }
            let mut a = i32::MAX;
            let mut b = i32::MIN;
            if i > 0 {
                a = a.min(nums[i - 1]);
                b = b.max(nums[i - 1]);
            }
            if j < n {
                a = a.min(nums[j]);
                b = b.max(nums[j]);
            }
            if a <= b {
                if j - i == 1 {
                    res = res.max((maxb - a).min(b - mina));
                } else {
                    res = res.max((maxb - a).min((b - mina).min((maxb - mina + 2) / 3 * 2)));
                }
            }
        }
        max_adj.max((res + 1) / 2)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_difference(vec![1, 2, -1, 10, 8]), 4);
    assert_eq!(Solution::min_difference(vec![-1, -1, -1]), 0);
    assert_eq!(Solution::min_difference(vec![-1, 10, -1, 8]), 1);
}
