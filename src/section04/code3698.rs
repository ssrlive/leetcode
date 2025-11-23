#![allow(dead_code)]

// 3698. Split Array With Minimum Difference
// https://leetcode.com/problems/split-array-with-minimum-difference/
// https://leetcode.cn/problems/split-array-with-minimum-difference/
//
// Medium
//
// You are given an integer array nums.
//
// Split the array into exactly two subarrays, left and right, such that left is strictly
// increasing and right is strictly decreasing.
//
// Return the minimum possible absolute difference between the sums of left and right.
// If no valid split exists, return -1.
//
// Example 1:
//
// Input: nums = [1,3,2]
//
// Output: 2
//
// Explanation:
//
// i	left	right	Validity	left sum	right sum	Absolute difference
// 0	[1]	[3, 2]	Yes	1	5	|1 - 5| = 4
// 1	[1, 3]	[2]	Yes	4	2	|4 - 2| = 2
// Thus, the minimum absolute difference is 2.
//
// Example 2:
//
// Input: nums = [1,2,4,3]
//
// Output: 4
//
// Explanation:
//
// i	left	right	Validity	left sum	right sum	Absolute difference
// 0	[1]	[2, 4, 3]	No	1	9	-
// 1	[1, 2]	[4, 3]	Yes	3	7	|3 - 7| = 4
// 2	[1, 2, 4]	[3]	Yes	7	3	|7 - 3| = 4
// Thus, the minimum absolute difference is 4.
//
// Example 3:
//
// Input: nums = [3,1,2]
//
// Output: -1
//
// Explanation:
//
// No valid split exists, so the answer is -1.
//
// Constraints:
//
// 2 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let (mut l, mut r) = (0, n - 1);
        let (mut lsum, mut rsum) = (0i64, 0i64);

        while l < n - 1 && nums[l] < nums[l + 1] {
            lsum += nums[l] as i64;
            l += 1;
        }

        while r > 0 && nums[r - 1] > nums[r] {
            rsum += nums[r] as i64;
            r -= 1;
        }

        if l == r {
            let option1 = (lsum + nums[l] as i64 - rsum).abs();
            let option2 = (lsum - (rsum + nums[l] as i64)).abs();
            option1.min(option2)
        } else if r - l == 1 && nums[l] == nums[r] {
            (lsum - rsum).abs()
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::split_array(vec![1, 3, 2]), 2);
    assert_eq!(Solution::split_array(vec![1, 2, 4, 3]), 4);
    assert_eq!(Solution::split_array(vec![3, 1, 2]), -1);
}
