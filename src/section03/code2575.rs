#![allow(dead_code)]

/*

// 2575. Find the Divisibility Array of a String
// https://leetcode.com/problems/find-the-divisibility-array-of-a-string/
// https://leetcode.cn/problems/find-the-divisibility-array-of-a-string/
//
// Medium
//
// You are given a 0-indexed string word of length n consisting of digits, and a positive integer m.

The divisibility array div of word is an integer array of length n such that:

    div[i] = 1 if the numeric value of word[0,...,i] is divisible by m, or
    div[i] = 0 otherwise.

Return the divisibility array of word.

Example 1:

Input: word = "998244353", m = 3
Output: [1,1,0,0,0,1,1,0,0]
Explanation: There are only 4 prefixes that are divisible by 3: "9", "99", "998244", and "9982443".

Example 2:

Input: word = "1010", m = 10
Output: [0,1,0,1]
Explanation: There are only 2 prefixes that are divisible by 10: "10", and "1010".

Constraints:

    1 <= n <= 10^5
    word.length == n
    word consists of digits from 0 to 9
    1 <= m <= 10^9
*/

struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let (mut a, mut ret, m) = (0, vec![], m as i64);
        for c in word.chars() {
            a = (a * 10 + c as i64 - '0' as i64) % m;
            ret.push(if a == 0 { 1 } else { 0 });
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![("998244353", 3, vec![1, 1, 0, 0, 0, 1, 1, 0, 0]), ("1010", 10, vec![0, 1, 0, 1])];
    for (word, m, expect) in cases {
        assert_eq!(Solution::divisibility_array(word.to_string(), m), expect);
    }
}
