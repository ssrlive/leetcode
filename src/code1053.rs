#![allow(dead_code)]

// 1053. Previous Permutation With One Swap
// https://leetcode.com/problems/previous-permutation-with-one-swap/
// https://leetcode.cn/problems/previous-permutation-with-one-swap/
//
// Given an array of positive integers arr (not necessarily distinct), return the
// lexicographically
//  largest permutation that is smaller than arr, that can be made with exactly one swap. If it cannot be done, then return the same array.
//
// Note that a swap exchanges the positions of two numbers arr[i] and arr[j]
//
// Example 1:
//
// Input: arr = [3,2,1]
// Output: [3,1,2]
// Explanation: Swapping 2 and 1.
//
// Example 2:
//
// Input: arr = [1,1,5]
// Output: [1,1,5]
// Explanation: This is already the smallest permutation.
//
// Example 3:
//
// Input: arr = [1,9,4,6,7]
// Output: [1,7,4,6,9]
// Explanation: Swapping 9 and 7.
//
// Constraints:
//
// - 1 <= arr.length <= 10^4
// - 1 <= arr[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let (n, mut i) = (arr.len(), arr.len() - 1);

        while i > 0 && arr[i] >= arr[i - 1] {
            i -= 1;
        }

        if i == 0 {
            return arr;
        }

        let mut j = i;

        for k in i..n {
            if arr[k] >= arr[i - 1] || arr[k] <= arr[j] {
                continue;
            }
            j = k;
        }

        let mut arr = arr;
        arr.swap(i - 1, j);

        arr
    }
}

#[test]
fn test() {
    let arr = vec![3, 2, 1];
    let res = vec![3, 1, 2];
    assert_eq!(Solution::prev_perm_opt1(arr), res);

    let arr = vec![1, 1, 5];
    let res = vec![1, 1, 5];
    assert_eq!(Solution::prev_perm_opt1(arr), res);

    let arr = vec![1, 9, 4, 6, 7];
    let res = vec![1, 7, 4, 6, 9];
    assert_eq!(Solution::prev_perm_opt1(arr), res);
}
