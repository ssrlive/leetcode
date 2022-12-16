#![allow(dead_code)]

// 592. Fraction Addition and Subtraction
// https://leetcode.com/problems/fraction-addition-and-subtraction/
//
// Given a string expression representing an expression of fraction addition and subtraction,
// return the calculation result in string format.
//
// The final result should be an irreducible fraction. If your final result is an integer, change it
// to the format of a fraction that has a denominator 1. So in this case, 2 should be converted to 2/1.
//
// Example 1:
//
// Input: expression = "-1/2+1/2"
// Output: "0/1"
//
// Example 2:
//
// Input: expression = "-1/2+1/2+1/3"
// Output: "1/3"
//
// Example 3:
//
// Input: expression = "1/3-1/2"
// Output: "-1/6"
//
// Constraints:
//
// - The input string only contains '0' to '9', '/', '+' and '-'. So does the output.
// - Each fraction (input and output) has the format ±numerator/denominator. If the first input fraction or the output is positive, then '+' will be omitted.
// - The input only contains valid irreducible fractions, where the numerator and denominator of each fraction
//   will always be in the range [1, 10]. If the denominator is 1, it means this fraction is actually an integer
//   in a fraction format defined above.
// - The number of given fractions will be in the range [1, 10].
// - The numerator and denominator of the final result are guaranteed to be valid and in the range of 32-bit int.
//
#[derive(Debug, PartialEq, Eq)]
struct Fraction {
    sign: i32,
    num: i32,
    denom: i32,
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.sign * self.num, self.denom)
    }
}

impl Fraction {
    fn new(sign: i32, num: i32, denom: i32) -> Self {
        let gcd = Self::gcd(num, denom);
        Fraction {
            sign,
            num: num / gcd,
            denom: denom / gcd,
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn lcm(a: i32, b: i32) -> i32 {
        a * b / Self::gcd(a, b)
    }

    fn add(&self, other: &Self) -> Self {
        let lcm = Self::lcm(self.denom, other.denom);
        let num = self.sign * self.num * (lcm / self.denom) + other.sign * other.num * (lcm / other.denom);
        let sign = if num < 0 { -1 } else { 1 };
        Self::new(sign, num.abs(), lcm)
    }
}

struct Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        fn _fraction_addition(expression: String) -> Option<String> {
            let chars: Vec<char> = expression.chars().collect();
            let mut fractions = vec![];
            let mut cursor = 0;
            while cursor < chars.len() {
                if chars[cursor] == '/' {
                    let mut i = cursor;
                    while i > 0 && chars[i - 1].is_ascii_digit() {
                        i -= 1;
                    }
                    let num = chars[i..cursor].iter().collect::<String>().parse::<i32>().ok()?;
                    let mut j = cursor + 1;
                    while j + 1 < chars.len() && chars[j + 1].is_ascii_digit() {
                        j += 1;
                    }
                    let denom = chars[cursor + 1..=j].iter().collect::<String>().parse::<i32>().ok()?;
                    let sign = if i > 0 && chars[i - 1] == '-' { -1 } else { 1 };
                    let fraction = Fraction::new(sign, num, denom);
                    fractions.push(fraction);
                }
                cursor += 1;
            }
            let f = |acc: Fraction, x: &Fraction| acc.add(x);
            let v = fractions.iter().fold(Fraction::new(1, 0, 1), f).to_string();
            Some(v)
        }
        _fraction_addition(expression).unwrap_or_default()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::fraction_addition("-1/2+1/2".to_string()), "0/1".to_string());
    assert_eq!(
        Solution::fraction_addition("-1/2+1/2+1/3".to_string()),
        "1/3".to_string()
    );
    assert_eq!(Solution::fraction_addition("1/3-1/2".to_string()), "-1/6".to_string());
}
