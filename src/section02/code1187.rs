#![allow(dead_code)]

// 1187. Make Array Strictly Increasing
// https://leetcode.com/problems/make-array-strictly-increasing/
// https://leetcode.cn/problems/make-array-strictly-increasing/
//
// Given two integer arrays arr1 and arr2, return the minimum number of operations (possibly zero) needed to make arr1 strictly increasing.
//
// In one operation, you can choose two indices 0 <= i < arr1.length and 0 <= j < arr2.length and do the assignment arr1[i] = arr2[j].
//
// If there is no way to make arr1 strictly increasing, return -1.
//
// Example 1:
//
// Input: arr1 = [1,5,3,6,7], arr2 = [1,3,2,4]
// Output: 1
// Explanation: Replace 5 with 2, then arr1 = [1, 2, 3, 6, 7].
//
// Example 2:
//
// Input: arr1 = [1,5,3,6,7], arr2 = [4,3,1]
// Output: 2
// Explanation: Replace 5 with 3 and then replace 3 with 4. arr1 = [1, 3, 4, 6, 7].
//
// Example 3:
//
// Input: arr1 = [1,5,3,6,7], arr2 = [1,6,3,3]
// Output: -1
// Explanation: You can't make arr1 strictly increasing.
//
// Constraints:
//
// - 1 <= arr1.length, arr2.length <= 2000
// - 0 <= arr1[i], arr2[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;
        arr2.sort();
        arr2.dedup();
        let m = arr1.len();
        let n = arr2.len();
        let mut f = vec![vec![i32::MAX; n]; m];
        let mut g = vec![i32::MAX; m];
        for i in 0..n {
            f[0][i] = 1;
        }
        g[0] = 0;
        for i in 1..m {
            for (j, &item) in arr2.iter().enumerate() {
                if item > arr1[i - 1] {
                    f[i][j] = std::cmp::min(g[i - 1], i32::MAX - 1) + 1;
                }
                if j > 0 {
                    f[i][j] = std::cmp::min(f[i][j], std::cmp::min(f[i - 1][j - 1], i32::MAX - 1) + 1);
                }
            }
            if arr1[i] > arr1[i - 1] {
                g[i] = g[i - 1];
            } else {
                g[i] = i32::MAX;
            }
            for (j, &item) in arr2.iter().enumerate() {
                if item < arr1[i] {
                    g[i] = std::cmp::min(g[i], f[i - 1][j]);
                }
            }
        }
        let ans = std::cmp::min(g[m - 1], *f[m - 1].iter().min().unwrap());
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

#[test]
fn test() {
    let arr1 = vec![1, 5, 3, 6, 7];
    let arr2 = vec![1, 3, 2, 4];
    assert_eq!(Solution::make_array_increasing(arr1, arr2), 1);

    let arr1 = vec![1, 5, 3, 6, 7];
    let arr2 = vec![4, 3, 1];
    assert_eq!(Solution::make_array_increasing(arr1, arr2), 2);

    let arr1 = vec![1, 5, 3, 6, 7];
    let arr2 = vec![1, 6, 3, 3];
    assert_eq!(Solution::make_array_increasing(arr1, arr2), -1);
}
