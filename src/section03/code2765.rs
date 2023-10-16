#![allow(dead_code)]

// 2765. Longest Alternating Subarray
// https://leetcode.com/problems/longest-alternating-subarray/
// https://leetcode.cn/problems/longest-alternating-subarray/
//
// Easy
//
// You are given a 0-indexed integer array nums. A subarray s of length m is called alternating if:
//
//     - m is greater than 1.
//     - s1 = s0 + 1.
//     - The 0-indexed subarray s looks like [s0, s1, s0, s1,...,s(m-1) % 2].
//       In other words, s1 - s0 = 1, s2 - s1 = -1, s3 - s2 = 1, s4 - s3 = -1, and so on up to s[m - 1] - s[m - 2] = (-1)m.
//
// Return the maximum length of all alternating subarrays present in nums or -1 if no such subarray exists.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [2,3,4,3,4]
// Output: 4
// Explanation: The alternating subarrays are [3,4], [3,4,3], and [3,4,3,4]. The longest of these is [3,4,3,4], which is of length 4.
//
// Example 2:
//
// Input: nums = [4,5,6]
// Output: 2
// Explanation: [4,5] and [5,6] are the only two alternating subarrays. They are both of length 2.
//
// Constraints:
//
//     2 <= nums.length <= 100
//     1 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut alternate_array = vec![];
        for num in nums.windows(2) {
            alternate_array.push(num[1] - num[0]);
        }
        let (mut prev, mut count, mut result) = (0, 0, 0);
        for &s in alternate_array.iter() {
            if (s == 1 && (prev == -1 || count == 0)) || (s == -1 && prev == 1) {
                count += 1;
                prev = s;
                continue;
            }
            prev = s;
            result = result.max(count);
            if s == 1 && prev != -1 {
                count = 1;
            } else {
                count = 0;
            }
        }
        if count != 0 {
            result = result.max(count);
        }
        if result == 0 {
            return -1;
        }
        result + 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
    assert_eq!(Solution::alternating_subarray(vec![4, 5, 6]), 2);
}
