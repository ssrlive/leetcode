#![allow(dead_code)]

// 978. Longest Turbulent Subarray
// https://leetcode.com/problems/longest-turbulent-subarray/
// https://leetcode.cn/problems/longest-turbulent-subarray/
//
// Given an integer array arr, return the length of a maximum size turbulent subarray of arr.
//
// A subarray is turbulent if the comparison sign flips between each adjacent pair of elements in the subarray.
//
// More formally, a subarray [arr[i], arr[i + 1], ..., arr[j]] of arr is said to be turbulent if and only if:
//
// For i <= k < j:
// arr[k] > arr[k + 1] when k is odd, and
// arr[k] < arr[k + 1] when k is even.
// Or, for i <= k < j:
// arr[k] > arr[k + 1] when k is even, and
// arr[k] < arr[k + 1] when k is odd.
//
// Example 1:
//
// Input: arr = [9,4,2,10,7,8,8,1,9]
// Output: 5
// Explanation: arr[1] > arr[2] < arr[3] > arr[4] < arr[5]
//
// Example 2:
//
// Input: arr = [4,8,12,16]
// Output: 2
//
// Example 3:
//
// Input: arr = [100]
// Output: 1
//
// Constraints:
//
// - 1 <= arr.length <= 4 * 10^4
// - 0 <= arr[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut max = 1;
        let mut l = 0;

        for i in 1..n {
            let ii = i as i32;
            if i % 2 == 1 {
                if arr[i] <= arr[i - 1] {
                    max = std::cmp::max(max, ii - l);
                    l = ii;
                }
            } else if arr[i] >= arr[i - 1] {
                max = std::cmp::max(max, ii - l);
                l = ii;
            }
        }
        max = std::cmp::max(max, n as i32 - l);

        let mut l = 0;
        for i in 1..n {
            let ii = i as i32;
            if i % 2 == 1 {
                if arr[i] >= arr[i - 1] {
                    max = std::cmp::max(max, ii - l);
                    l = ii;
                }
            } else if arr[i] <= arr[i - 1] {
                max = std::cmp::max(max, ii - l);
                l = ii;
            }
        }
        max = std::cmp::max(max, n as i32 - l);
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
    assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
    assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
}
