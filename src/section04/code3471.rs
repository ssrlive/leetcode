#![allow(dead_code)]

// 3471. Find the Largest Almost Missing Integer
// https://leetcode.com/problems/find-the-largest-almost-missing-integer/
// https;//leetcode.cn/problems/find-the-largest-almost-missing-integer/
//
// Easy
//
// You are given an integer array nums and an integer k.
//
// An integer x is almost missing from nums if x appears in exactly one subarray of size k within nums.
//
// Return the largest almost missing integer from nums. If no such integer exists, return -1.
//
// A subarray is a contiguous sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [3,9,2,1,7], k = 3
//
// Output: 7
//
// Explanation:
//
// 1 appears in 2 subarrays of size 3: [9, 2, 1] and [2, 1, 7].
// 2 appears in 3 subarrays of size 3: [3, 9, 2], [9, 2, 1], [2, 1, 7].
// 3 appears in 1 subarray of size 3: [3, 9, 2].
// 7 appears in 1 subarray of size 3: [2, 1, 7].
// 9 appears in 2 subarrays of size 3: [3, 9, 2], and [9, 2, 1].
// We return 7 since it is the largest integer that appears in exactly one subarray of size k.
//
// Example 2:
//
// Input: nums = [3,9,7,2,1,7], k = 4
//
// Output: 3
//
// Explanation:
//
// 1 appears in 2 subarrays of size 4: [9, 7, 2, 1], [7, 2, 1, 7].
// 2 appears in 3 subarrays of size 4: [3, 9, 7, 2], [9, 7, 2, 1], [7, 2, 1, 7].
// 3 appears in 1 subarray of size 4: [3, 9, 7, 2].
// 7 appears in 3 subarrays of size 4: [3, 9, 7, 2], [9, 7, 2, 1], [7, 2, 1, 7].
// 9 appears in 2 subarrays of size 4: [3, 9, 7, 2], [9, 7, 2, 1].
// We return 3 since it is the largest and only integer that appears in exactly one subarray of size k.
//
// Example 3:
//
// Input: nums = [0,0], k = 1
//
// Output: -1
//
// Explanation:
//
// There is no integer that appears in only one subarray of size 1.
//
// Constraints:
//
// 1 <= nums.length <= 50
// 0 <= nums[i] <= 50
// 1 <= k <= nums.length
//

struct Solution;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut subarray_count = std::collections::HashMap::new();

        for i in 0..=n - k as usize {
            let mut unique_nums = std::collections::HashSet::new();
            for &nums_j in nums.iter().skip(i).take(k as usize) {
                unique_nums.insert(nums_j);
            }
            for num in unique_nums {
                *subarray_count.entry(num).or_insert(0) += 1;
            }
        }

        let mut ans = -1;
        for (num, count) in subarray_count {
            if count == 1 && num > ans {
                ans = num;
            }
        }

        ans
    }
}

#[test]
fn test() {
    let nums = vec![3, 9, 2, 1, 7];
    let k = 3;
    let output = 7;
    assert_eq!(Solution::largest_integer(nums, k), output);

    let nums = vec![3, 9, 7, 2, 1, 7];
    let k = 4;
    let output = 3;
    assert_eq!(Solution::largest_integer(nums, k), output);

    let nums = vec![0, 0];
    let k = 1;
    let output = -1;
    assert_eq!(Solution::largest_integer(nums, k), output);
}
