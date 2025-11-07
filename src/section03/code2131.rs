#![allow(dead_code)]

/*

// 2131. Longest Palindrome by Concatenating Two Letter Words
// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/
// https://leetcode.cn/problems/longest-palindrome-by-concatenating-two-letter-words/
//
// Medium
//
// You are given an array of strings words. Each element of words consists of two lowercase English letters.

Create the longest possible palindrome by selecting some elements from words and concatenating them in any order. Each element can be selected at most once.

Return the length of the longest palindrome that you can create. If it is impossible to create any palindrome, return 0.

A palindrome is a string that reads the same forward and backward.

Example 1:

Input: words = ["lc","cl","gg"]
Output: 6
Explanation: One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of length 6.
Note that "clgglc" is another longest palindrome that can be created.

Example 2:

Input: words = ["ab","ty","yt","lc","cl","ab"]
Output: 8
Explanation: One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt", of length 8.
Note that "lcyttycl" is another longest palindrome that can be created.

Example 3:

Input: words = ["cc","ll","xx"]
Output: 2
Explanation: One longest palindrome is "cc", of length 2.
Note that "ll" is another longest palindrome that can be created, and so is "xx".

Constraints:

    1 <= words.length <= 10^5
    words[i].length == 2
    words[i] consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut occ = vec![vec![0; 26]; 26];
        for word in words {
            let cc: Vec<usize> = word.chars().map(|e| (e as usize) - ('a' as usize)).collect();
            occ[cc[0]][cc[1]] += 1;
        }
        let mut result = 0;
        let mut have_middle = false;
        for (i, occ_i) in occ.iter().enumerate() {
            for (j, &occ_ij) in occ_i.iter().enumerate().skip(i) {
                if i == j {
                    let aa = occ_ij;
                    result += (aa / 2) * 4;
                    if aa % 2 == 1 && !have_middle {
                        result += 2;
                        have_middle = true;
                    }
                } else {
                    let ab = occ[i][j];
                    let ba = occ[j][i];
                    let numpair = std::cmp::min(ab, ba);
                    result += numpair * 4;
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["lc", "cl", "gg"], 6),
        (vec!["ab", "ty", "yt", "lc", "cl", "ab"], 8),
        (vec!["cc", "ll", "xx"], 2),
    ];
    for (words, expected) in cases {
        let words = words.into_iter().map(|w| w.to_string()).collect();
        assert_eq!(Solution::longest_palindrome(words), expected);
    }
}
