#![allow(dead_code)]

/*

// 2325. Decode the Message
// https://leetcode.com/problems/decode-the-message/
// https://leetcode.cn/problems/decode-the-message/
//
// Easy
//
// You are given the strings key and message, which represent a cipher key and a secret message,
// respectively. The steps to decode message are as follows:

    Use the first appearance of all 26 lowercase English letters in key as the order of the substitution table.
    Align the substitution table with the regular English alphabet.
    Each letter in message is then substituted using the table.
    Spaces ' ' are transformed to themselves.

    For example, given key = "happy boy" (actual key would have at least one instance of each letter in the alphabet), we have the partial substitution table of ('h' -> 'a', 'a' -> 'b', 'p' -> 'c', 'y' -> 'd', 'b' -> 'e', 'o' -> 'f').

Return the decoded message.

Example 1:

Input: key = "the quick brown fox jumps over the lazy dog", message = "vkbs bs t suepuv"
Output: "this is a secret"
Explanation: The diagram above shows the substitution table.
It is obtained by taking the first appearance of each letter in "the quick brown fox jumps over the lazy dog".

Example 2:

Input: key = "eljuxhpwnyrdgtqkviszcfmabo", message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb"
Output: "the five boxing wizards jump quickly"
Explanation: The diagram above shows the substitution table.
It is obtained by taking the first appearance of each letter in "eljuxhpwnyrdgtqkviszcfmabo".

Constraints:

    26 <= key.length <= 2000
    key consists of lowercase English letters and ' '.
    key contains every letter in the English alphabet ('a' to 'z') at least once.
    1 <= message.length <= 2000
    message consists of lowercase English letters and ' '.
*/

struct Solution;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let idx = |b: u8| (b - b'a') as usize;
        let mut map = [0; 26];
        let mut i = 0;
        for b in key.bytes().filter(|&b| b != b' ') {
            match b {
                _ if i == 26 => break,
                _ if map[idx(b)] > 0 => continue,
                _ => map[idx(b)] = b'a' + i + 1,
            }
            i += 1;
        }
        let res = message
            .bytes()
            .map(|b| match b {
                b' ' => b' ',
                _ => map[idx(b)] - 1,
            })
            .collect();
        String::from_utf8(res).unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            "the quick brown fox jumps over the lazy dog",
            "vkbs bs t suepuv",
            "this is a secret",
        ),
        (
            "eljuxhpwnyrdgtqkviszcfmabo",
            "zwx hnfx lqantp mnoeius ycgk vcnjrdb",
            "the five boxing wizards jump quickly",
        ),
    ];
    for (key, m, ex) in cases {
        assert_eq!(Solution::decode_message(key.to_string(), m.to_string()), ex);
    }
}
