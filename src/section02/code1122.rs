#![allow(dead_code)]

// 1122. Relative Sort Array
// https://leetcode.com/problems/relative-sort-array/
// https://leetcode.cn/problems/relative-sort-array/
//
// Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2 are also in arr1.
//
// Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2.
// Elements that do not appear in arr2 should be placed at the end of arr1 in ascending order.
//
// Example 1:
//
// Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
// Output: [2,2,2,1,4,3,3,9,6,7,19]
//
// Example 2:
//
// Input: arr1 = [28,6,22,8,44,17], arr2 = [22,28,8,6]
// Output: [22,28,8,6,17,44]
//
// Constraints:
//
// - 1 <= arr1.length, arr2.length <= 1000
// - 0 <= arr1[i], arr2[i] <= 1000
// - All the elements of arr2 are distinct.
// Each arr2[i] is in arr1.
//

struct Solution;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut n: Vec<i32> = vec![];
        let mut m: Vec<i32> = vec![];
        for &item2 in &arr2 {
            for &item1 in &arr1 {
                if item2 == item1 {
                    n.push(item1);
                } else {
                    m.push(item1);
                }
            }
        }

        m = {
            let mut k: Vec<i32> = vec![];
            let mut b;
            for &item1 in &arr1 {
                b = false;
                for &item2 in &arr2 {
                    if item1 == item2 {
                        b = true;
                        break;
                    }
                }
                if !b {
                    k.push(item1);
                }
            }
            k
        };
        m.sort();
        for &item in &m {
            n.push(item);
        }
        n
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6],
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19],
        ),
        (
            vec![28, 6, 22, 8, 44, 17],
            vec![22, 28, 8, 6],
            vec![22, 28, 8, 6, 17, 44],
        ),
    ];
    for (arr1, arr2, expected) in cases {
        assert_eq!(Solution::relative_sort_array(arr1, arr2), expected);
    }
}
