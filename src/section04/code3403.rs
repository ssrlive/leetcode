#![allow(dead_code)]

// 3403. Find the Lexicographically Largest String From the Box I
// https://leetcode.com/problems/find-the-lexicographically-largest-string-from-the-box-i/
// https://leetcode.cn/problems/find-the-lexicographically-largest-string-from-the-box-i/
//
// Medium
//
// You are given a string word, and an integer numFriends.
//
// Alice is organizing a game for her numFriends friends. There are multiple rounds in the game, where in each round:
//
// word is split into numFriends non-empty strings, such that no previous round has had the exact same split.
// All the split words are put into a box.
// Find the lexicographically largest string from the box after all the rounds are finished.
//
// A string a is lexicographically smaller than a string b if in the first position where a and b differ,
// string a has a letter that appears earlier in the alphabet than the corresponding letter in b.
// If the first min(a.length, b.length) characters do not differ, then the shorter string is the lexicographically smaller one.
//
// Example 1:
//
// Input: word = "dbca", numFriends = 2
//
// Output: "dbc"
//
// Explanation:
//
// All possible splits are:
//
// "d" and "bca".
// "db" and "ca".
// "dbc" and "a".
//
// Example 2:
//
// Input: word = "gggg", numFriends = 4
//
// Output: "g"
//
// Explanation:
//
// The only possible split is: "g", "g", "g", and "g".
//
// Constraints:
//
// 1 <= word.length <= 5 * 10^3
// word consists only of lowercase English letters.
// 1 <= numFriends <= word.length
//

struct Solution;

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let word = word.as_bytes();
        let n = word.len();
        let maxlen = n - num_friends as usize + 1;
        let mut maxchar = word[0];
        let mut res = vec![];
        for i in 0..n {
            if word[i] >= maxchar {
                let curr = &word[i..(i + maxlen).min(n)];
                if curr > &res {
                    res = curr.to_vec();
                }
                maxchar = word[i];
            }
        }
        String::from_utf8(res).unwrap()
    }
}

#[test]
fn test() {
    let word = "dbca".to_string();
    let num_friends = 2;
    let output = "dbc".to_string();
    assert_eq!(Solution::answer_string(word, num_friends), output);

    let word = "gggg".to_string();
    let num_friends = 4;
    let output = "g".to_string();
    assert_eq!(Solution::answer_string(word, num_friends), output);

    let word = "aann".to_string();
    let num_friends = 2;
    let output = "nn".to_string();
    assert_eq!(Solution::answer_string(word, num_friends), output);
}
