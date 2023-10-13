#![allow(dead_code)]

// 1346. Check If N and Its Double Exist
// https://leetcode.com/problems/check-if-n-and-its-double-exist/
// https://leetcode.cn/problems/check-if-n-and-its-double-exist/
//
// Easy
//
// Given an array arr of integers, check if there exist two indices i and j such that :
//
//     i != j
//     0 <= i, j < arr.length
//     arr[i] == 2 * arr[j]
//
// Example 1:
//
// Input: arr = [10,2,5,3]
// Output: true
// Explanation: For i = 0 and j = 2, arr[i] == 10 == 2 * 5 == 2 * arr[j]
//
// Example 2:
//
// Input: arr = [3,1,7,11]
// Output: false
// Explanation: There is no i and j that satisfy the conditions.
//
// Constraints:
//
// -    2 <= arr.length <= 500
// -    -10^3 <= arr[i] <= 10^3
//

struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut freqs = std::collections::HashMap::new();
        for item in arr.iter() {
            *freqs.entry(item).or_insert(0) += 1;
        }

        for &item in arr.iter() {
            if item == 0 {
                if freqs[&0] > 1 {
                    return true;
                }
            } else if freqs.contains_key(&(item * 2)) {
                return true;
            }
        }

        false
    }
}

#[test]
fn test() {
    let arr = vec![10, 2, 5, 3];
    assert!(Solution::check_if_exist(arr));

    let arr = vec![3, 1, 7, 11];
    assert!(!Solution::check_if_exist(arr));
}
