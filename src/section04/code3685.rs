#![allow(dead_code)]

// 3685. Subsequence Sum After Capping Elements
// https://leetcode.com/problems/subsequence-sum-after-capping-elements/
// https://leetcode.cn/problems/subsequence-sum-after-capping-elements/
//
// Medium
//
// You are given an integer array nums of size n and a positive integer k.
//
// An array capped by value x is obtained by replacing every element nums[i] with min(nums[i], x).
//
// For each integer x from 1 to n, determine whether it is possible to choose a subsequence from the array capped by x such that the sum of the chosen elements is exactly k.
//
// Return a 0-indexed boolean array answer of size n, where answer[i] is true if it is possible when using x = i + 1, and false otherwise.
//
// Example 1:
//
// Input: nums = [4,3,2,4], k = 5
//
// Output: [false,false,true,true]
//
// Explanation:
//
// For x = 1, the capped array is [1, 1, 1, 1]. Possible sums are 1, 2, 3, 4, so it is impossible to form a sum of 5.
// For x = 2, the capped array is [2, 2, 2, 2]. Possible sums are 2, 4, 6, 8, so it is impossible to form a sum of 5.
// For x = 3, the capped array is [3, 3, 2, 3]. A subsequence [2, 3] sums to 5, so it is possible.
// For x = 4, the capped array is [4, 3, 2, 4]. A subsequence [3, 2] sums to 5, so it is possible.
//
// Example 2:
//
// Input: nums = [1,2,3,4,5], k = 3
//
// Output: [true,true,true,true,true]
//
// Explanation:
//
// For every value of x, it is always possible to select a subsequence from the capped array that sums exactly to 3.
//
// Constraints:
//
// 1 <= n == nums.length <= 4000
// 1 <= nums[i] <= n
// 1 <= k <= 4000
//

struct Solution;

impl Solution {
    pub fn subsequence_sum_after_capping(nums: Vec<i32>, k: i32) -> Vec<bool> {
        let n = nums.len();
        let mut arr = nums.clone();
        arr.sort_unstable();
        let mut sum = vec![false; (k + 1) as usize];
        let mut ans = vec![false; n];
        sum[0] = true;

        let mut i = 0;
        for x in 1..=n as i32 {
            while i < n && arr[i] < x {
                for j in (arr[i]..=k).rev() {
                    if sum[(j - arr[i]) as usize] {
                        sum[j as usize] = true;
                    }
                }
                i += 1;
            }

            let bigger = n - i;
            for j in 0..=bigger {
                if j as i32 * x > k {
                    break;
                }
                if sum[(k - j as i32 * x) as usize] {
                    ans[(x - 1) as usize] = true;
                    break;
                }
            }
        }

        ans
    }
}

#[test]
fn test() {
    let nums = vec![4, 3, 2, 4];
    let k = 5;
    let result = Solution::subsequence_sum_after_capping(nums, k);
    assert_eq!(result, vec![false, false, true, true]);

    let nums = vec![1, 2, 3, 4, 5];
    let k = 3;
    let result = Solution::subsequence_sum_after_capping(nums, k);
    assert_eq!(result, vec![true, true, true, true, true]);
}
