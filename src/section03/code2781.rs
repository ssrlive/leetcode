#![allow(dead_code)]

// 2781. Length of the Longest Valid Substring
// https://leetcode.com/problems/length-of-the-longest-valid-substring/
// https://leetcode.cn/problems/length-of-the-longest-valid-substring/
//
// Hard
//
// You are given a string word and an array of strings forbidden.
//
// A string is called valid if none of its substrings are present in forbidden.
//
// Return the length of the longest valid substring of the string word.
//
// A substring is a contiguous sequence of characters in a string, possibly empty.
//
// Example 1:
//
// Input: word = "cbaaaabc", forbidden = ["aaa","cb"]
// Output: 4
// Explanation: There are 11 valid substrings in word: "c", "b", "a", "ba", "aa", "bc", "baa", "aab", "ab", "abc" and "aabc". The length of the longest valid substring is 4.
// It can be shown that all other substrings contain either "aaa" or "cb" as a substring.
//
// Example 2:
//
// Input: word = "leetcode", forbidden = ["de","le","e"]
// Output: 4
// Explanation: There are 11 valid substrings in word: "l", "t", "c", "o", "d", "tc", "co", "od", "tco", "cod", and "tcod". The length of the longest valid substring is 4.
// It can be shown that all other substrings contain either "de", "le", or "e" as a substring.
//
// Constraints:
//
//     1 <= word.length <= 10^5
//     word consists only of lowercase English letters.
//     1 <= forbidden.length <= 10^5
//     1 <= forbidden[i].length <= 10
//     forbidden[i] consists only of lowercase English letters.
//

struct Solution;

struct Trie {
    children: std::collections::HashMap<char, Trie>,
    leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: std::collections::HashMap::new(),
            leaf: false,
        }
    }
}

impl Solution {
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        let mut root = Trie::new();
        for f in forbidden {
            Self::add(&mut root, &f);
        }

        let word = word.chars().collect::<Vec<char>>();
        let mut data = vec![];
        for i in 0..word.len() {
            Self::collect(&mut root, &word, i, &mut data);
        }

        let mut mp = std::collections::BTreeMap::<usize, i32>::new();
        for item in &data {
            *mp.entry(item.1).or_insert(0) += 1;
        }

        let (mut ret, mut k) = (0, 0);
        for i in 0..word.len() {
            while k < data.len() && data[k].0 < i {
                if *mp.get(&data[k].1).unwrap() == 1 {
                    mp.remove(&data[k].1);
                } else {
                    *mp.entry(data[k].1).or_insert(0) -= 1;
                }
                k += 1;
            }
            let mut end = word.len();
            if let Some((val, _)) = mp.iter().next() {
                end = *val;
            }

            ret = ret.max(end as i32 - i as i32);
        }

        ret
    }

    fn add(root: &mut Trie, word: &str) {
        let mut p = root;

        for c in word.chars() {
            p = p.children.entry(c).or_insert(Trie::new());
        }

        p.leaf = true;
    }

    fn collect(root: &mut Trie, word: &[char], i: usize, v: &mut Vec<(usize, usize)>) {
        let mut p = root;

        for (j, item) in word.iter().enumerate().skip(i) {
            if !p.children.contains_key(item) {
                break;
            }
            p = p.children.entry(*item).or_insert(Trie::new());
            if !p.leaf {
                continue;
            }
            v.push((i, j));
            break;
        }
    }
}

#[test]
fn test() {
    let word = "cbaaaabc".to_string();
    let forbidden = vec!["aaa".to_string(), "cb".to_string()];
    let res = 4;
    assert_eq!(Solution::longest_valid_substring(word, forbidden), res);

    let word = "leetcode".to_string();
    let forbidden = vec!["de".to_string(), "le".to_string(), "e".to_string()];
    let res = 4;
    assert_eq!(Solution::longest_valid_substring(word, forbidden), res);
}
