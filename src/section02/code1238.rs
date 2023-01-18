#![allow(dead_code)]

// 1238. Circular Permutation in Binary Representation
// https://leetcode.com/problems/circular-permutation-in-binary-representation/
// https://leetcode.cn/problems/circular-permutation-in-binary-representation/
//
// Medium
//
// Given 2 integers n and start. Your task is return any permutation p of (0,1,2.....,2^n -1) such that :
//
//     p[0] = start
//     p[i] and p[i+1] differ by only one bit in their binary representation.
//     p[0] and p[2^n -1] must also differ by only one bit in their binary representation.
//
// Example 1:
//
// Input: n = 2, start = 3
// Output: [3,2,0,1]
// Explanation: The binary representation of the permutation is (11,10,00,01).
// All the adjacent element differ by one bit. Another valid permutation is [3,1,0,2]
//
// Example 2:
//
// Input: n = 3, start = 2
// Output: [2,6,7,5,4,0,1,3]
// Explanation: The binary representation of the permutation is (010,110,111,101,100,000,001,011).
//
// Constraints:
//
// -    1 <= n <= 16
// -    0 <= start < 2 ^ n
//

struct Solution;

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..1 << n {
            res.push(start ^ i ^ i >> 1);
        }
        res
    }
}

#[test]
fn test() {
    let n = 2;
    let start = 3;
    let output = vec![3, 2, 0, 1];
    assert_eq!(Solution::circular_permutation(n, start), output);

    let n = 3;
    let start = 2;
    let output = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mut result = Solution::circular_permutation(n, start);
    result.sort();
    assert_eq!(result, output);
}
