#![allow(dead_code)]

/*

// 1508. Range Sum of Sorted Subarray Sums
// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/
// https://leetcode.cn/problems/range-sum-of-sorted-subarray-sums/
//
// Medium
//
// You are given the array nums consisting of n positive integers. You computed the sum of all non-empty continuous subarrays from the array and then sorted them in non-decreasing order, creating a new array of n * (n + 1) / 2 numbers.

Return the sum of the numbers from index left to index right (indexed from 1), inclusive, in the new array. Since the answer can be a huge number return it modulo 109 + 7.

Example 1:

Input: nums = [1,2,3,4], n = 4, left = 1, right = 5
Output: 13
Explanation: All subarray sums are 1, 3, 6, 10, 2, 5, 9, 3, 7, 4. After sorting them in non-decreasing order we have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 1 to ri = 5 is 1 + 2 + 3 + 3 + 4 = 13.

Example 2:

Input: nums = [1,2,3,4], n = 4, left = 3, right = 4
Output: 6
Explanation: The given array is the same as example 1. We have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 3 to ri = 4 is 3 + 3 = 6.

Example 3:

Input: nums = [1,2,3,4], n = 4, left = 1, right = 10
Output: 50

Constraints:

    n == nums.length
    1 <= nums.length <= 1000
    1 <= nums[i] <= 100
    1 <= left <= right <= n * (n + 1) / 2
*/

struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let left = left as usize;
        let right = right as usize;
        let mut sums = Vec::with_capacity(n * (n + 1) / 2);
        for i in 0..n {
            let mut sum = 0;
            for &num in nums.iter().take(n).skip(i) {
                sum += num;
                sums.push(sum);
            }
        }
        sums.sort_unstable();
        let mut res = 0;
        for &sum in sums.iter().take(right).skip(left - 1) {
            res += sum;
            res %= 1_000_000_007;
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], 4, 1, 5, 13),
        (vec![1, 2, 3, 4], 4, 3, 4, 6),
        (vec![1, 2, 3, 4], 4, 1, 10, 50),
    ];
    for (nums, n, left, right, expect) in cases {
        assert_eq!(Solution::range_sum(nums, n, left, right), expect);
    }
}
