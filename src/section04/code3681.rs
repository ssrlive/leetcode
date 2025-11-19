#![allow(dead_code)]

// 3681. Maximum XOR of Subsequences
// https://leetcode.com/problems/maximum-xor-of-subsequences/
// https://leetcode.cn/problems/maximum-xor-of-subsequences/
//
// Hard
//
// You are given an integer array nums of length n where each element is a non-negative integer.
//
// Select two subsequences of nums (they may be empty and are allowed to overlap), each preserving the original order of elements, and let:
//
// X be the bitwise XOR of all elements in the first subsequence.
// Y be the bitwise XOR of all elements in the second subsequence.
// Return the maximum possible value of X XOR Y.
//
// Note: The XOR of an empty subsequence is 0.
//
// Example 1:
//
// Input: nums = [1,2,3]
//
// Output: 3
//
// Explanation:
//
// Choose subsequences:
//
// First subsequence [2], whose XOR is 2.
// Second subsequence [2,3], whose XOR is 1.
// Then, XOR of both subsequences = 2 XOR 1 = 3.
//
// This is the maximum XOR value achievable from any two subsequences.
//
// Example 2:
//
// Input: nums = [5,2]
//
// Output: 7
//
// Explanation:
//
// Choose subsequences:
//
// First subsequence [5], whose XOR is 5.
// Second subsequence [2], whose XOR is 2.
// Then, XOR of both subsequences = 5 XOR 2 = 7.
//
// This is the maximum XOR value achievable from any two subsequences.
//
// Constraints:
//
// 2 <= nums.length <= 10^5
// 0 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_xor_subsequences(nums: Vec<i32>) -> i32 {
        let mut temp = [0; 32];
        for &num in &nums {
            let mut num = num;
            for i in (0..32).rev() {
                if (num & (1 << i)) == 0 {
                    continue;
                }
                if temp[i] == 0 {
                    temp[i] = num;
                    break;
                }
                num ^= temp[i];
            }
        }

        let mut ans = 0;
        for i in (0..32).rev() {
            ans = ans.max(ans ^ temp[i]);
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    assert_eq!(Solution::max_xor_subsequences(nums), 3);

    let nums = vec![5, 2];
    assert_eq!(Solution::max_xor_subsequences(nums), 7);
}
