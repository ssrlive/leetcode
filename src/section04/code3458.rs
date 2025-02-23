#![allow(dead_code)]

// 3458. Select K Disjoint Special Substrings
// https://leetcode.com/problems/select-k-disjoint-special-substrings/
// https://leetcode.cn/problems/select-k-disjoint-special-substrings/
//
// Medium
//
// Given a string s of length n and an integer k, determine whether it is possible to select k disjoint special substrings.
//
// A special substring is a substring where:
//
//     Any character present inside the substring should not appear outside it in the string.
//     The substring is not the entire string s.
//
// Note that all k substrings must be disjoint, meaning they cannot overlap.
//
// Return true if it is possible to select k such disjoint special substrings; otherwise, return false.
//
// Example 1:
//
// Input: s = "abcdbaefab", k = 2
//
// Output: true
//
// Explanation:
//
//     We can select two disjoint special substrings: "cd" and "ef".
//     "cd" contains the characters 'c' and 'd', which do not appear elsewhere in s.
//     "ef" contains the characters 'e' and 'f', which do not appear elsewhere in s.
//
// Example 2:
//
// Input: s = "cdefdc", k = 3
//
// Output: false
//
// Explanation:
//
// There can be at most 2 disjoint special substrings: "e" and "f". Since k = 3, the output is false.
//
// Example 3:
//
// Input: s = "abeabe", k = 0
//
// Output: true
//
// Constraints:
//
//     2 <= n == s.length <= 5 * 10^4
//     0 <= k <= 26
//     s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn max_substring_length(s: String, k: i32) -> bool {
        use std::collections::{HashMap, HashSet};
        if k == 0 {
            return true;
        }

        let n = s.len();
        let mut first: HashMap<char, usize> = HashMap::new();
        let mut last: HashMap<char, usize> = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();

        for (i, &c) in s_chars.iter().enumerate() {
            first.entry(c).or_insert(i);
            last.insert(c, i);
        }

        let mut intervals = Vec::new();

        for start in 0..n {
            if start > *first.get(&s_chars[start]).unwrap() {
                continue;
            }

            let mut chars_in_interval = HashSet::new();
            let mut max_last = start;

            for (end, &curr_char) in s_chars.iter().enumerate().take(n).skip(start) {
                if *first.get(&curr_char).unwrap() < start || *last.get(&curr_char).unwrap() > max_last {
                    if *first.get(&curr_char).unwrap() >= start {
                        max_last = std::cmp::max(max_last, *last.get(&curr_char).unwrap());
                    } else {
                        break;
                    }
                }

                chars_in_interval.insert(curr_char);

                if end == max_last && (start > 0 || end < n - 1) {
                    intervals.push((start, end));
                }
            }
        }

        intervals.sort_unstable_by_key(|&(_, end)| end);

        let mut count = 0;
        let mut last_end: Option<usize> = None;

        for (start, end) in intervals {
            if last_end.is_none_or(|last| start > last) {
                count += 1;
                last_end = Some(end);
                if count >= k {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
fn test() {
    let s = "abcdbaefab".to_string();
    let k = 2;
    let res = true;
    assert_eq!(Solution::max_substring_length(s, k), res);

    let s = "cdefdc".to_string();
    let k = 3;
    let res = false;
    assert_eq!(Solution::max_substring_length(s, k), res);

    let s = "abeabe".to_string();
    let k = 0;
    let res = true;
    assert_eq!(Solution::max_substring_length(s, k), res);
}
