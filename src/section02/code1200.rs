#![allow(dead_code)]

// 1200. Minimum Absolute Difference
// https://leetcode.com/problems/minimum-absolute-difference/
// https://leetcode.cn/problems/minimum-absolute-difference/
//
// Given an array of distinct integers arr, find all pairs of elements with the minimum absolute difference of any two elements.
//
// Return a list of pairs in ascending order(with respect to pairs), each pair [a, b] follows
//
// a, b are from arr
// a < b
// b - a equals to the minimum absolute difference of any two elements in arr
//
// Example 1:
//
// Input: arr = [4,2,1,3]
// Output: [[1,2],[2,3],[3,4]]
// Explanation: The minimum absolute difference is 1. List all pairs with difference equal to 1 in ascending order.
//
// Example 2:
//
// Input: arr = [1,3,6,10,15]
// Output: [[1,3]]
//
// Example 3:
//
// Input: arr = [3,8,-10,23,19,-4,-14,27]
// Output: [[-14,-10],[19,23],[23,27]]
//
// Constraints:
//
// - 2 <= arr.length <= 10^5
// - -10^6 <= arr[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort();
        let mut min = i32::MAX;
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff < min {
                min = diff;
            }
        }
        let mut result = Vec::new();
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff == min {
                result.push(vec![arr[i - 1], arr[i]]);
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![4, 2, 1, 3], vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        (vec![1, 3, 6, 10, 15], vec![vec![1, 3]]),
        (
            vec![3, 8, -10, 23, 19, -4, -14, 27],
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]],
        ),
    ];
    for (arr, expected) in cases {
        assert_eq!(Solution::minimum_abs_difference(arr), expected);
    }
}
