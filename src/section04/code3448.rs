#![allow(dead_code)]

// 3448. Count Substrings Divisible By Last Digit
// https://leetcode.com/problems/count-substrings-divisible-by-last-digit/
// https://leetcode.cn/problems/count-substrings-divisible-by-last-digit/
//
// Hard
//
// You are given a string s consisting of digits.
//
// Return the number of substrings of s divisible by their non-zero last digit.
//
// Note: A substring may contain leading zeros.
//
// Example 1:
//
// Input: s = "12936"
//
// Output: 11
//
// Explanation:
//
// Substrings "29", "129", "293" and "2936" are not divisible by their last digit.
// There are 15 substrings in total, so the answer is 15 - 4 = 11.
//
// Example 2:
//
// Input: s = "5701283"
//
// Output: 18
//
// Explanation:
//
// Substrings "01", "12", "701", "012", "128", "5701", "7012", "0128", "57012", "70128", "570128", and "701283" are all
// divisible by their last digit. Additionally, all substrings that are just 1 non-zero digit are divisible by themselves.
// Since there are 6 such digits, the answer is 12 + 6 = 18.
//
// Example 3:
//
// Input: s = "1010101010"
//
// Output: 25
//
// Explanation:
//
// Only substrings that end with digit '1' are divisible by their last digit. There are 25 such substrings.
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     s consists of digits only.
//

struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let n = s.len();
        let mut ans: i64 = 0;
        let mut p3 = vec![0; n];
        let mut p7 = vec![0; n];
        let mut p9 = vec![0; n];
        let s_bytes = s.as_bytes();

        p3[0] = (s_bytes[0] - b'0') as i32 % 3;
        p7[0] = (s_bytes[0] - b'0') as i32 % 7;
        p9[0] = (s_bytes[0] - b'0') as i32 % 9;

        for i in 1..n {
            let dig = (s_bytes[i] - b'0') as i32;
            p3[i] = (p3[i - 1] * 10 + dig) % 3;
            p7[i] = (p7[i - 1] * 10 + dig) % 7;
            p9[i] = (p9[i - 1] * 10 + dig) % 9;
        }

        let mut freq3 = [0i64; 3];
        let mut freq9 = [0i64; 9];
        let mut freq7 = vec![vec![0i64; 7]; 6];
        let inv7 = [1, 5, 4, 6, 2, 3];

        for j in 0..n {
            let d = (s_bytes[j] - b'0') as i32;
            if d == 0 {
                // Skip 0 as last digit
            } else if d == 1 || d == 2 || d == 5 {
                ans += (j + 1) as i64;
            } else if d == 4 {
                if j == 0 {
                    ans += 1;
                } else {
                    let num = ((s_bytes[j - 1] - b'0') as i32) * 10 + d;
                    ans += if num % 4 == 0 { (j + 1) as i64 } else { 1 };
                }
            } else if d == 8 {
                if j == 0 {
                    ans += 1;
                } else if j == 1 {
                    let num = ((s_bytes[0] - b'0') as i32) * 10 + 8;
                    ans += if num % 8 == 0 { 2 } else { 1 };
                } else {
                    let num3 = ((s_bytes[j - 2] - b'0') as i32) * 100 + ((s_bytes[j - 1] - b'0') as i32) * 10 + 8;
                    let num2 = ((s_bytes[j - 1] - b'0') as i32) * 10 + 8;
                    ans += if num3 % 8 == 0 { (j - 1) as i64 } else { 0 } + if num2 % 8 == 0 { 1 } else { 0 } + 1;
                }
            } else if d == 3 || d == 6 {
                ans += if p3[j] == 0 { 1 } else { 0 } + freq3[p3[j] as usize];
            } else if d == 7 {
                ans += if p7[j] == 0 { 1 } else { 0 };
                for (m, &inv7_m) in inv7.iter().enumerate() {
                    let idx = ((j % 6) as i32 - m as i32 + 6) % 6;
                    let req = (p7[j] * inv7_m) % 7;
                    ans += freq7[idx as usize][req as usize];
                }
            } else if d == 9 {
                ans += if p9[j] == 0 { 1 } else { 0 } + freq9[p9[j] as usize];
            }
            freq3[p3[j] as usize] += 1;
            freq7[j % 6][p7[j] as usize] += 1;
            freq9[p9[j] as usize] += 1;
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_substrings("12936".to_string()), 11);
    assert_eq!(Solution::count_substrings("5701283".to_string()), 18);
    assert_eq!(Solution::count_substrings("1010101010".to_string()), 25);
}
