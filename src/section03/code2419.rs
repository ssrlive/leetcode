#![allow(dead_code)]

// 2419. Longest Subarray With Maximum Bitwise AND
// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/
// https://leetcode.cn/problems/longest-subarray-with-maximum-bitwise-and/
//
// You are given an integer array nums of size n.
//
// Consider a non-empty subarray from nums that has the maximum possible bitwise AND.
//
// In other words, let k be the maximum value of the bitwise AND of any subarray of nums.
// Then, only subarrays with a bitwise AND equal to k should be considered.
//
// Return the length of the longest such subarray.
//
// The bitwise AND of an array is the bitwise AND of all the numbers in it.
//
// A subarray is a contiguous sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,2,3,3,2,2]
// Output: 2
// Explanation:
// The maximum possible bitwise AND of a subarray is 3.
// The longest subarray with that value is [3,3], so we return 2.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: 1
// Explanation:
// The maximum possible bitwise AND of a subarray is 4.
// The longest subarray with that value is [4], so we return 1.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 1 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max = nums.iter().copied().max().unwrap();
        let mut slow = 0;
        let mut rez = 0;
        for (fast, &item) in nums.iter().enumerate() {
            if item == max {
                rez = rez.max(fast - slow + 1);
            } else {
                slow = fast + 1;
            }
        }
        rez as _
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 3, 2, 2];
    let rez = Solution::longest_subarray(nums);
    assert_eq!(rez, 2);

    let nums = vec![1, 2, 3, 4];
    let rez = Solution::longest_subarray(nums);
    assert_eq!(rez, 1);
}
