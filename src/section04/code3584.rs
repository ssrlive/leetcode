#![allow(dead_code)]

// 3584. Maximum Product of First and Last Elements of a Subsequence
// https://leetcode.com/problems/maximum-product-of-first-and-last-elements-of-a-subsequence/
// https://leetcode.cn/problems/maximum-product-of-first-and-last-elements-of-a-subsequence/
//
// Medium
//
// You are given an integer array nums and an integer m.
//
// Return the maximum product of the first and last elements of any of nums of size m.
//
// Example 1:
//
// Input: nums = [-1,-9,2,3,-2,-3,1], m = 1
//
// Output: 81
//
// Explanation:
//
// The subsequence [-9] has the largest product of the first and last elements: -9 * -9 = 81. Therefore, the answer is 81.
//
// Example 2:
//
// Input: nums = [1,3,-5,5,6,-4], m = 3
//
// Output: 20
//
// Explanation:
//
// The subsequence [-5, 6, -4] has the largest product of the first and last elements.
//
// Example 3:
//
// Input: nums = [2,-1,2,-6,5,2,-5,7], m = 2
//
// Output: 35
//
// Explanation:
//
// The subsequence [5, 7] has the largest product of the first and last elements.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     -10^5 <= nums[i] <= 10^5
//     1 <= m <= nums.length
//

struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        let n = nums.len();
        let mut ans: i64 = i64::MIN;

        // If m == 1, return the maximum square of any number
        if m == 1 {
            for &num in &nums {
                let v = (num as i64) * (num as i64);
                ans = ans.max(v);
            }
            return ans;
        }

        // Initialize max and min of first elements in window
        let mut maxi: i64 = i64::MIN;
        let mut mini: i64 = i64::MAX;

        // Sliding window of size m
        for j in (m - 1) as usize..n {
            let i = j - (m - 1) as usize; // start of the window
            maxi = maxi.max(nums[i] as i64);
            mini = mini.min(nums[i] as i64);

            let prod1 = maxi * (nums[j] as i64);
            let prod2 = mini * (nums[j] as i64);
            ans = ans.max(prod1).max(prod2);
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_product(vec![-1, -9, 2, 3, -2, -3, 1], 1), 81);
    assert_eq!(Solution::maximum_product(vec![1, 3, -5, 5, 6, -4], 3), 20);
    assert_eq!(Solution::maximum_product(vec![2, -1, 2, -6, 5, 2, -5, 7], 2), 35);
}
