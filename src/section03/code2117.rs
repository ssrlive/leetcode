#![allow(dead_code)]

/*

// 2117. Abbreviating the Product of a Range
// https://leetcode.com/problems/abbreviating-the-product-of-a-range/
// https://leetcode.cn/problems/abbreviating-the-product-of-a-range/
//
// Hard
//
// You are given two positive integers left and right with left <= right. Calculate the product of all integers in the inclusive range [left, right].

Since the product may be very large, you will abbreviate it following these steps:

    Count all trailing zeros in the product and remove them. Let us denote this count as C.
        For example, there are 3 trailing zeros in 1000, and there are 0 trailing zeros in 546.
    Denote the remaining number of digits in the product as d. If d > 10, then express the product as <pre>...<suf> where <pre> denotes the first 5 digits of the product, and <suf> denotes the last 5 digits of the product after removing all trailing zeros. If d <= 10, we keep it unchanged.
        For example, we express 1234567654321 as 12345...54321, but 1234567 is represented as 1234567.
    Finally, represent the product as a string "<pre>...<suf>eC".
        For example, 12345678987600000 will be represented as "12345...89876e5".

Return a string denoting the abbreviated product of all integers in the inclusive range [left, right].

Example 1:

Input: left = 1, right = 4
Output: "24e0"
Explanation: The product is 1 × 2 × 3 × 4 = 24.
There are no trailing zeros, so 24 remains the same. The abbreviation will end with "e0".
Since the number of digits is 2, which is less than 10, we do not have to abbreviate it further.
Thus, the final representation is "24e0".

Example 2:

Input: left = 2, right = 11
Output: "399168e2"
Explanation: The product is 39916800.
There are 2 trailing zeros, which we remove to get 399168. The abbreviation will end with "e2".
The number of digits after removing the trailing zeros is 6, so we do not abbreviate it further.
Hence, the abbreviated product is "399168e2".

Example 3:

Input: left = 371, right = 375
Output: "7219856259e3"
Explanation: The product is 7219856259000.

Constraints:

    1 <= left <= right <= 10^4
*/

struct Solution;

impl Solution {
    pub fn abbreviate_product(left: i32, right: i32) -> String {
        let (mut suff, mut c, mut total, max_suff) = (1, 0, 0, 100_000_000_000);
        let mut pref = 1.0;
        for i in left..=right {
            pref *= i as f64;
            suff *= i as i64;
            while pref >= 100_000.0 {
                pref /= 10.0;
                total = if total == 0 { 6 } else { total + 1 };
            }
            while suff % 10 == 0 {
                suff /= 10;
                c += 1;
            }
            suff %= max_suff;
        }
        let s = suff.to_string();
        format!(
            "{}{}{}e{}",
            pref as i32,
            if total - c <= 10 { "" } else { "..." },
            if total - c < 5 {
                ""
            } else {
                &s[s.len() - (total - c - 5).min(5) as usize..]
            },
            c
        )
    }
}

#[test]
fn test() {
    let cases = vec![
        (1, 4, "24e0".to_string()),
        (2, 11, "399168e2".to_string()),
        (371, 375, "7219856259e3".to_string()),
    ];
    for (left, right, expected) in cases {
        assert_eq!(Solution::abbreviate_product(left, right), expected);
    }
}
