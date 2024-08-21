#![allow(dead_code)]

// 3255. Find the Power of K-Size Subarrays II
// https://leetcode.com/problems/find-the-power-of-k-size-subarrays-ii/
// https://leetcode.cn/problems/find-the-power-of-k-size-subarrays-ii/
//
// Medium
//
// You are given an array of integers nums of length n and a positive integer k.
//
// The power of an array is defined as:
//
//     Its maximum element if all of its elements are consecutive and sorted in ascending order.
//     -1 otherwise.
//
// You need to find the power of all subarrays of nums of size k.
//
// Return an integer array results of size n - k + 1, where results[i] is the power of nums[i..(i + k - 1)].
//
// Example 1:
//
// Input: nums = [1,2,3,4,3,2,5], k = 3
//
// Output: [3,4,-1,-1,-1]
//
// Explanation:
//
// There are 5 subarrays of nums of size 3:
//
//     [1, 2, 3] with the maximum element 3.
//     [2, 3, 4] with the maximum element 4.
//     [3, 4, 3] whose elements are not consecutive.
//     [4, 3, 2] whose elements are not sorted.
//     [3, 2, 5] whose elements are not consecutive.
//
// Example 2:
//
// Input: nums = [2,2,2,2,2], k = 4
//
// Output: [-1,-1]
//
// Example 3:
//
// Input: nums = [3,2,3,2,3,2], k = 2
//
// Output: [-1,3,-1,3,-1]
//
// Constraints:
//
//     1 <= n == nums.length <= 10^5
//     1 <= nums[i] <= 10^6
//     1 <= k <= n
//

struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut c = 1;
        let mut dp: Vec<_> = nums
            .windows(2)
            .map(|w| {
                if w[1] - w[0] == 1 {
                    c += 1
                } else {
                    c = 1
                };
                c
            })
            .collect();
        dp.insert(0, 1);
        (k as usize - 1..nums.len())
            .map(|i| if dp[i] >= k { nums[i] } else { -1 })
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3), vec![3, 4, -1, -1, -1]);
    assert_eq!(Solution::results_array(vec![2, 2, 2, 2, 2], 4), vec![-1, -1]);
    assert_eq!(Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2), vec![-1, 3, -1, 3, -1]);
}
