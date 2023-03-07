#![allow(dead_code)]

/*

// 2384. Largest Palindromic Number
// https://leetcode.com/problems/largest-palindromic-number/
// https://leetcode.cn/problems/largest-palindromic-number/
//
// Medium
//
// You are given a string num consisting of digits only.

Return the largest palindromic integer (in the form of a string) that can be formed using digits taken from num. It should not contain leading zeroes.

Notes:

    You do not need to use all the digits of num, but you must use at least one digit.
    The digits can be reordered.

Example 1:

Input: num = "444947137"
Output: "7449447"
Explanation:
Use the digits "4449477" from "444947137" to form the palindromic integer "7449447".
It can be shown that "7449447" is the largest palindromic integer that can be formed.

Example 2:

Input: num = "00009"
Output: "9"
Explanation:
It can be shown that "9" is the largest palindromic integer that can be formed.
Note that the integer returned should not contain leading zeroes.

Constraints:

    1 <= num.length <= 10^5
    num consists of digits.
*/

struct Solution;

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut res = Vec::with_capacity(num.len());
        let mut mid = None;
        let mut counter = [0; 10];
        num.bytes().for_each(|b| counter[(b - b'0') as usize] += 1);
        for (i, n) in counter.iter().enumerate().filter(|(_, &n)| n > 0).rev() {
            let b = i as u8 + b'0';
            if mid.is_none() && n % 2 == 1 {
                mid = Some(b);
            }
            if b == b'0' {
                match (res.is_empty(), mid) {
                    (true, Some(b)) => return String::from_utf8(vec![b]).unwrap(),
                    (true, None) => return "0".to_string(),
                    _ => (),
                }
            }
            (0..n / 2).for_each(|_| res.push(b));
        }
        let l = res.len();
        if let Some(mid) = mid {
            res.push(mid);
        }
        (0..l).rev().for_each(|i| res.push(res[i]));
        String::from_utf8(res).unwrap()
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("444947137", "7449447"),
        ("00009", "9"),
        ("0000", "0"),
        ("0001", "1"),
    ];
    for (num, expect) in cases {
        assert_eq!(Solution::largest_palindromic(num.to_string()), expect);
    }
}
