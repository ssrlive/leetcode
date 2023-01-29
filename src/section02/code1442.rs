#![allow(dead_code)]

// 1442. Count Triplets That Can Form Two Arrays of Equal XOR
// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/
// https://leetcode.cn/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/
//
// Medium
//
// Given an array of integers arr.
//
// We want to select three indices i, j and k where (0 <= i < j <= k < arr.length).
//
// Let's define a and b as follows:
//
//     a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
//     b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]
//
// Note that ^ denotes the bitwise-xor operation.
//
// Return the number of triplets (i, j and k) Where a == b.
//
// Example 1:
//
// Input: arr = [2,3,1,6,7]
// Output: 4
// Explanation: The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)
//
// Example 2:
//
// Input: arr = [1,1,1,1,1]
// Output: 10
//
// Constraints:
//
// -    1 <= arr.length <= 300
// -    1 <= arr[i] <= 10^8
//

struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..arr.len() {
            let mut x = arr[i];
            for (k, &item) in arr.iter().enumerate().skip(i + 1) {
                x ^= item;
                if x == 0 {
                    ans += k - i;
                }
            }
        }
        ans as i32
    }
}

#[test]
fn test() {
    let cases = vec![(vec![2, 3, 1, 6, 7], 4), (vec![1, 1, 1, 1, 1], 10)];
    for (arr, expected) in cases {
        assert_eq!(Solution::count_triplets(arr), expected);
    }
}
