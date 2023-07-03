#![allow(dead_code)]

/*

// 1619. Mean of Array After Removing Some Elements
// https://leetcode.com/problems/mean-of-array-after-removing-some-elements/
// https://leetcode.cn/problems/mean-of-array-after-removing-some-elements/
//
// Easy
//
// Given an integer array arr, return the mean of the remaining integers after removing the smallest 5% and the largest 5% of the elements.

Answers within 10-5 of the actual answer will be considered accepted.

Example 1:

Input: arr = [1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3]
Output: 2.00000
Explanation: After erasing the minimum and the maximum values of this array, all elements are equal to 2, so the mean is 2.

Example 2:

Input: arr = [6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]
Output: 4.00000

Example 3:

Input: arr = [6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]
Output: 4.77778

Constraints:

    20 <= arr.length <= 1000
    arr.length is a multiple of 20.
    0 <= arr[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort();
        let n = arr.len();
        let mut sum = 0;
        for &item in arr.iter().take(n - n / 20).skip(n / 20) {
            sum += item;
        }
        sum as f64 / (n - n / 10) as f64
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3], 2.00000),
        (vec![6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0], 4.00000),
        (
            vec![
                6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4,
            ],
            4.77778,
        ),
    ];
    for (arr, expected) in cases {
        assert!((Solution::trim_mean(arr) - expected).abs() < 1e-5);
    }
}
