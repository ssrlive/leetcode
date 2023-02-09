#![allow(dead_code)]

/*

// 1502. Can Make Arithmetic Progression From Sequence
// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
// https://leetcode.cn/problems/can-make-arithmetic-progression-from-sequence/
//
// Easy
//
// A sequence of numbers is called an arithmetic progression if the difference between any two consecutive elements is the same.

Given an array of numbers arr, return true if the array can be rearranged to form an arithmetic progression. Otherwise, return false.

Example 1:

Input: arr = [3,5,1]
Output: true
Explanation: We can reorder the elements as [1,3,5] or [5,3,1] with differences 2 and -2 respectively, between each consecutive elements.

Example 2:

Input: arr = [1,2,4]
Output: false
Explanation: There is no way to reorder the elements to obtain an arithmetic progression.

Constraints:

    2 <= arr.length <= 1000
    -106 <= arr[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort();
        let diff = arr[1] - arr[0];
        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] != diff {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 5, 1], true),
        (vec![1, 2, 4], false),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], true),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11], false),
    ];
    for (arr, expected) in cases {
        assert_eq!(Solution::can_make_arithmetic_progression(arr), expected);
    }
}
