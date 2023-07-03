#![allow(dead_code)]

/*

// 1493. Longest Subarray of 1's After Deleting One Element
// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
// https://leetcode.cn/problems/longest-subarray-of-1s-after-deleting-one-element/
//
// Medium
//
// Given a binary array nums, you should delete one element from it.

Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.

Example 1:

Input: nums = [1,1,0,1]
Output: 3
Explanation: After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1's.

Example 2:

Input: nums = [0,1,1,1,0,1,1,0,1]
Output: 5
Explanation: After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1's is [1,1,1,1,1].

Example 3:

Input: nums = [1,1,1]
Output: 2
Explanation: You must delete one element.

Constraints:

    1 <= nums.length <= 10^5
    nums[i] is either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut zeroes = 0;
        let mut ans = 0;

        for end in 0..nums.len() {
            if nums[end] == 0 {
                zeroes += 1;
            }
            while zeroes > 1 {
                if nums[start] == 0 {
                    zeroes -= 1;
                }
                start += 1;
            }
            ans = ans.max(end - start);
        }
        ans as i32
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 1, 0, 1], 3), (vec![0, 1, 1, 1, 0, 1, 1, 0, 1], 5), (vec![1, 1, 1], 2)];
    for (nums, expected) in cases {
        assert_eq!(Solution::longest_subarray(nums), expected);
    }
}
