#![allow(dead_code)]

// 166. Fraction to Recurring Decimal
// https://leetcode.com/problems/fraction-to-recurring-decimal/
//
// Given two integers representing the numerator and denominator of a fraction, return the fraction in string format.
//
// If the fractional part is repeating, enclose the repeating part in parentheses.
//
// If multiple answers are possible, return any of them.
//
// It is guaranteed that the length of the answer string is less than 104 for all the given inputs.
//

struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut n = numerator as i64;
        let mut d = denominator as i64;
        let mut res = String::new();
        if n * d < 0 {
            res.push('-');
        }
        n = n.abs();
        d = d.abs();
        res.push_str(&(n / d).to_string());
        n %= d;
        if n == 0 {
            return res;
        }
        res.push('.');
        let mut map = std::collections::HashMap::new();
        while n != 0 {
            if map.contains_key(&n) {
                res.insert(map[&n], '(');
                res.push(')');
                break;
            }
            map.insert(n, res.len());
            n *= 10;
            res.push_str(&(n / d).to_string());
            n %= d;
        }
        res
    }
}

#[test]
fn test_fraction_to_decimal() {
    assert_eq!(
        Solution::fraction_to_decimal(1, 2),
        "0.5".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(2, 1),
        "2".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(2, 3),
        "0.(6)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(4, 333),
        "0.(012)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 5),
        "0.2".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 6),
        "0.1(6)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 7),
        "0.(142857)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 17),
        "0.(0588235294117647)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 90),
        "0.0(1)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 99),
        "0.(01)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 999),
        "0.(001)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 9999),
        "0.(0001)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 99999),
        "0.(00001)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 999999),
        "0.(000001)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 9999999),
        "0.(0000001)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 99999999),
        "0.(00000001)".to_string()
    );
    assert_eq!(
        Solution::fraction_to_decimal(1, 999999999),
        "0.(000000001)".to_string()
    );
}
