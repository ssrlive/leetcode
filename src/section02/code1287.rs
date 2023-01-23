#![allow(dead_code)]

// 1287. Element Appearing More Than 25% In Sorted Array
// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/
// https://leetcode.cn/problems/element-appearing-more-than-25-in-sorted-array/
//
// Easy
//
// Given an integer array sorted in non-decreasing order, there is exactly one integer in the array
// that occurs more than 25% of the time, return that integer.
//
// Example 1:
//
// Input: arr = [1,2,2,6,6,6,6,7,10]
// Output: 6
//
// Example 2:
//
// Input: arr = [1,1]
// Output: 1
//
// Constraints:
//
// -    1 <= arr.length <= 10^4
// -    0 <= arr[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && arr[j] == arr[i] {
                j += 1;
            }
            if j - i > n / 4 {
                return arr[i];
            }
            i = j;
        }
        0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]), 6);
    assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
}
