#![allow(dead_code)]

// 3435. Frequencies of Shortest Supersequences
// https://leetcode.com/problems/frequencies-of-shortest-supersequences/
// https://leetcode.cn/problems/frequencies-of-shortest-supersequences/
//
// Hard
//
// You are given an array of strings words. Find all shortest common supersequences (SCS) of words that are not permutations of each other.
//
// A shortest common supersequence is a string of minimum length that contains each string in words as a subsequence.
//
// Return a 2D array of integers freqs that represent all the SCSs. Each freqs[i] is an array of size 26,
// representing the frequency of each letter in the lowercase English alphabet for a single SCS.
// You may return the frequency arrays in any order.
//
// Example 1:
//
// Input: words = ["ab","ba"]
//
// Output: [[1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]
//
// Explanation:
//
// The two SCSs are "aba" and "bab". The output is the letter frequencies for each one.
//
// Example 2:
//
// Input: words = ["aa","ac"]
//
// Output: [[2,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]
//
// Explanation:
//
// The two SCSs are "aac" and "aca". Since they are permutations of each other, keep only "aac".
//
// Example 3:
//
// Input: words = ["aa","bb","cc"]
//
// Output: [[2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]
//
// Explanation:
//
// "aabbcc" and all its permutations are SCSs.
//
// Constraints:
//
//     1 <= words.length <= 256
//     words[i].length == 2
//     All strings in words will altogether be composed of no more than 16 unique lowercase letters.
//     All strings in words are unique.
//

struct Solution;

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::{HashMap, HashSet};
        const LETTERS: usize = 16;
        type Pick = u16;
        type Edges = [Pick; LETTERS];
        fn backtrack(edges: &Edges, cycles: &mut HashSet<Pick>, mut cycle: Pick, current: usize, cache: &mut HashSet<(usize, Pick)>) {
            debug_assert_eq!(cycle & (1 << current), 0);

            if cache.contains(&(current, cycle)) {
                return;
            }
            cache.insert((current, cycle));

            cycle |= 1 << current;

            if edges[current] & cycle != 0 {
                cycles.insert(cycle);
                return;
            }
            let next_mask = edges[current] & !cycle;
            for next in 0..LETTERS {
                if next_mask & (1 << next) != 0 {
                    backtrack(edges, cycles, cycle, next, cache);
                }
            }
        }

        fn backtrack2(unsolved_cycles: &[Pick], picked: Pick, solutions: &mut HashSet<Pick>) {
            let Some((next, rest)) = unsolved_cycles.split_first() else {
                solutions.insert(picked);
                return;
            };
            if next & picked == 0 {
                for l in 0..LETTERS {
                    if next & (1 << l) != 0 {
                        backtrack2(rest, picked | (1 << l), solutions);
                    }
                }
            } else {
                backtrack2(rest, picked, solutions);
            }
        }

        let mut present = HashSet::new();
        for word in &words {
            let &[b1, b2] = word.as_bytes() else {
                panic!("Invalid word: {word}");
            };

            present.insert(b1);
            present.insert(b2);
        }
        let idents = present.into_iter().enumerate().map(|(i, c)| (c, i)).collect::<HashMap<u8, usize>>();

        let mut edges = Edges::default();
        for word in words {
            let &[b1, b2] = word.as_bytes() else {
                panic!("Invalid word: {word}");
            };
            let i1 = idents[&b1];
            let i2 = idents[&b2];

            edges[i1] |= 1 << i2;
        }

        let mut cycles = HashSet::<Pick>::new();
        for start in 0..LETTERS {
            backtrack(&edges, &mut cycles, Pick::default(), start, &mut HashSet::new());
        }

        let mut cycles = cycles.into_iter().collect::<Vec<_>>();
        cycles.sort_by_key(|c| c.count_ones());
        let mut solutions = HashSet::new();
        backtrack2(cycles.as_slice(), Pick::default(), &mut solutions);

        let shortest_len = solutions.iter().map(|sol| sol.count_ones()).min().unwrap_or(0);

        let char_ind = idents
            .iter()
            .map(|(c, i)| (*i, (*c - b'a') as usize))
            .collect::<HashMap<usize, usize>>();

        solutions
            .into_iter()
            .filter(|sol| sol.count_ones() == shortest_len)
            .map(move |sol| {
                let mut res = vec![0; 26];
                for (i, &ind) in &char_ind {
                    if sol & (1 << i) != 0 {
                        res[ind] += 1;
                    }
                    res[ind] += 1;
                }
                res
            })
            .collect()
    }
}

#[test]
fn test() {
    let words = vec!["ab".to_string(), "ba".to_string()];
    let expected = vec![
        vec![1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let mut v = Solution::supersequences(words);
    v.sort();
    assert_eq!(v, expected);

    let words = vec!["aa".to_string(), "ac".to_string()];
    let expected = vec![vec![2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    assert_eq!(Solution::supersequences(words), expected);

    let words = vec!["aa".to_string(), "bb".to_string(), "cc".to_string()];
    let expected = vec![vec![2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    assert_eq!(Solution::supersequences(words), expected);
}
