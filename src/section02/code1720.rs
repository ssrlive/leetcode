#![allow(dead_code)]

/*

// 1720. Decode XORed Array
// https://leetcode.com/problems/decode-xored-array/
// https://leetcode.cn/problems/decode-xored-array/
//
// Easy
//
// There is a hidden integer array arr that consists of n non-negative integers.

It was encoded into another integer array encoded of length n - 1, such that encoded[i] = arr[i] XOR arr[i + 1]. For example, if arr = [1,0,2,1], then encoded = [1,2,3].

You are given the encoded array. You are also given an integer first, that is the first element of arr, i.e. arr[0].

Return the original array arr. It can be proved that the answer exists and is unique.

Example 1:

Input: encoded = [1,2,3], first = 1
Output: [1,0,2,1]
Explanation: If arr = [1,0,2,1], then first = 1 and encoded = [1 XOR 0, 0 XOR 2, 2 XOR 1] = [1,2,3]

Example 2:

Input: encoded = [6,2,7,3], first = 4
Output: [4,2,0,7,4]

Constraints:

    2 <= n <= 10^4
    encoded.length == n - 1
    0 <= encoded[i] <= 10^5
    0 <= first <= 10^5
*/

struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut res = vec![first];
        for i in 0..encoded.len() {
            res.push(res[i] ^ encoded[i]);
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3], 1, vec![1, 0, 2, 1]),
        (vec![6, 2, 7, 3], 4, vec![4, 2, 0, 7, 4]),
    ];
    for (encoded, first, expected) in cases {
        assert_eq!(Solution::decode(encoded, first), expected);
    }
}
