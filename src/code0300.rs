#![allow(dead_code)]

// 300. Longest Increasing Subsequence
// https://leetcode.com/problems/longest-increasing-subsequence/
// https://leetcode.cn/problems/longest-increasing-subsequence/
//
// Given an integer array nums, return the length of the longest strictly increasing subsequence.
//
// A subsequence is a sequence that can be derived from an array by deleting some or no elements
// without changing the order of the remaining elements. For example, [3,6,2,7] is a subsequence of
// the array [0,3,1,6,2,2,7].
//
// Example 1:
//
// Input: nums = [10,9,2,5,3,7,101,18]
// Output: 4
// Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
//
// Example 2:
//
// Input: nums = [0,1,0,3,2,3]
// Output: 4
//
// Example 3:
//
// Input: nums = [7,7,7,7,7,7,7]
// Output: 1
//
// Constraints:
//
// - 1 <= nums.length <= 2500
// - -10^4 <= nums[i] <= 10^4
//
// Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?
//

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = vec![];
        for num in nums {
            let mut left = 0;
            let mut right = tails.len();
            while left < right {
                let mid = left + (right - left) / 2;
                if tails[mid] < num {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            if left == tails.len() {
                tails.push(num);
            } else {
                tails[left] = num;
            }
        }
        tails.len() as i32
    }
}

#[test]
fn test_length_of_lis() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
}
