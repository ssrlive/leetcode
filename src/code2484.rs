#![allow(dead_code)]

// 2484. Count Palindromic Subsequences
// https://leetcode.com/problems/count-palindromic-subsequences/
// https://leetcode.cn/problems/count-palindromic-subsequences/
//
// Given a string of digits s, return the number of palindromic subsequences of s having length 5. Since the answer may be very large, return it modulo 109 + 7.
//
// Note:
//
// - A string is palindromic if it reads the same forward and backward.
// - A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
//
// Example 1:
//
// - Input: s = "103301"
// - Output: 2
// - Explanation:
// - There are 6 possible subsequences of length 5: "10330","10331","10301","10301","13301","03301".
// - Two of them (both equal to "10301") are palindromic.
//
// Example 2:
//
// Input: s = "0000000"
// Output: 21
// Explanation: All 21 subsequences are "00000", which is palindromic.
//
// Example 3:
//
// Input: s = "9999900000"
// Output: 2
// Explanation: The only two palindromic subsequences are "99999" and "00000".
//
// Constraints:
//
// - 1 <= s.length <= 10^4
// - s consists of digits.
//

/*
// java solution

class Solution {
    public int countPalindromes(String s) {
        final int PALINDROME_LENGTH = 5;
        final int MODULO = (int) Math.pow(10, 9) + 7;

        long totalPalindromesWithLengthFive = 0;

        for (int first = 0; first <= 9; ++first) {
            for (int second = 0; second <= 9; ++second) {

                int[] palindrome = {first, second, 0, second, first};
                long[] memo = {0, 0, 0, 0, 0, 1};

                for (int i = 0; i < s.length(); ++i) {
                    for (int n = 0; n < PALINDROME_LENGTH; ++n) {

                        int digit = s.charAt(i) - '0';
                        if (digit == palindrome[n] || n == 2) {
                            memo[n] = (memo[n] + memo[n + 1]) % MODULO;
                        }
                    }
                }
                totalPalindromesWithLengthFive = (totalPalindromesWithLengthFive + memo[0]) % MODULO;
            }
        }
        return (int) totalPalindromesWithLengthFive;
    }
}

*/

// rust solution
struct Solution;
impl Solution {
    pub fn count_palindromes(s: String) -> i32 {
        const PALINDROME_LENGTH: usize = 5;
        const MODULO: i32 = 1_000_000_007;

        let mut result = 0;

        for first in 0..=9 {
            for second in 0..=9 {
                let palindrome = [first, second, 0, second, first];
                let mut memo = [0, 0, 0, 0, 0, 1];

                for i in 0..s.len() {
                    for n in 0..PALINDROME_LENGTH {
                        let digit = s.as_bytes()[i] - b'0';
                        if digit == palindrome[n] || n == 2 {
                            memo[n] = (memo[n] + memo[n + 1]) % MODULO;
                        }
                    }
                }
                result = (result + memo[0]) % MODULO;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_palindromes("103301".to_string()), 2);
    assert_eq!(Solution::count_palindromes("0000000".to_string()), 21);
    assert_eq!(Solution::count_palindromes("9999900000".to_string()), 2);
}
