#![allow(dead_code)]

// 402. Remove K Digits
// https://leetcode.com/problems/remove-k-digits/
// https://leetcode.cn/problems/remove-k-digits/
//
// Given string num representing a non-negative integer num, and an integer k,
// return the smallest possible integer after removing k digits from num.
//
// Example 1:
//
// Input: num = "1432219", k = 3
// Output: "1219"
// Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219 which is the smallest.
//
// Example 2:
//
// Input: num = "10200", k = 1
// Output: "200"
// Explanation: Remove the leading 1 and the number is 200. Note that the output must not contain leading zeroes.
//
// Example 3:
//
// Input: num = "10", k = 2
// Output: "0"
// Explanation: Remove all the digits from the number and it is left with nothing which is 0.
//
// Constraints:
//
// - 1 <= k <= num.length <= 10^5
// - num consists of only digits.
// - num does not have any leading zeros except for the zero itself.
//

struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut result = Vec::new();
        let mut k = k;
        for c in num.chars() {
            while k > 0 && !result.is_empty() && result.last().unwrap() > &c {
                result.pop();
                k -= 1;
            }
            result.push(c);
        }
        while k > 0 {
            result.pop();
            k -= 1;
        }
        let mut result = result.into_iter().collect::<String>();
        while result.starts_with('0') {
            result.remove(0);
        }
        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_kdigits("1432219".to_string(), 3), "1219".to_string());
    assert_eq!(Solution::remove_kdigits("10200".to_string(), 1), "200".to_string());
    assert_eq!(Solution::remove_kdigits("10".to_string(), 2), "0".to_string());
    assert_eq!(Solution::remove_kdigits("123".to_string(), 1), "12".to_string());
    assert_eq!(Solution::remove_kdigits("123".to_string(), 2), "1".to_string());
    assert_eq!(Solution::remove_kdigits("123".to_string(), 18), "0".to_string());
    assert_eq!(Solution::remove_kdigits("123".to_string(), 19), "0".to_string());
    assert_eq!(Solution::remove_kdigits("123".to_string(), 20), "0".to_string());
}
