#![allow(dead_code)]

// 1416. Restore The Array
// https://leetcode.com/problems/restore-the-array/
// https://leetcode.cn/problems/restore-the-array/
//
// Hard
//
// A program was supposed to print an array of integers. The program forgot to print whitespaces and the
// array is printed as a string of digits s and all we know is that all integers in the array were
// in the range [1, k] and there are no leading zeros in the array.
//
// Given the string s and the integer k, return the number of the possible arrays that can be printed
// as s using the mentioned program. Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "1000", k = 10000
// Output: 1
// Explanation: The only possible array is [1000]
//
// Example 2:
//
// Input: s = "1000", k = 10
// Output: 0
// Explanation: There cannot be an array that was printed this way and has all integer >= 1 and <= 10.
//
// Example 3:
//
// Input: s = "1317", k = 2000
// Output: 8
// Explanation: Possible arrays are [1317],[131,7],[13,17],[1,317],[13,1,7],[1,31,7],[1,3,17],[1,3,1,7]
//
// Constraints:
//
// -    1 <= s.length <= 10^5
// -    s consists of only digits and does not contain leading zeros.
// -    1 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![0; n + 1];
        dp[n] = 1;
        for i in (0..n).rev() {
            if s[i] == b'0' {
                continue;
            }
            let mut num = 0;
            for j in i..n {
                num = num * 10 + (s[j] - b'0') as usize;
                if num > k {
                    break;
                }
                dp[i] = (dp[i] + dp[j + 1]) % 1000000007;
            }
        }
        dp[0]
    }
}

#[test]
fn test() {
    let cases = vec![
        ("1000", 10000, 1),
        ("1000", 10, 0),
        ("1317", 2000, 8),
        ("2020", 30, 1),
        ("1234567890", 90, 34),
    ];
    for (s, k, expected) in cases {
        assert_eq!(Solution::number_of_arrays(s.to_string(), k), expected);
    }
}
