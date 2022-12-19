#![allow(dead_code)]

// 756. Pyramid Transition Matrix
// https://leetcode.com/problems/pyramid-transition-matrix/
// https://leetcode.cn/problems/pyramid-transition-matrix/
//
// You are stacking blocks to form a pyramid. Each block has a color, which is represented by a single letter.
// Each row of blocks contains one less block than the row beneath it and is centered on top.
//
// To make the pyramid aesthetically pleasing, there are only specific triangular patterns that are allowed.
// A triangular pattern consists of a single block stacked on top of two blocks.
// The patterns are given as a list of three-letter strings allowed, where the first two characters of a pattern
// represent the left and right bottom blocks respectively, and the third character is the top block.
//
// - For example, "ABC" represents a triangular pattern with a 'C' block stacked on top of an 'A' (left) and 'B' (right) block.
//   Note that this is different from "BAC" where 'B' is on the left bottom and 'A' is on the right bottom.
//
// You start with a bottom row of blocks bottom, given as a single string, that you must use as the base of the pyramid.
//
// Given bottom and allowed, return true if you can build the pyramid all the way to the top such that every triangular
// pattern in the pyramid is in allowed, or false otherwise.
//
// Example 1:
//
// Input: bottom = "BCD", allowed = ["BCC","CDE","CEA","FFF"]
// Output: true
// Explanation: The allowed triangular patterns are shown on the right.
// Starting from the bottom (level 3), we can build "CE" on level 2 and then build "A" on level 1.
// There are three triangular patterns in the pyramid, which are "BCC", "CDE", and "CEA". All are allowed.
//
// Example 2:
//
// Input: bottom = "AAAA", allowed = ["AAB","AAC","BCD","BBE","DEF"]
// Output: false
// Explanation: The allowed triangular patterns are shown on the right.
// Starting from the bottom (level 4), there are multiple ways to build level 3, but trying all the possibilites,
// you will get always stuck before building level 1.
//
// Constraints:
//
// - 2 <= bottom.length <= 6
// - 0 <= allowed.length <= 216
// - allowed[i].length == 3
// - The letters in all input strings are from the set {'A', 'B', 'C', 'D', 'E', 'F'}.
// - All the values of allowed are unique.
//

struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut m = HashMap::new();
        for s in allowed {
            m.entry(s[0..2].to_string())
                .or_insert_with(Vec::new)
                .push(s[2..3].to_string());
        }
        let mut failed_states = HashSet::new();
        Self::helper(&bottom, &m, 0, "", &mut failed_states)
    }

    fn helper(
        bottom: &str,
        m: &HashMap<String, Vec<String>>,
        start: usize,
        next: &str,
        failed_states: &mut HashSet<String>,
    ) -> bool {
        if bottom.len() == 1 {
            return true;
        }
        if start == bottom.len() - 1 {
            return Self::helper(next, m, 0, "", failed_states);
        }
        if failed_states.contains(&(bottom[start..].to_string() + "#" + next)) {
            return false;
        }
        let v = m
            .get(&(bottom[start..start + 2].to_string()))
            .unwrap_or(&vec![])
            .clone();
        for c in v.into_iter() {
            let next = next.to_string() + c.as_str();
            if Self::helper(bottom, m, start + 1, &next, failed_states) {
                return true;
            }
        }
        failed_states.insert(bottom[start..].to_string() + "#" + next);
        false
    }
}

#[test]
fn test() {
    let allowed = vec!["BCC", "CDE", "CEA", "FFF"];
    let allowed = allowed.iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::pyramid_transition("BCD".to_string(), allowed), true);

    let allowed = vec!["AAB", "AAC", "BCD", "BBE", "DEF"];
    let allowed = allowed.iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::pyramid_transition("AAAA".to_string(), allowed), false);

    let allowed = vec![
        "CBB", "ACB", "ABD", "CDB", "BDC", "CBC", "DBA", "DBB", "CAB", "BCB", "BCC", "BAA", "CCD", "BDD", "DDD", "CCA",
        "CAA", "CCC", "CCB",
    ];
    let allowed = allowed.iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::pyramid_transition("CCC".to_string(), allowed), true);
}
