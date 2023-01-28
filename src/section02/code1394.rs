#![allow(dead_code)]

// 1394. Find Lucky Integer in an Array
// https://leetcode.com/problems/find-lucky-integer-in-an-array/
// https://leetcode.cn/problems/find-lucky-integer-in-an-array/
//
// Easy
//
// Given an array of integers arr, a lucky integer is an integer that has a frequency in the array equal to its value.
//
// Return the largest lucky integer in the array. If there is no lucky integer return -1.
//
// Example 1:
//
// Input: arr = [2,2,3,4]
// Output: 2
// Explanation: The only lucky number in the array is 2 because frequency[2] == 2.
//
// Example 2:
//
// Input: arr = [1,2,2,3,3,3]
// Output: 3
// Explanation: 1, 2 and 3 are all lucky numbers, return the largest of them.
//
// Example 3:
//
// Input: arr = [2,2,2,3,3]
// Output: -1
// Explanation: There are no lucky numbers in the array.
//
// Constraints:
//
// -    1 <= arr.length <= 500
// -    1 <= arr[i] <= 500
//

struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = vec![0; 501];
        for &x in arr.iter() {
            freq[x as usize] += 1;
        }
        let mut ans = -1;
        for (i, &item) in freq.iter().enumerate().take(500 + 1).skip(1) {
            if item == i {
                ans = i as i32;
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
}
