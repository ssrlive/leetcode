#![allow(dead_code)]

/*

// 1793. Maximum Score of a Good Subarray
Hard
870
27
Companies

You are given an array of integers nums (0-indexed) and an integer k.

The score of a subarray (i, j) is defined as min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1). A good subarray is a subarray where i <= k <= j.

Return the maximum possible score of a good subarray.

Example 1:

Input: nums = [1,4,3,7,4,5], k = 3
Output: 15
Explanation: The optimal subarray is (1, 5) with a score of min(4,3,7,4,5) * (5-1+1) = 3 * 5 = 15.

Example 2:

Input: nums = [5,5,4,5,4,1,1,1], k = 0
Output: 20
Explanation: The optimal subarray is (0, 4) with a score of min(5,5,4,5,4) * (4-0+1) = 4 * 5 = 20.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 2 * 10^4
    0 <= k < nums.length
*/

struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let (mut res, mut mini) = (nums[k as usize], nums[k as usize]);
        let (mut i, mut j) = (k, k);
        let n = nums.len() as i32;
        while i > 0 || j < n - 1 {
            let left = if i > 0 { nums[(i - 1) as usize] } else { 0 };
            let right = if j < n - 1 { nums[(j + 1) as usize] } else { 0 };
            if left < right {
                mini = mini.min(nums[(j + 1) as usize]);
                j += 1;
            } else {
                mini = mini.min(nums[(i - 1) as usize]);
                i -= 1;
            }
            res = res.max(mini * (j - i + 1));
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 4, 3, 7, 4, 5];
    let k = 3;
    let res = 15;
    assert_eq!(Solution::maximum_score(nums, k), res);

    let nums = vec![5, 5, 4, 5, 4, 1, 1, 1];
    let k = 0;
    let res = 20;
    assert_eq!(Solution::maximum_score(nums, k), res);
}
