#![allow(dead_code)]

/*

// 1524. Number of Sub-arrays With Odd Sum
// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/
// https://leetcode.cn/problems/number-of-sub-arrays-with-odd-sum/
//
// Medium
//
// Given an array of integers arr, return the number of subarrays with an odd sum.

Since the answer can be very large, return it modulo 109 + 7.

Example 1:

Input: arr = [1,3,5]
Output: 4
Explanation: All subarrays are [[1],[1,3],[1,3,5],[3],[3,5],[5]]
All sub-arrays sum are [1,4,9,3,8,5].
Odd sums are [1,9,3,5] so the answer is 4.

Example 2:

Input: arr = [2,4,6]
Output: 0
Explanation: All subarrays are [[2],[2,4],[2,4,6],[4],[4,6],[6]]
All sub-arrays sum are [2,6,12,4,10,6].
All sub-arrays have even sum and the answer is 0.

Example 3:

Input: arr = [1,2,3,4,5,6,7]
Output: 16

Constraints:

    1 <= arr.length <= 10^5
    1 <= arr[i] <= 100
*/

struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let arr = arr.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mut ans = 0_i64;
        let mut odd = 0;
        let mut even = 1;
        let mut sum = 0;
        for &item in arr.iter() {
            sum += item;
            if sum % 2 == 0 {
                ans += odd;
                even += 1;
            } else {
                ans += even;
                odd += 1;
            }
        }
        (ans % 1_000_000_007) as _
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 3, 5], 4), (vec![2, 4, 6], 0), (vec![1, 2, 3, 4, 5, 6, 7], 16)];
    for (arr, expect) in cases {
        assert_eq!(Solution::num_of_subarrays(arr), expect);
    }
}
