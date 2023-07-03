#![allow(dead_code)]

// 269. Alien-Dictionary
// https://leetcode.com/problems/alien-dictionary/
// https://leetcode.cn/problems/alien-dictionary/
//
// Hard
//
// There is a new alien language which uses the latin alphabet. However, the order among letters are unknown to you.
// You receive a list of non-empty words from the dictionary, where words are sorted lexicographically by the rules of this new language.
// Derive the order of letters in this language.
//​
// Example 1:
// Input:
// [
//   "wrt",
//   "wrf",
//   "er",
//   "ett",
//   "rftt"
// ]
// Output: "wertf"
//
// Example 2:
// Input:
// [
//   "z",
//   "x"
// ]
// Output: "zx"
//
// Example 3:
// Input:
// [
//   "z",
//   "x",
//   "z"
// ]
// Output: ""
// Explanation: The order is invalid, so return "".
//
// Note:
//
// - You may assume all letters are in lowercase.
// - You may assume that if a is a prefix of b, then a must appear before b in the given dictionary.
// - If the order is invalid, return an empty string.
// - There may be multiple valid order of letters, return any one of them is fine.
//

struct Solution;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        use std::collections::{hash_map::Entry, HashMap};
        fn build_graph(words: &Vec<String>, map: &mut HashMap<char, Vec<char>>, visited: &mut HashMap<char, i32>) {
            for word in words {
                for c in word.chars() {
                    if let Entry::Vacant(e) = map.entry(c) {
                        e.insert(Vec::new());
                        visited.insert(c, 0);
                    }
                }
            }

            for i in 0..words.len() - 1 {
                let mut index = 0;
                while index < words[i].len() && index < words[i + 1].len() {
                    let c1 = words[i].chars().nth(index).unwrap();
                    let c2 = words[i + 1].chars().nth(index).unwrap();
                    if c1 != c2 {
                        map.get_mut(&c1).unwrap().push(c2);
                        break;
                    }
                    index += 1;
                }
            }
        }

        fn dfs(c: &char, map: &mut HashMap<char, Vec<char>>, visited: &mut HashMap<char, i32>, sb: &mut String) -> bool {
            if *visited.get(c).unwrap() == 1 {
                return true;
            }
            if *visited.get(c).unwrap() == -1 {
                return false;
            }

            visited.insert(*c, -1);
            let neighbors = map.get(c).unwrap().clone();
            for neightbor in neighbors.iter() {
                if !dfs(neightbor, map, visited, sb) {
                    return false;
                }
            }

            visited.insert(*c, 1);
            sb.insert(0, *c);
            true
        }

        let mut map = HashMap::new();
        let mut visited = HashMap::new();
        let mut sb = String::new();

        build_graph(&words, &mut map, &mut visited);

        let keys = map.keys().cloned().collect::<Vec<char>>();
        for c in keys.iter() {
            if !dfs(c, &mut map, &mut visited, &mut sb) {
                return "".to_string();
            }
        }

        sb
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["wrt", "wrf", "er", "ett", "rftt"], "wertf"),
        (vec!["z", "x"], "zx"),
        (vec!["z", "x", "z"], ""),
    ];
    for (words, expected) in cases {
        let words = words.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::alien_order(words), expected);
    }
}
