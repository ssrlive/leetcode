#![allow(dead_code)]

// 2953. Count Complete Substrings
// https://leetcode.com/problems/count-complete-substrings/
// https://leetcode.cn/problems/count-complete-substrings/
//
// Medium
//
// You are given a string word and an integer k.
//
// A substring s of word is complete if:
//
//    - Each character in s occurs exactly k times.
//    - The difference between two adjacent characters is at most 2.
//      That is, for any two adjacent characters c1 and c2 in s,
//      the absolute difference in their positions in the alphabet is at most 2.
//
// Return the number of complete substrings of word.
//
// A substring is a non-empty contiguous sequence of characters in a string.
//
// Example 1:
//
// Input: word = "igigee", k = 2
// Output: 3
// Explanation: The complete substrings where each character appears exactly twice and the
// difference between adjacent characters is at most 2 are: igigee, igigee, igigee.
//
// Example 2:
//
// Input: word = "aaabbbccc", k = 3
// Output: 6
// Explanation: The complete substrings where each character appears exactly three times and
// the difference between adjacent characters is at most 2 are:
// aaabbbccc, aaabbbccc, aaabbbccc, aaabbbccc, aaabbbccc, aaabbbccc.
//
// Constraints:
//
//     1 <= word.length <= 10^5
//     word consists only of lowercase English letters.
//     1 <= k <= word.length
//

struct Solution;

impl Solution {
    pub fn count_complete_substrings(word: String, k: i32) -> i32 {
        let mut vec = vec![];
        let mut v = vec![];
        let s = word.as_bytes();
        for i in 1..s.len() {
            if i < s.len() && (s[i] as i32 - s[i - 1] as i32).abs() > 2 {
                v.push(i);
            }
        }
        let mut st = 0;
        let e = s.len() - 1;
        for it in v {
            if st < it {
                vec.push(vec![st, it - 1]);
            }
            st = it;
        }
        if st <= e {
            // vec will store ranges in which substring can be formed
            vec.push(vec![st, e]);
        }
        let mut ans = 0;
        for i in 1..=26 {
            if i * k <= s.len() as i32 {
                // func will return number of substrings with i different characters each occuring exactly k times
                ans += Solution::func(i, k, s, &vec);
            } else {
                break;
            }
        }
        ans
    }

    // Standard sliding window approach counting substrings with length x*k with each character occuring exactly k times
    fn func(x: i32, k: i32, s: &[u8], vec: &Vec<Vec<usize>>) -> i32 {
        let mut ans = 0;
        for it in vec {
            let mut j = it[0];
            let e = it[1];
            let mut cntk = 0;
            let mut mp = std::collections::HashMap::new();
            let mut i = it[0];
            while j <= e {
                mp.insert(s[j], mp.get(&s[j]).unwrap_or(&0) + 1);
                if mp.get(&s[j]) == Some(&k) {
                    cntk += 1;
                }
                if j - i + 1 > (x * k) as usize {
                    mp.insert(s[i], mp.get(&s[i]).unwrap() - 1);
                    if mp.get(&s[i]) == Some(&(k - 1)) {
                        cntk -= 1;
                    }
                    if mp.get(&s[i]) == Some(&0) {
                        mp.remove(&s[i]);
                    }
                    i += 1;
                }
                if mp.len() == x as usize && cntk == mp.len() {
                    ans += 1;
                }
                j += 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_complete_substrings("igigee".to_string(), 2), 3);
    assert_eq!(Solution::count_complete_substrings("aaabbbccc".to_string(), 3), 6);
}
