#![allow(dead_code)]

// 1096. Brace Expansion II
// https://leetcode.com/problems/brace-expansion-ii/
// https://leetcode.cn/problems/brace-expansion-ii/
//
// Under the grammar given below, strings can represent a set of lowercase words. Let R(expr) denote the set of words the expression represents.
//
// The grammar can best be understood through simple examples:
//
// Single letters represent a singleton set containing that word.
// R("a") = {"a"}
// R("w") = {"w"}
// When we take a comma-delimited list of two or more expressions, we take the union of possibilities.
// R("{a,b,c}") = {"a","b","c"}
// R("{{a,b},{b,c}}") = {"a","b","c"} (notice the final set only contains each word at most once)
// When we concatenate two expressions, we take the set of possible concatenations between two words where the
// first word comes from the first expression and the second word comes from the second expression.
// R("{a,b}{c,d}") = {"ac","ad","bc","bd"}
// R("a{b,c}{d,e}f{g,h}") = {"abdfg", "abdfh", "abefg", "abefh", "acdfg", "acdfh", "acefg", "acefh"}
// Formally, the three rules for our grammar:
//
// For every lowercase letter x, we have R(x) = {x}.
// For expressions e1, e2, ... , ek with k >= 2, we have R({e1, e2, ...}) = R(e1) ∪ R(e2) ∪ ...
// For expressions e1 and e2, we have R(e1 + e2) = {a + b for (a, b) in R(e1) × R(e2)}, where + denotes concatenation, and × denotes the cartesian product.
// Given an expression representing a set of words under the given grammar, return the sorted list of words that the expression represents.
//
// Example 1:
//
// Input: expression = "{a,b}{c,{d,e}}"
// Output: ["ac","ad","ae","bc","bd","be"]
//
// Example 2:
//
// Input: expression = "{{a,z},a{b,c},{ab,z}}"
// Output: ["a","ab","ac","z"]
// Explanation: Each distinct word is written only once in the final answer.
//
// Constraints:
//
// - 1 <= expression.length <= 60
// - expression[i] consists of '{', '}', ','or lowercase English letters.
// - The given expression represents a set of words based on the grammar given in the description.
//

struct Solution;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        use std::collections::BTreeSet;
        let mut balance = 0;
        let mut data = vec![];
        let mut expression = expression;

        Self::remove_brace(&mut expression);

        let mut str = String::new();
        for c in expression.chars() {
            if c == '{' {
                balance += 1;
            }
            if c == '}' {
                balance -= 1;
            }

            if c == ',' && balance == 0 {
                if !str.is_empty() {
                    data.push(str.clone());
                }
                str.clear();
            } else {
                str.push(c);
            }
        }
        if !str.is_empty() {
            data.push(str);
        }

        let mut st = BTreeSet::new();
        for mut s in data {
            let v = Self::process(&mut s);
            for it in v {
                st.insert(it);
            }
        }

        st.into_iter().collect()
    }

    fn process(s: &mut str) -> Vec<String> {
        let mut ret = vec![String::new()];

        let mut str = String::new();
        let mut balance = 0;

        for c in s.chars() {
            if c == '{' {
                balance += 1;

                if balance == 1 {
                    for item in &mut ret {
                        *item += &str;
                    }
                    str.clear();
                } else {
                    str.push(c);
                }
                continue;
            }

            if c == '}' {
                balance -= 1;

                if balance == 0 {
                    if str.is_empty() {
                        continue;
                    }
                    let data = Self::brace_expansion_ii(str.clone());
                    let mut temp = vec![];

                    for a in &ret {
                        for it in &data {
                            let mut it1 = (*a).clone();
                            it1 += it;
                            temp.push(it1);
                        }
                    }
                    ret = temp;
                    str.clear();
                } else {
                    str.push(c);
                }
                continue;
            }

            str.push(c);
        }

        for item in &mut ret {
            *item += &str;
        }

        ret
    }

    fn remove_brace(s: &mut String) {
        let n = s.len();
        let mut t = s.chars().collect::<Vec<char>>();
        if t[0] != '{' || t[n - 1] != '}' {
            return;
        }

        let mut balance = 0;
        for (i, &item) in t.iter().enumerate().take(n) {
            if item == '{' {
                balance += 1;
            }
            if item == '}' {
                balance -= 1;
            }
            if i > 0 && i + 1 < n && balance == 0 {
                return;
            }
        }

        t.pop();
        t.reverse();
        t.pop();
        t.reverse();
        *s = t.iter().collect();
    }
}

#[test]
fn test() {
    let expression = "{a,b}{c,{d,e}}".to_string();
    let ret = Solution::brace_expansion_ii(expression);
    assert_eq!(ret, vec!["ac", "ad", "ae", "bc", "bd", "be"]);

    let expression = "{{a,z},a{b,c},{ab,z}}".to_string();
    let ret = Solution::brace_expansion_ii(expression);
    assert_eq!(ret, vec!["a", "ab", "ac", "z"]);
}
