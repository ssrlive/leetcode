#![allow(dead_code)]

// 273. Integer to English Words
// https://leetcode.com/problems/integer-to-english-words/
//
// Convert a non-negative integer to its english words representation. Given
// input is guaranteed to be less than 231 - 1.
//
// Example 1:
//
// Input: 123
// Output: "One Hundred Twenty Three"
//
// Example 2:
//
// Input: 12345
// Output: "Twelve Thousand Three Hundred Forty Five"
//
// Example 3:
//
// Input: 1234567
// Output: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
//
// Example 4:
//
// Input: 1234567891
// Output: "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
//
// Constraints:
//
// - 0 <= num <= 2^31 - 1
//

struct Solution;

impl Solution {
    const WORD_TABLE: [&'static str; 28] = [
        "Zero",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
        "Twenty",
        "Thirty",
        "Forty",
        "Fifty",
        "Sixty",
        "Seventy",
        "Eighty",
        "Ninety",
    ];
    const GROUP_TABLE: [&'static str; 3] = [" Billion", " Million", " Thousand"];
    const GROUP_VALUES: [i32; 3] = [1000000000, 1000000, 1000];
    fn do_hundred(out: &mut String, mut num: i32) {
        if num >= 100 {
            out.push_str(Self::WORD_TABLE[(num / 100) as usize]);
            out.push_str(" Hundred");
            num %= 100;
            if num == 0 {
                return;
            }
            out.push(' ');
        }
        if num > 20 {
            out.push_str(Self::WORD_TABLE[(20 + (num - 20) / 10) as usize]);
            if num % 10 != 0 {
                out.push(' ');
                out.push_str(Self::WORD_TABLE[(num % 10) as usize]);
            }
        } else {
            out.push_str(Self::WORD_TABLE[num as usize]);
        }
    }
    pub fn number_to_words(num: i32) -> String {
        let mut num = num;
        let mut out = String::new();
        let mut base: i32;
        for i in 0..3 {
            base = Self::GROUP_VALUES[i];
            if num >= base {
                Self::do_hundred(&mut out, num / base);
                out.push_str(Self::GROUP_TABLE[i]);
                num %= base;
                if num == 0 {
                    return out;
                }
                out.push(' ');
            }
        }
        Self::do_hundred(&mut out, num);
        out
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_to_words(1000), "One Thousand");
    assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
    assert_eq!(
        Solution::number_to_words(12345),
        "Twelve Thousand Three Hundred Forty Five"
    );
    assert_eq!(
        Solution::number_to_words(1234567),
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
    );
    assert_eq!(
        Solution::number_to_words(1234567891),
        "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
    );
}
