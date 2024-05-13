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
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let mut ans = 0;
        let mut arr = Vec::new();
        let mut i0 = -1;
        for (i, &num) in nums.iter().enumerate() {
            if k % num > 0 {
                arr.clear();
                i0 = i as i32;
                continue;
            }
            arr.push(vec![num, i as i32]);
            let mut j = 0;
            for k in 0..arr.len() {
                arr[k][0] = arr[k][0] / gcd(arr[k][0], num) * num;
                if arr[k][0] != arr[j][0] {
                    j += 1;
                    #[allow(clippy::assigning_clones)]
                    {
                        arr[j] = arr[k].clone();
                    }
                } else {
                    arr[j][1] = arr[k][1];
                }
            }
            arr.truncate(j + 1);
            if arr[0][0] == k {
                ans += arr[0][1] - i0;
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
