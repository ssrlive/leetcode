#![allow(dead_code)]

// 1089. Duplicate Zeros
// https://leetcode.com/problems/duplicate-zeros/
// https://leetcode.cn/problems/duplicate-zeros/
//
// Given a fixed-length integer array arr, duplicate each occurrence of zero, shifting the remaining elements to the right.
//
// Note that elements beyond the length of the original array are not written.
// Do the above modifications to the input array in place and do not return anything.
//
// Example 1:
//
// Input: arr = [1,0,2,3,0,4,5,0]
// Output: [1,0,0,2,3,0,0,4]
// Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
//
// Example 2:
//
// Input: arr = [1,2,3]
// Output: [1,2,3]
// Explanation: After calling your function, the input array is modified to: [1,2,3]
//
// Constraints:
//
// - 1 <= arr.length <= 10^4
// - 0 <= arr[i] <= 9
//

struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut skip = 0;
        let len = arr.len();
        for i in 0..=(arr.len() - 1) {
            if arr[i] == 0 && i != arr.len() - 1 && skip != 1 {
                arr.insert(i + 1, 0);
                skip += 1
            } else {
                skip = 0;
            }
            if arr.len() > len {
                arr.pop();
            }
        }
    }
}

#[test]
fn test() {
    let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
    Solution::duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![1, 0, 0, 2, 3, 0, 0, 4]);

    let mut arr = vec![1, 2, 3];
    Solution::duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![1, 2, 3]);
}
