#![allow(dead_code)]

/*

// 1539. Kth Missing Positive Number
// https://leetcode.com/problems/kth-missing-positive-number/
// https://leetcode.cn/problems/kth-missing-positive-number/
//
// Easy
//
// Given an array arr of positive integers sorted in a strictly increasing order, and an integer k.

Return the kth positive integer that is missing from this array.

Example 1:

Input: arr = [2,3,4,7,11], k = 5
Output: 9
Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5th missing positive integer is 9.

Example 2:

Input: arr = [1,2,3,4], k = 2
Output: 6
Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.

Constraints:

    1 <= arr.length <= 1000
    1 <= arr[i] <= 1000
    1 <= k <= 1000
    arr[i] < arr[j] for 1 <= i < j <= arr.length

Follow up:

Could you solve this problem in less than O(n) complexity?
*/

struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut missing = 0;
        let mut i = 0;
        let mut j = 1;
        while missing < k {
            if i < arr.len() && arr[i] == j {
                i += 1;
            } else {
                missing += 1;
            }
            j += 1;
        }
        j - 1
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 3, 4, 7, 11], 5, 9),
        (vec![1, 2, 3, 4], 2, 6),
        (vec![1, 2, 3, 4], 3, 7),
    ];
    for (arr, k, expected) in cases {
        assert_eq!(Solution::find_kth_positive(arr, k), expected);
    }
}
