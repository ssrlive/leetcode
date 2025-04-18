#![allow(dead_code)]

// 3519. Count Numbers with Non-Decreasing Digits
// https://leetcode.com/problems/count-numbers-with-non-decreasing-digits/
// https://leetcode.cn/problems/count-numbers-with-non-decreasing-digits/
//
// Hard
//
// You are given two integers, l and r, represented as strings, and an integer b. Return the count of integers
// in the inclusive range [l, r] whose digits are in non-decreasing order when represented in base b.
//
// An integer is considered to have non-decreasing digits if, when read from left to right (from the most significant
// digit to the least significant digit), each digit is greater than or equal to the previous one.
//
// Since the answer may be too large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: l = "23", r = "28", b = 8
//
// Output: 3
//
// Explanation:
//
//     The numbers from 23 to 28 in base 8 are: 27, 30, 31, 32, 33, and 34.
//     Out of these, 27, 33, and 34 have non-decreasing digits. Hence, the output is 3.
//
// Example 2:
//
// Input: l = "2", r = "7", b = 2
//
// Output: 2
//
// Explanation:
//
//     The numbers from 2 to 7 in base 2 are: 10, 11, 100, 101, 110, and 111.
//     Out of these, 11 and 111 have non-decreasing digits. Hence, the output is 2.
//
// Constraints:
//
//     1 <= l.length <= r.length <= 100
//     2 <= b <= 10
//     l and r consist only of digits.
//     The value represented by l is less than or equal to the value represented by r.
//     l and r do not contain leading zeros.
//

struct Solution;

impl Solution {
    pub fn count_numbers(l: String, r: String, b: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; 10]; 2]; 401];
        const MOD: i64 = 1_000_000_007;

        fn subtract_one(s: &str) -> String {
            let mut s = s.chars().collect::<Vec<_>>();
            let mut i = s.len() as i32 - 1;
            while i >= 0 {
                if s[i as usize] > '0' {
                    s[i as usize] = (s[i as usize] as u8 - 1) as char;
                    break;
                }
                s[i as usize] = '9';
                i -= 1;
            }
            if s[0] == '0' && s.len() > 1 {
                s.remove(0);
            }
            s.iter().collect()
        }

        fn divmod_string(num: &str, base: i32, rem_out: &mut i32) -> String {
            let mut quotient = String::new();
            let mut carry = 0;
            for c in num.chars() {
                let d = c.to_digit(10).unwrap();
                let cur = carry * 10 + d;
                let q = cur / base as u32;
                carry = cur % base as u32;
                if !quotient.is_empty() || q != 0 {
                    quotient.push((q as u8 + b'0') as char);
                }
            }
            *rem_out = carry as i32;
            if quotient.is_empty() { "0".to_string() } else { quotient }
        }

        fn convert_to_base_x(decimal_str: &str, base: i32) -> String {
            if decimal_str == "0" {
                return "0".to_string();
            }
            let digits = "0123456789";
            let mut n = decimal_str.to_string();
            let mut result = String::new();
            while n != "0" {
                let mut rem: i32 = 0;
                n = divmod_string(&n, base, &mut rem);
                result.push(digits.chars().nth(rem as usize).unwrap());
            }
            result.chars().rev().collect()
        }

        fn f(num: &str, pos: usize, tight: bool, base: i32, prev_digit: usize, dp: &mut Vec<Vec<Vec<i64>>>) -> i64 {
            if pos == num.len() {
                return 1;
            }
            let res = &mut dp[pos][tight as usize][prev_digit];
            if *res != -1 {
                return *res;
            }
            *res = 0;
            let ub = if tight {
                (num.chars().nth(pos).unwrap() as u8 - b'0') as i32
            } else {
                base - 1
            };
            let mut res = 0;
            for d in prev_digit as i32..=ub {
                res += f(num, pos + 1, tight && (d == ub), base, d as usize, dp);
                dp[pos][tight as usize][prev_digit] = res;
            }
            res
        }

        let l1 = if l == "0" { "0".to_string() } else { subtract_one(&l) };
        let lb = convert_to_base_x(&l1, b);
        let rb = convert_to_base_x(&r, b);
        dp.iter_mut().for_each(|x| x.iter_mut().for_each(|y| y.fill(-1)));
        let cnt_r = f(&rb, 0, true, b, 0, &mut dp);
        dp.iter_mut().for_each(|x| x.iter_mut().for_each(|y| y.fill(-1)));
        let cnt_l = f(&lb, 0, true, b, 0, &mut dp);
        let ans = (cnt_r - cnt_l + MOD) % MOD;
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_numbers("23".to_string(), "28".to_string(), 8), 3);
    assert_eq!(Solution::count_numbers("2".to_string(), "7".to_string(), 2), 2);
}
