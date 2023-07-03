#![allow(dead_code)]

// 1343. Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold
// https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
// https://leetcode.cn/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
//
// Medium
//
// Given an array of integers arr and two integers k and threshold, return the number of sub-arrays of
// size k and average greater than or equal to threshold.
//
// Example 1:
//
// Input: arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
// Output: 3
// Explanation: Sub-arrays [2,5,5],[5,5,5] and [5,5,8] have averages 4, 5 and 6 respectively.
// All other sub-arrays of size 3 have averages less than 4 (the threshold).
//
// Example 2:
//
// Input: arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
// Output: 6
// Explanation: The first 6 sub-arrays of size 3 have averages greater than 5. Note that averages are not integers.
//
// Constraints:
//
// -    1 <= arr.length <= 10^5
// -    1 <= arr[i] <= 10^4
// -    1 <= k <= arr.length
// -    0 <= threshold <= 10^4
//

struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let k = k as usize;
        let threshold = threshold as i64;
        let mut sum = 0;
        for &item in arr.iter().take(k) {
            sum += item as i64;
        }
        let mut count = 0;
        if sum / k as i64 >= threshold {
            count += 1;
        }
        for i in k..arr.len() {
            sum += arr[i] as i64 - arr[i - k] as i64;
            if sum / k as i64 >= threshold {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
    assert_eq!(Solution::num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5), 6);
}
