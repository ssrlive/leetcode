#![allow(dead_code)]

// 852. Peak Index in a Mountain Array
// https://leetcode.com/problems/peak-index-in-a-mountain-array/
// https://leetcode.cn/problems/peak-index-in-a-mountain-array/
//
// An array arr a mountain if the following properties hold:
//
// arr.length >= 3
// There exists some i with 0 < i < arr.length - 1 such that:
// arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
// arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
// Given a mountain array arr, return the index i such that arr[0] < arr[1] < ... < arr[i - 1] < arr[i] > arr[i + 1] > ... > arr[arr.length - 1].
//
// You must solve it in O(log(arr.length)) time complexity.
//
// Example 1:
//
// Input: arr = [0,1,0]
// Output: 1
//
// Example 2:
//
// Input: arr = [0,2,1,0]
// Output: 1
//
// Example 3:
//
// Input: arr = [0,10,5,2]
// Output: 1
//
// Constraints:
//
// - 3 <= arr.length <= 10^5
// - 0 <= arr[i] <= 10^6
// - arr is guaranteed to be a mountain array.
//

struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        arr.iter().zip(arr[1..].iter()).position(|(a, b)| a > b).unwrap() as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
}
