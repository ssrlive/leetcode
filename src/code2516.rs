#![allow(dead_code)]

// 2516. Take K of Each Character From Left and Right
// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/
// https://leetcode.cn/problems/take-k-of-each-character-from-left-and-right/
//
// You are given a string s consisting of the characters 'a', 'b', and 'c' and a non-negative integer k. Each minute, you may take either the leftmost character of s, or the rightmost character of s.
//
// Return the minimum number of minutes needed for you to take at least k of each character, or return -1 if it is not possible to take k of each character.
//
// Example 1:
//
// Input: s = "aabaaaacaabc", k = 2
// Output: 8
// Explanation:
// Take three characters from the left of s. You now have two 'a' characters, and one 'b' character.
// Take five characters from the right of s. You now have four 'a' characters, two 'b' characters, and two 'c' characters.
// A total of 3 + 5 = 8 minutes is needed.
// It can be proven that 8 is the minimum number of minutes needed.
//
// Example 2:
//
// Input: s = "a", k = 1
// Output: -1
// Explanation: It is not possible to take one 'b' or 'c' so return -1.
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of only the letters 'a', 'b', and 'c'.
// - 0 <= k <= s.length
//

struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }

        let (mut i, n) = (0, s.len());
        let s = s.chars().collect::<Vec<char>>();
        let mut count = vec![0; 3];

        while i < n && (count[0] < k || count[1] < k || count[2] < k) {
            count[(s[i] as u8 - b'a') as usize] += 1;
            i += 1;
        }

        if *count.iter().min().unwrap() < k {
            return -1;
        }
        let mut ret = i;

        let mut j = n;
        loop {
            count[(s[i - 1] as u8 - b'a') as usize] -= 1;

            while count[0] < k || count[1] < k || count[2] < k {
                j -= 1;
                count[(s[j] as u8 - b'a') as usize] += 1;
            }

            ret = ret.min(i - 1 + n - j);
            println!["ret: {}", ret];
            if i == 1 {
                break;
            }
            i -= 1;
        }

        ret as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::take_characters("aabaaaacaabc".to_string(), 2), 8);
    assert_eq!(Solution::take_characters("a".to_string(), 1), -1);
    assert_eq!(Solution::take_characters("abc".to_string(), 1), 3);
    assert_eq!(Solution::take_characters("abc".to_string(), 2), -1);
    assert_eq!(Solution::take_characters("abc".to_string(), 3), -1);
    assert_eq!(Solution::take_characters("abc".to_string(), 4), -1);
}
