#![allow(dead_code)]

// 483. Smallest Good Base
// https://leetcode.com/problems/smallest-good-base/
//
// Given an integer n represented as a string, return the smallest good base of n.
//
// We call k >= 2 a good base of n, if all digits of n base k are 1's.
//
// Example 1:
//
// Input: n = "13"
// Output: "3"
// Explanation: 13 base 3 is 111.
//
// Example 2:
//
// Input: n = "4681"
// Output: "8"
// Explanation: 4681 base 8 is 11111.
//
// Example 3:
//
// Input: n = "1000000000000000000"
// Output: "999999999999999999"
// Explanation: 1000000000000000000 base 999999999999999999 is 11.
//
// Constraints:
//
// - n is an integer in the range [3, 1018].
// - n does not contain any leading zeros.
//

struct Solution;

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        fn _smallest_good_base(n: String) -> Option<String> {
            let n = n.parse::<u64>().ok()?;
            let mut m_max = (n as f64).log(2.0).floor() as u32;
            while m_max > 1 {
                let k = (n as f64).powf(1.0 / m_max as f64).floor() as u64;
                if k > 1 {
                    let mut sum = 1;
                    let mut mul = 1;
                    for _ in 0..m_max {
                        mul *= k;
                        sum += mul;
                    }
                    if sum == n {
                        return Some(k.to_string());
                    }
                }
                m_max -= 1;
            }
            Some((n - 1).to_string())
        }

        _smallest_good_base(n).unwrap_or_default()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_good_base("13".to_string()), "3".to_string());
    assert_eq!(Solution::smallest_good_base("4681".to_string()), "8".to_string());
    let n = "1000000000000000000".to_string();
    let r = "999999999999999999".to_string();
    assert_eq!(Solution::smallest_good_base(n), r);
}
