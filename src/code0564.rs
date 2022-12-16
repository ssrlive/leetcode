#![allow(dead_code)]

// 564. Find the Closest Palindrome
// https://leetcode.com/problems/find-the-closest-palindrome/
//
// Given a string n representing an integer, return the closest integer (not including itself), which is a palindrome. If there is a tie, return the smaller one.
//
// The closest is defined as the absolute difference minimized between two integers.
//
// Example 1:
//
// Input: n = "123"
// Output: "121"
//
// Example 2:
//
// Input: n = "1"
// Output: "0"
// Explanation: 0 and 2 are the closest palindromes but we return the smallest which is 0.
//
// Constraints:
//
// - 1 <= n.length <= 18
// - n consists of only digits.
// - n does not have leading zeros.
// - n is representing an integer in the range [1, 10^18 - 1].
//

struct Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        Solution::_nearest_palindromic(n).unwrap_or_default()
    }

    fn _nearest_palindromic(n: String) -> Option<String> {
        let mut arr = n.chars().collect::<Vec<char>>();
        let len = arr.len();
        for i in 0..len / 2 {
            arr[len - 1 - i] = arr[i];
        }
        let cur_p = arr.iter().collect::<String>();
        let pre_p = Solution::nearest_palindrom(&cur_p, false)?;
        let next_p = Solution::nearest_palindrom(&cur_p, true)?;
        let num = n.parse::<i64>().ok()?;
        let cur = cur_p.parse::<i64>().ok()?;
        let pre = pre_p.parse::<i64>().ok()?;
        let next = next_p.parse::<i64>().ok()?;
        let d1 = (num - pre).abs();
        let d2 = (num - cur).abs();
        let d3 = (num - next).abs();
        if num == cur {
            if d1 <= d3 {
                Some(pre_p)
            } else {
                Some(next_p)
            }
        } else if num > cur {
            if d2 <= d3 {
                Some(cur_p)
            } else {
                Some(next_p)
            }
        } else if d1 <= d2 {
            Some(pre_p)
        } else {
            Some(cur_p)
        }
    }

    fn nearest_palindrom(cur_p: &str, dir: bool) -> Option<String> {
        let k = cur_p.len() / 2;
        let p = cur_p.len() - k;
        let mut l = cur_p[0..p].parse::<i64>().ok()?;
        if dir {
            l += 1;
        } else {
            l -= 1;
        }
        if l == 0 {
            if k == 0 {
                return Some("0".to_string());
            } else {
                return Some("9".to_string());
            }
        }
        let mut left = l.to_string();
        let mut right = left.clone();
        right = right.chars().rev().collect::<String>();
        if k > left.len() {
            right.push('9');
        }
        left.push_str(&right[right.len() - k..]);
        Some(left)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::nearest_palindromic("123".to_string()), "121".to_string());
    assert_eq!(Solution::nearest_palindromic("1".to_string()), "0".to_string());
    assert_eq!(Solution::nearest_palindromic("10".to_string()), "9".to_string());
    assert_eq!(Solution::nearest_palindromic("14".to_string()), "11".to_string());
    assert_eq!(Solution::nearest_palindromic("15".to_string()), "11".to_string());
    assert_eq!(Solution::nearest_palindromic("25".to_string()), "22".to_string());
    assert_eq!(Solution::nearest_palindromic("26".to_string()), "22".to_string());
    assert_eq!(Solution::nearest_palindromic("27".to_string()), "22".to_string());
    assert_eq!(Solution::nearest_palindromic("28".to_string()), "33".to_string());
    assert_eq!(Solution::nearest_palindromic("29".to_string()), "33".to_string());
    assert_eq!(Solution::nearest_palindromic("30".to_string()), "33".to_string());
    assert_eq!(Solution::nearest_palindromic("34".to_string()), "33".to_string());
    assert_eq!(Solution::nearest_palindromic("35".to_string()), "33".to_string());
}
