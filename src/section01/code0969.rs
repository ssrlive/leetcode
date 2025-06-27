#![allow(dead_code)]

// 969. Pancake Sorting
// https://leetcode.com/problems/pancake-sorting/
// https://leetcode.cn/problems/pancake-sorting/
//
// Given an array of integers arr, sort the array by performing a series of pancake flips.
//
// In one pancake flip we do the following steps:
//
// - Choose an integer k where 1 <= k <= arr.length.
// - Reverse the sub-array arr[0...k-1] (0-indexed).
//
// For example, if arr = [3,2,1,4] and we performed a pancake flip choosing k = 3, we reverse the sub-array [3,2,1],
// so arr = [1,2,3,4] after the pancake flip at k = 3.
//
// Return an array of the k-values corresponding to a sequence of pancake flips that sort arr. Any valid answer
// that sorts the array within 10 * arr.length flips will be judged as correct.
//
// Example 1:
//
// Input: arr = [3,2,4,1]
// Output: [4,2,4,3]
// Explanation:
// We perform 4 pancake flips, with k values 4, 2, 4, and 3.
// Starting state: arr = [3, 2, 4, 1]
// After 1st flip (k = 4): arr = [1, 4, 2, 3]
// After 2nd flip (k = 2): arr = [4, 1, 2, 3]
// After 3rd flip (k = 4): arr = [3, 2, 1, 4]
// After 4th flip (k = 3): arr = [1, 2, 3, 4], which is sorted.
//
// Example 2:
//
// Input: arr = [1,2,3]
// Output: []
// Explanation: The input is already sorted, so there is no need to flip anything.
// Note that other answers, such as [3, 3], would also be accepted.
//
// Constraints:
//
// - 1 <= arr.length <= 100
// - 1 <= arr[i] <= arr.length
// - All integers in arr are unique (i.e. arr is a permutation of the integers from 1 to arr.length).
//

struct Solution;

impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let n = arr.len();
        let mut base = arr.clone();
        base.sort();
        let mut result = vec![];
        for i in (0..n).rev() {
            let tv = base[i];
            if arr[i] == tv {
                continue;
            }
            let mut temp = vec![];
            for (j, &item) in arr.iter().enumerate() {
                let v = item;
                temp.push(v);
                if v == tv {
                    result.push((j + 1) as i32);
                    result.push((i + 1) as i32);
                    break;
                }
            }
            temp.reverse();
            arr[..temp.len()].copy_from_slice(&temp[..]);
            let mut temp = vec![];
            for &item in arr.iter().take(i + 1) {
                temp.push(item);
            }
            temp.reverse();
            arr[..temp.len()].copy_from_slice(&temp[..]);
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![(vec![3, 2, 4, 1], vec![4, 2, 4, 3]), (vec![1, 2, 3], vec![])];
    for (arr, _expect) in cases {
        let result = Solution::pancake_sort(arr);
        println!("result = {result:?}");
    }
}
