#![allow(dead_code)]

// 126. Word Ladder II
// https://leetcode.com/problems/word-ladder-ii/
//
// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
// - Every adjacent pair of words differs by a single letter.
// - Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
// - sk == endWord
//
// Given two words, beginWord and endWord, and a dictionary wordList, return all the shortest transformation sequences from beginWord to endWord,
// or an empty list if no such sequence exists. Each sequence should be returned as a list of the words [beginWord, s1, s2, ..., sk].

struct Solution;

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        fn backtrack(
            route: &mut Vec<usize>,
            ans: &mut Vec<Vec<String>>,
            helper: &Vec<(Vec<usize>, i32)>,
            word_list: &Vec<String>,
        ) {
            let curr = *route.last().unwrap();
            if curr == word_list.len() - 1 {
                let ans2 = route.iter().rev().map(|&i| word_list[i].to_owned()).collect();
                ans.push(ans2);
            } else {
                for &p in helper[curr].0.iter() {
                    route.push(p);
                    backtrack(route, ans, helper, word_list);
                    route.pop();
                }
            }
        }

        fn is_adj(word1: &str, word2: &str) -> bool {
            word1.chars().zip(word2.chars()).filter(|(ch1, ch2)| ch1 != ch2).count() == 1
        }

        let mut word_list = word_list;
        let end = match word_list.iter().position(|s| s == &end_word) {
            None => return Vec::new(),
            Some(i) => i,
        };
        word_list.push(begin_word);
        // (list of previous words, depth of BFS)
        let mut helper: Vec<(Vec<usize>, i32)> = vec![(Vec::new(), -1); word_list.len()];
        helper[word_list.len() - 1].1 = 0;
        for depth in 0.. {
            let mut no_route = true;
            for i in 0..word_list.len() {
                if helper[i].1 == depth {
                    for j in 0..word_list.len() - 1 {
                        if (helper[j].1 == -1 || helper[j].1 == depth + 1) && is_adj(&word_list[i], &word_list[j]) {
                            no_route = false;
                            helper[j].0.push(i);
                            helper[j].1 = depth + 1;
                        }
                    }
                }
            }
            if no_route {
                return Vec::new();
            }
            if helper[end].1 != -1 {
                break;
            }
        }
        let mut ans = Vec::new();
        backtrack(&mut vec![end], &mut ans, &helper, &word_list);
        ans
    }
}

#[test]
fn test_find_ladders() {
    let begin_word = "hit".to_owned();
    let end_word = "cog".to_owned();
    let word_list = vec![
        "hot".to_owned(),
        "dot".to_owned(),
        "dog".to_owned(),
        "lot".to_owned(),
        "log".to_owned(),
        "cog".to_owned(),
    ];
    let ans = vec![
        vec![
            "hit".to_owned(),
            "hot".to_owned(),
            "dot".to_owned(),
            "dog".to_owned(),
            "cog".to_owned(),
        ],
        vec![
            "hit".to_owned(),
            "hot".to_owned(),
            "lot".to_owned(),
            "log".to_owned(),
            "cog".to_owned(),
        ],
    ];
    assert_eq!(Solution::find_ladders(begin_word, end_word, word_list), ans);
}
