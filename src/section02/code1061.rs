#![allow(dead_code)]

/*
// 1061. Lexicographically Smallest Equivalent String
// https://leetcode.com/problems/lexicographically-smallest-equivalent-string/
// https://leetcode.cn/problems/lexicographically-smallest-equivalent-string/
//
// Medium
//
// You are given two strings of the same length s1 and s2 and a string baseStr.

We say s1[i] and s2[i] are equivalent characters.

For example, if s1 = "abc" and s2 = "cde", then we have 'a' == 'c', 'b' == 'd', and 'c' == 'e'.
Equivalent characters follow the usual rules of any equivalence relation:

Reflexivity: 'a' == 'a'.
Symmetry: 'a' == 'b' implies 'b' == 'a'.
Transitivity: 'a' == 'b' and 'b' == 'c' implies 'a' == 'c'.
For example, given the equivalency information from s1 = "abc" and s2 = "cde", "acd" and "aab" are equivalent strings of baseStr = "eed", and "aab" is the lexicographically smallest equivalent string of baseStr.

Return the lexicographically smallest equivalent string of baseStr by using the equivalency information from s1 and s2.

Example 1:

Input: s1 = "parker", s2 = "morris", baseStr = "parser"
Output: "makkek"
Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [m,p], [a,o], [k,r,s], [e,i].
The characters in each group are equivalent and sorted in lexicographical order.
So the answer is "makkek".

Example 2:

Input: s1 = "hello", s2 = "world", baseStr = "hold"
Output: "hdld"
Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [h,w], [d,e,o], [l,r].
So only the second letter 'o' in baseStr is changed to 'd', the answer is "hdld".

Example 3:

Input: s1 = "leetcode", s2 = "programs", baseStr = "sourcecode"
Output: "aauaaaaada"
Explanation: We group the equivalent characters in s1 and s2 as [a,o,e,r,s,c], [l,p], [g,t] and [d,m], thus all letters in baseStr except 'u' and 'd' are transformed to 'a', the answer is "aauaaaaada".

Constraints:

1 <= s1.length, s2.length, baseStr <= 1000
s1.length == s2.length
s1, s2, and baseStr consist of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        base_str
            .chars()
            .scan(
                Iterator::zip(s1.chars(), s2.chars()).fold(UnionFind::from((0..26usize).collect::<Vec<usize>>()), |uf, (c1, c2)| {
                    match (uf.find(ctu(c1)), uf.find(ctu(c2))) {
                        (h1, h2) if h1 > h2 => uf.union(h2, h1),
                        (h1, h2) => uf.union(h1, h2),
                    }
                }),
                |m, c| Some(utc(m.find(ctu(c)))),
            )
            .collect()
    }
}

struct UnionFind {
    content: Vec<usize>,
}
impl UnionFind {
    fn union(mut self, i: usize, j: usize) -> Self {
        self.content[j] = self.content[i];
        self
    }
    fn find(&self, i: usize) -> usize {
        match self.content[i] {
            p if p == i => i,
            p => self.find(p),
        }
    }
}
impl From<Vec<usize>> for UnionFind {
    fn from(s: Vec<usize>) -> Self {
        UnionFind { content: s }
    }
}

fn ctu(c: char) -> usize {
    c as usize - 'a' as usize
}
fn utc(i: usize) -> char {
    (i + 'a' as usize) as u8 as char
}

#[test]
fn test() {
    let cases = vec![
        ("parker", "morris", "parser", "makkek"),
        ("hello", "world", "hold", "hdld"),
        ("leetcode", "programs", "sourcecode", "aauaaaaada"),
    ];
    for (s1, s2, base_str, expected) in cases {
        assert_eq!(
            Solution::smallest_equivalent_string(s1.to_string(), s2.to_string(), base_str.to_string()),
            expected.to_string()
        );
    }
}
