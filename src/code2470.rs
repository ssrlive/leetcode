#![allow(dead_code)]

// 2470. Number of Subarrays With LCM Equal to K
// https://leetcode.com/problems/number-of-subarrays-with-lcm-equal-to-k/
// https://leetcode.cn/problems/number-of-subarrays-with-lcm-equal-to-k/
//
// Given an integer array nums and an integer k, return the number of subarrays of nums
// where the least common multiple of the subarray's elements is k.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// The least common multiple of an array is the smallest positive integer that is divisible by all the array elements.
//
// Example 1:
//
// Input: nums = [3,6,2,7,1], k = 6
// Output: 4
// Explanation: The subarrays of nums where 6 is the least common multiple of all the subarray's elements are:
// - [3,6,2,7,1]
// - [3,6,2,7,1]
// - [3,6,2,7,1]
// - [3,6,2,7,1]
//
// Example 2:
//
// Input: nums = [3], k = 2
// Output: 0
// Explanation: There are no subarrays of nums where 2 is the least common multiple of all the subarray's elements.
//
// Constraints:
//
// - 1 <= nums.length <= 1000
// - 1 <= nums[i], k <= 1000
//

struct Solution;

impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::{max, min};

        fn gcd(a: i32, b: i32) -> i32 {
            match ((a, b), (a & 1, b & 1)) {
                ((x, y), _) if x == y => y,
                ((0, x), _) | ((x, 0), _) => x,
                ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
                ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
                ((x, y), (1, 1)) => {
                    let (x, y) = (min(x, y), max(x, y));
                    gcd((y - x) >> 1, x)
                }
                _ => unreachable!(),
            }
        }

        fn lcm(a: i32, b: i32) -> i32 {
            a * b / gcd(a, b)
        }

        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            let mut l = 1;
            for &item in nums.iter().take(n).skip(i) {
                l = lcm(l, item);
                if l == k {
                    ans += 1;
                }
                if l > k {
                    break;
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::subarray_lcm(vec![3, 6, 2, 7, 1], 6), 4);
    assert_eq!(Solution::subarray_lcm(vec![3], 2), 0);
}
