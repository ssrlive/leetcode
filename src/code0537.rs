#![allow(dead_code)]

// 537. Complex Number Multiplication
// https://leetcode.com/problems/complex-number-multiplication/
// https://leetcode.cn/problems/complex-number-multiplication/
//
// A complex number can be represented as a string on the form "real+imaginaryi" where:
//
// - real is the real part and is an integer in the range [-100, 100].
// - imaginary is the imaginary part and is an integer in the range [-100, 100].
// - i^2 == -1.
//
// Given two complex numbers num1 and num2 as strings, return a string of the complex number that represents their multiplications.
//
// Example 1:
//
// Input: num1 = "1+1i", num2 = "1+1i"
// Output: "0+2i"
// Explanation: (1 + i) * (1 + i) = 1 + i2 + 2 * i = 2i, and you need convert it to the form of 0+2i.
//
// Example 2:
//
// Input: num1 = "1+-1i", num2 = "1+-1i"
// Output: "0+-2i"
// Explanation: (1 - i) * (1 - i) = 1 + i2 - 2 * i = -2i, and you need convert it to the form of 0+-2i.
//
// Constraints:
//
// num1 and num2 are valid complex numbers.
//
// Follow up: Can you solve the problem without using the built-in complex data type?

#[derive(Debug, PartialEq, Default, Clone, Copy)]
struct Complex {
    real: i32,
    imaginary: i32,
}

impl Complex {
    fn new(real: i32, imaginary: i32) -> Self {
        Self { real, imaginary }
    }

    pub fn from_str(s: &str) -> Self {
        fn _from_str(s: &str) -> Option<Complex> {
            let mut parts = s.split('+');
            let real = parts.next()?.parse().ok()?;
            let imaginary = parts.next()?.trim_end_matches('i').parse().ok()?;
            Some(Complex { real, imaginary })
        }
        _from_str(s).unwrap_or_default()
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.real, self.imaginary)
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let real = self.real * rhs.real - self.imaginary * rhs.imaginary;
        let imaginary = self.real * rhs.imaginary + self.imaginary * rhs.real;
        Self { real, imaginary }
    }
}

struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let c1 = Complex::from_str(&num1);
        let c2 = Complex::from_str(&num2);
        let c3 = c1 * c2;
        c3.to_string()
    }
}

#[test]
fn test() {
    let num1 = "1+1i".to_string();
    let num2 = "1+1i".to_string();
    let expected = "0+2i".to_string();
    assert_eq!(Solution::complex_number_multiply(num1, num2), expected);

    let num1 = "1+-1i".to_string();
    let num2 = "1+-1i".to_string();
    let expected = "0+-2i".to_string();
    assert_eq!(Solution::complex_number_multiply(num1, num2), expected);
}
