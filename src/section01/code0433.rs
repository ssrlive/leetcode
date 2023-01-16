#![allow(dead_code)]

// 433. Minimum Genetic Mutation
// https://leetcode.com/problems/minimum-genetic-mutation/
// https://leetcode.cn/problems/minimum-genetic-mutation/
//
// A gene string can be represented by an 8-character long string, with choices
// from "A", "C", "G", "T".
//
// Suppose we need to investigate about a mutation (mutation from "start" to
// "end"), where ONE mutation is defined as ONE single character changed in the
// gene string.
//
// For example, "AACCGGTT" -> "AACCGGTA" is 1 mutation.
//
// Also, there is a given gene "bank", which records all the valid gene
// mutations. A gene must be in the bank to make it a valid gene string.
//
// Now, given 3 things - start, end, bank, your task is to determine what is the
// minimum number of mutations needed to mutate from "start" to "end". If there
// is no such a mutation, return -1.
//
// Note:
//
// Starting point is assumed to be valid, so it might not be included in the
// bank.
// If multiple mutations are needed, all mutations during in the sequence must
// be valid.
// You may assume start and end string is not the same.
//
// Example 1:
//
// start: "AACCGGTT"
// end:   "AACCGGTA"
// bank: ["AACCGGTA"]
//
// return: 1
//
// Example 2:
//
// start: "AACCGGTT"
// end:   "AAACGGTA"
// bank: ["AACCGGTA", "AACCGCTA", "AAACGGTA"]
//
// return: 2
//
// Example 3:
//
// start: "AAAAACCC"
// end:   "AACCCCCC"
// bank: ["AAAACCCC", "AAACCCCC", "AACCCCCC"]
//
// return: 3

struct Solution;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut bank = bank.into_iter().collect::<std::collections::HashSet<_>>();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start, 0));
        while let Some((mut s, step)) = queue.pop_front() {
            if s == end {
                return step;
            }
            for i in 0..s.len() {
                let c = s.as_bytes()[i];
                for &ch in b"ACGT" {
                    if ch == c {
                        continue;
                    }
                    unsafe {
                        s.as_bytes_mut()[i] = ch;
                    }
                    if bank.remove(&s) {
                        queue.push_back((s.clone(), step + 1));
                    }
                }
                unsafe {
                    s.as_bytes_mut()[i] = c;
                }
            }
        }
        -1
    }
}

#[test]
fn test_min_mutation() {
    let start = "AACCGGTT".to_string();
    let end = "AACCGGTA".to_string();
    let bank = vec!["AACCGGTA".to_string()];
    assert_eq!(Solution::min_mutation(start, end, bank), 1);

    let start = "AACCGGTT".to_string();
    let end = "AAACGGTA".to_string();
    let bank = vec!["AACCGGTA".to_string(), "AACCGCTA".to_string(), "AAACGGTA".to_string()];
    assert_eq!(Solution::min_mutation(start, end, bank), 2);

    let start = "AAAAACCC".to_string();
    let end = "AACCCCCC".to_string();
    let bank = vec!["AAAACCCC".to_string(), "AAACCCCC".to_string(), "AACCCCCC".to_string()];
    assert_eq!(Solution::min_mutation(start, end, bank), 3);
}
