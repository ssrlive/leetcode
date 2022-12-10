#![allow(dead_code)]

// 726. Number of Atoms
// https://leetcode.com/problems/number-of-atoms/
//
// Given a string formula representing a chemical formula, return the count of each atom.
//
// The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.
//
// One or more digits representing that element's count may follow if the count is greater than 1. If the count is 1, no digits will follow.
//
// - For example, "H2O" and "H2O2" are possible, but "H1O2" is impossible.
//
// Two formulas are concatenated together to produce another formula.
//
// - For example, "H2O2He3Mg4" is also a formula.
//
// A formula placed in parentheses, and a count (optionally added) is also a formula.
//
// - For example, "(H2O2)" and "(H2O2)3" are formulas.
//
// Return the count of all elements as a string in the following form: the first name (in sorted order), followed by its count (if that count is more than 1),
// followed by the second name (in sorted order), followed by its count (if that count is more than 1), and so on.
//
// The test cases are generated so that all the values in the output fit in a 32-bit integer.
//
// Example 1:
//
// Input: formula = "H2O"
// Output: "H2O"
// Explanation: The count of elements are {'H': 2, 'O': 1}.
//
// Example 2:
//
// Input: formula = "Mg(OH)2"
// Output: "H2MgO2"
// Explanation: The count of elements are {'H': 2, 'Mg': 1, 'O': 2}.
//
// Example 3:
//
// Input: formula = "K4(ON(SO3)2)2"
// Output: "K4N2O14S4"
// Explanation: The count of elements are {'K': 4, 'N': 2, 'O': 14, 'S': 4}.
//
// Constraints:
//
// - 1 <= formula.length <= 1000
// - formula consists of English letters, digits, '(', and ')'.
// - formula is always valid.
//

struct Solution;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        use std::collections::BTreeMap;
        let mut stack = vec![];
        let mut map = BTreeMap::new();
        let mut i = 0;
        let n = formula.len();

        while i < n {
            let c = formula.chars().nth(i).unwrap();
            if c == '(' {
                stack.push(map);
                map = BTreeMap::new();
                i += 1;
            } else if c == ')' {
                i += 1;
                let mut count = 0;
                while i < n && formula.chars().nth(i).unwrap().is_ascii_digit() {
                    count = count * 10 + (formula.chars().nth(i).unwrap() as u8 - b'0') as usize;
                    i += 1;
                }
                if count == 0 {
                    count = 1;
                }
                let mut tmp = BTreeMap::new();
                for (k, v) in map {
                    tmp.insert(k, v * count);
                }
                map = stack.pop().unwrap();
                for (k, v) in tmp {
                    *map.entry(k).or_insert(0) += v;
                }
            } else {
                let mut name = String::new();
                name.push(c);
                i += 1;
                while i < n && formula.chars().nth(i).unwrap().is_ascii_lowercase() {
                    name.push(formula.chars().nth(i).unwrap());
                    i += 1;
                }
                let mut count = 0;
                while i < n && formula.chars().nth(i).unwrap().is_ascii_digit() {
                    count = count * 10 + (formula.chars().nth(i).unwrap() as u8 - b'0') as usize;
                    i += 1;
                }
                if count == 0 {
                    count = 1;
                }
                *map.entry(name).or_insert(0) += count;
            }
        }

        let mut res = String::new();
        for (k, v) in map {
            res.push_str(&k);
            if v > 1 {
                res.push_str(&v.to_string());
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_of_atoms("H2O".to_string()), "H2O".to_string());
    assert_eq!(Solution::count_of_atoms("Mg(OH)2".to_string()), "H2MgO2".to_string());
    assert_eq!(
        Solution::count_of_atoms("K4(ON(SO3)2)2".to_string()),
        "K4N2O14S4".to_string()
    );
}
