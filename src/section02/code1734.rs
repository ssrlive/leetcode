#![allow(dead_code)]

/*

// 1734. Decode XORed Permutation
// https://leetcode.com/problems/decode-xored-permutation/
// https://leetcode.cn/problems/decode-xored-permutation/
//
// Medium
//
// There is an integer array perm that is a permutation of the first n positive integers, where n is always odd.

It was encoded into another integer array encoded of length n - 1, such that encoded[i] = perm[i] XOR perm[i + 1]. For example, if perm = [1,3,2], then encoded = [2,1].

Given the encoded array, return the original array perm. It is guaranteed that the answer exists and is unique.

Example 1:

Input: encoded = [3,1]
Output: [1,2,3]
Explanation: If perm = [1,2,3], then encoded = [1 XOR 2,2 XOR 3] = [3,1]

Example 2:

Input: encoded = [6,5,4,6]
Output: [2,4,1,5,3]

Constraints:

    3 <= n < 10^5
    n is odd.
    encoded.length == n - 1
*/

struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let mut perm = vec![0; n];
        let mut total = 0;
        for i in 1..=n {
            total ^= i as i32;
        }
        let mut odd = 0;
        for i in (1..n - 1).step_by(2) {
            odd ^= encoded[i];
        }
        perm[0] = total ^ odd;
        for i in 0..n - 1 {
            perm[i + 1] = perm[i] ^ encoded[i];
        }
        perm
    }
}

#[test]
fn test() {
    let encoded = vec![3, 1];
    let output = vec![1, 2, 3];
    assert_eq!(Solution::decode(encoded), output);
    let encoded = vec![6, 5, 4, 6];
    let output = vec![2, 4, 1, 5, 3];
    assert_eq!(Solution::decode(encoded), output);
}
