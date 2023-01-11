#![allow(dead_code)]

// 1073. Adding Two Negabinary Numbers
// https://leetcode.com/problems/adding-two-negabinary-numbers/
// https://leetcode.cn/problems/adding-two-negabinary-numbers/
//
// Given two numbers arr1 and arr2 in base -2, return the result of adding them together.
//
// Each number is given in array format:  as an array of 0s and 1s, from most significant bit to least significant bit.
// For example, arr = [1,1,0,1] represents the number (-2)^3 + (-2)^2 + (-2)^0 = -3.
// A number arr in array, format is also guaranteed to have no leading zeros: either arr == [0] or arr[0] == 1.
//
// Return the result of adding arr1 and arr2 in the same format: as an array of 0s and 1s with no leading zeros.
//
// Example 1:
//
// Input: arr1 = [1,1,1,1,1], arr2 = [1,0,1]
// Output: [1,0,0,0,0]
// Explanation: arr1 represents 11, arr2 represents 5, the output represents 16.
//
// Example 2:
//
// Input: arr1 = [0], arr2 = [0]
// Output: [0]
//
// Example 3:
//
// Input: arr1 = [0], arr2 = [1]
// Output: [1]
//
// Constraints:
//
// - 1 <= arr1.length, arr2.length <= 1000
// - arr1[i] and arr2[i] are 0 or 1
// - arr1 and arr2 have no leading zeros
//

struct Solution;

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut i = arr1.len();
        let mut j = arr2.len();
        let mut carry = 0;
        while i > 0 && j > 0 {
            i -= 1;
            j -= 1;
            let tmp = arr1[i] + arr2[j] + carry;
            let digit;
            match tmp {
                -1 => {
                    digit = 1;
                    carry = 1;
                }
                2 => {
                    digit = 0;
                    carry = -1;
                }
                3 => {
                    digit = 1;
                    carry = -1;
                }
                _ => {
                    digit = tmp;
                    carry = 0;
                }
            }
            res.insert(0, digit);
        }
        while i > 0 {
            i -= 1;
            let tmp = arr1[i] + carry;
            let digit;
            match tmp {
                -1 => {
                    digit = 1;
                    carry = 1;
                }
                2 => {
                    digit = 0;
                    carry = -1;
                }
                _ => {
                    digit = tmp;
                    carry = 0;
                }
            }
            res.insert(0, digit);
        }
        while j > 0 {
            j -= 1;
            let tmp = arr2[j] + carry;
            let digit;
            match tmp {
                -1 => {
                    digit = 1;
                    carry = 1;
                }
                2 => {
                    digit = 0;
                    carry = -1;
                }
                _ => {
                    digit = tmp;
                    carry = 0;
                }
            }
            res.insert(0, digit);
        }
        if carry == 1 {
            res.insert(0, 1);
        }
        if carry == -1 {
            res.insert(0, 1);
            res.insert(0, 1);
        }
        while res.len() > 1 && res[0] == 0 {
            res.remove(0);
        }
        res
    }
}

#[test]
fn test() {
    let arr1 = vec![1, 1, 1, 1, 1];
    let arr2 = vec![1, 0, 1];
    let res = vec![1, 0, 0, 0, 0];
    assert_eq!(Solution::add_negabinary(arr1, arr2), res);

    let arr1 = vec![0];
    let arr2 = vec![0];
    let res = vec![0];
    assert_eq!(Solution::add_negabinary(arr1, arr2), res);

    let arr1 = vec![0];
    let arr2 = vec![1];
    let res = vec![1];
    assert_eq!(Solution::add_negabinary(arr1, arr2), res);
}
