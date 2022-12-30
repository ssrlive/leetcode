#![allow(dead_code)]

// 898. Bitwise ORs of Subarrays
// https://leetcode.com/problems/bitwise-ors-of-subarrays/
// https://leetcode.cn/problems/bitwise-ors-of-subarrays/
//
// Given an integer array arr, return the number of distinct bitwise ORs of all the non-empty subarrays of arr.
//
// The bitwise OR of a subarray is the bitwise OR of each integer in the subarray. The bitwise OR of a subarray of one integer is that integer.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: arr = [0]
// Output: 1
// Explanation: There is only one possible result: 0.
//
// Example 2:
//
// Input: arr = [1,1,2]
// Output: 3
// Explanation: The possible subarrays are [1], [1], [2], [1, 1], [1, 2], [1, 1, 2].
// These yield the results 1, 1, 2, 1, 3, 3.
// There are 3 unique values, so the answer is 3.
//
// Example 3:
//
// Input: arr = [1,2,4]
// Output: 6
// Explanation: The possible results are 1, 2, 3, 4, 6, and 7.
//
// Constraints:
//
// - 1 <= arr.length <= 5 * 10^4
// - 0 <= arr[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let n = arr.len();
        set.insert(arr[0]);
        for i in 1..n {
            let mut pre_or = 0;
            let mut curr_or = arr[i];
            let mut j = (i - 1) as i32;
            set.insert(curr_or);
            while 0 <= j && curr_or != pre_or {
                let uj = j as usize;
                curr_or |= arr[uj];
                pre_or |= arr[uj];
                set.insert(curr_or);
                j -= 1;
            }
        }
        set.len() as i32
    }
}

#[test]
fn test() {
    let arr = vec![1, 1, 2];
    let res = Solution::subarray_bitwise_o_rs(arr);
    assert_eq!(res, 3);

    let arr = vec![1, 2, 4];
    let res = Solution::subarray_bitwise_o_rs(arr);
    assert_eq!(res, 6);
}
