#![allow(dead_code)]

// 992. Subarrays with K Different Integers
// https://leetcode.com/problems/subarrays-with-k-different-integers/
// https://leetcode.cn/problems/subarrays-with-k-different-integers/
//
// Given an integer array nums and an integer k, return the number of good subarrays of nums.
//
// A good array is an array where the number of different integers in that array is exactly k.
//
// For example, [1,2,3,1,2] has 3 different integers: 1, 2, and 3.
// A subarray is a contiguous part of an array.
//
// Example 1:
//
// Input: nums = [1,2,1,2,3], k = 2
// Output: 7
// Explanation: Subarrays formed with exactly 2 different integers: [1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]
//
// Example 2:
//
// Input: nums = [1,2,1,3,4], k = 3
// Output: 3
// Explanation: Subarrays formed with exactly 3 different integers: [1,2,1,3], [2,1,3], [1,3,4].
//
// Constraints:
//
// - 1 <= nums.length <= 2 * 10^4
// - 1 <= nums[i], k <= nums.length
//

struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        fn at_most_k(nums: &[i32], k: i32) -> i32 {
            let mut i = 0;
            let mut res = 0;
            let mut count = std::collections::HashMap::new();
            let mut k = k;
            for (j, &num) in nums.iter().enumerate() {
                *count.entry(num).or_insert(0) += 1;
                if count[&num] == 1 {
                    k -= 1;
                }
                while k < 0 {
                    *count.entry(nums[i]).or_insert(0) -= 1;
                    if count[&nums[i]] == 0 {
                        k += 1;
                    }
                    i += 1;
                }
                res += j - i + 1;
            }
            res as _
        }

        at_most_k(&nums, k) - at_most_k(&nums, k - 1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
}
