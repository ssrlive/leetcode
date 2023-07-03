#![allow(dead_code)]

// 995. Minimum Number of K Consecutive Bit Flips
// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/
// https://leetcode.cn/problems/minimum-number-of-k-consecutive-bit-flips/
//
// You are given a binary array nums and an integer k.
//
// A k-bit flip is choosing a subarray of length k from nums and simultaneously changing every 0 in the subarray to 1, and every 1 in the subarray to 0.
//
// Return the minimum number of k-bit flips required so that there is no 0 in the array. If it is not possible, return -1.
//
// A subarray is a contiguous part of an array.
//
// Example 1:
//
// Input: nums = [0,1,0], k = 1
// Output: 2
// Explanation: Flip nums[0], then flip nums[2].
//
// Example 2:
//
// Input: nums = [1,1,0], k = 2
// Output: -1
// Explanation: No matter how we flip subarrays of size 2, we cannot make the array become [1,1,1].
//
// Example 3:
//
// Input: nums = [0,0,0,1,0,1,1,0], k = 3
// Output: 3
// Explanation:
// Flip nums[0],nums[1],nums[2]: nums becomes [1,1,1,1,0,1,1,0]
// Flip nums[4],nums[5],nums[6]: nums becomes [1,1,1,1,1,0,0,0]
// Flip nums[5],nums[6],nums[7]: nums becomes [1,1,1,1,1,1,1,1]
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 1 <= k <= nums.length
//

struct Solution;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = k as usize;
        let mut cur = 0;
        let mut res = 0;
        let n = nums.len();

        for i in 0..n {
            if i >= k && nums[i - k] > 1 {
                cur -= 1;
                nums[i - k] -= 2;
            }
            if cur % 2 == nums[i] as usize {
                if i + k > n {
                    return -1;
                }
                nums[i] += 2;
                cur += 1;
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![(vec![0, 1, 0], 1, 2), (vec![1, 1, 0], 2, -1), (vec![0, 0, 0, 1, 0, 1, 1, 0], 3, 3)];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::min_k_bit_flips(nums, k), expected);
    }
}
