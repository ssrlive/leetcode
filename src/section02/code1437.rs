#![allow(dead_code)]

// 1437. Check If All 1's Are at Least Length K Places Away
// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/
// https://leetcode.cn/problems/check-if-all-1s-are-at-least-length-k-places-away/
//
// Easy
//
// Given an binary array nums and an integer k, return true if all 1's are at least k places away from each other, otherwise return false.
//
// Example 1:
//
// Input: nums = [1,0,0,0,1,0,0,1], k = 2
// Output: true
// Explanation: Each of the 1s are at least 2 places away from each other.
//
// Example 2:
//
// Input: nums = [1,0,0,1,0,1], k = 2
// Output: false
// Explanation: The second 1 and third 1 are only one apart from each other.
//
// Constraints:
//
// -    1 <= nums.length <= 10^5
// -    0 <= k <= nums.length
// -    nums[i] is 0 or 1
//

struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last = -1;
        for (i, &n) in nums.iter().enumerate() {
            let i = i as i32;
            if n == 1 {
                if last != -1 && i - last - 1 < k {
                    return false;
                }
                last = i;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2));
    assert!(!Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2));
    assert!(Solution::k_length_apart(vec![1, 1, 1, 1, 1], 0));
    assert!(Solution::k_length_apart(vec![0, 1, 0, 1], 1));
}
