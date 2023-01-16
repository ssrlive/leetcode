#![allow(dead_code)]

// 1147. Longest Chunked Palindrome Decomposition
// https://leetcode.com/problems/longest-chunked-palindrome-decomposition/
// https://leetcode.cn/problems/longest-chunked-palindrome-decomposition/
//
// You are given a string text. You should split it to k substrings (subtext1, subtext2, ..., subtextk) such that:
//
// subtexti is a non-empty string.
// The concatenation of all the substrings is equal to text (i.e., subtext1 + subtext2 + ... + subtextk == text).
// subtexti == subtextk - i + 1 for all valid values of i (i.e., 1 <= i <= k).
// Return the largest possible value of k.
//
// Example 1:
//
// Input: text = "ghiabcdefhelloadamhelloabcdefghi"
// Output: 7
// Explanation: We can split the string on "(ghi)(abcdef)(hello)(adam)(hello)(abcdef)(ghi)".
//
// Example 2:
//
// Input: text = "merchant"
// Output: 1
// Explanation: We can split the string on "(merchant)".
//
// Example 3:
//
// Input: text = "antaprezatepzapreanta"
// Output: 11
// Explanation: We can split the string on "(a)(nt)(a)(pre)(za)(tep)(za)(pre)(a)(nt)(a)".
//
// Constraints:
//
// - 1 <= text.length <= 1000
// - text consists only of lowercase English characters.
//

struct Solution;

impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let mut singlecentre = true;
        let mut tokencount = 0;
        let mut prevtokenend = 0;
        let tl = text.len();
        let maxi = (tl) / 2 + 1;
        for i in 1..maxi {
            if i <= prevtokenend {
                continue;
            }
            if text[prevtokenend..i] == text[(tl - i)..(tl - prevtokenend)] {
                tokencount += 1;
                if i >= maxi - 1 && tl % 2 != 1 {
                    singlecentre = false;
                    if prevtokenend > i - 1 {
                        tokencount -= 1;
                    }
                }
                prevtokenend = i;
            }
        }
        tokencount * 2 + i32::from(singlecentre)
    }
}

#[test]
fn test() {
    let cases = vec![
        ("ghiabcdefhelloadamhelloabcdefghi", 7),
        ("merchant", 1),
        ("antaprezatepzapreanta", 11),
        ("aaa", 3),
        ("aaaa", 4),
    ];
    for (text, expected) in cases {
        assert_eq!(Solution::longest_decomposition(text.to_string()), expected);
    }
}
