#![allow(dead_code)]

/*

// 1588. Sum of All Odd Length Subarrays
Easy
3K
224
Companies

Given an array of positive integers arr, return the sum of all possible odd-length subarrays of arr.

A subarray is a contiguous subsequence of the array.

Example 1:

Input: arr = [1,4,2,5,3]
Output: 58
Explanation: The odd-length subarrays of arr and their sums are:
[1] = 1
[4] = 4
[2] = 2
[5] = 5
[3] = 3
[1,4,2] = 7
[4,2,5] = 11
[2,5,3] = 10
[1,4,2,5,3] = 15
If we add all these together we get 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58

Example 2:

Input: arr = [1,2]
Output: 3
Explanation: There are only 2 subarrays of odd length, [1] and [2]. Their sum is 3.

Example 3:

Input: arr = [10,11,12]
Output: 66

Constraints:

    1 <= arr.length <= 100
    1 <= arr[i] <= 1000

Follow up:

Could you solve this problem in O(n) time complexity?
*/

struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut ans = 0;
        for (i, &item) in arr.iter().enumerate() {
            let l = i + 1;
            let r = n - i;
            let l_odd = l.div_ceil(2);
            let l_even = l / 2;
            let r_odd = r.div_ceil(2);
            let r_even = r / 2;
            ans += item as usize * (l_odd * r_odd + l_even * r_even);
        }
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
}
