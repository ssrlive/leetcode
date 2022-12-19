#![allow(dead_code)]

// 165. Compare Version Numbers
// https://leetcode.com/problems/compare-version-numbers/
// https://leetcode.cn/problems/compare-version-numbers/
//
// Given two version numbers, version1 and version2, compare them.
//
// Version numbers consist of one or more revisions joined by a dot '.'. Each revision consists of digits and may contain leading zeros. Every revision contains at least one character. Revisions are 0-indexed from left to right, with the leftmost revision being revision 0, the next revision being revision 1, and so on. For example 2.5.33 and 0.1 are valid version numbers.
//
// To compare version numbers, compare their revisions in left-to-right order. Revisions are compared using their integer value ignoring any leading zeros. This means that revisions 1 and 001 are considered equal. If a version number does not specify a revision at an index, then treat the revision as 0. For example, version 1.0 is less than version 1.1 because their revision 0s are the same, but their revision 1s are 0 and 1 respectively, and 0 < 1.
//
// Return the following:
// - If version1 < version2, return -1.
// - If version1 > version2, return 1.
// - Otherwise, return 0.
//

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        fn _compare_version(version1: String, version2: String) -> Option<i32> {
            let v1: Vec<&str> = version1.split('.').collect();
            let v2: Vec<&str> = version2.split('.').collect();
            let mut i = 0;
            while i < v1.len() || i < v2.len() {
                let n1 = if i < v1.len() { v1[i].parse::<i32>().ok()? } else { 0 };
                let n2 = if i < v2.len() { v2[i].parse::<i32>().ok()? } else { 0 };
                match n1.cmp(&n2) {
                    std::cmp::Ordering::Less => return Some(-1),
                    std::cmp::Ordering::Greater => return Some(1),
                    _ => {}
                }
                i += 1;
            }
            Some(0)
        }

        _compare_version(version1, version2).unwrap_or(0)
    }
}

#[test]
fn test_compare_version() {
    assert_eq!(Solution::compare_version("1.01".to_string(), "1.001".to_string()), 0);
    assert_eq!(Solution::compare_version("1.0".to_string(), "1.0.0".to_string()), 0);
    assert_eq!(Solution::compare_version("0.1".to_string(), "1.1".to_string()), -1);
    assert_eq!(Solution::compare_version("1.0.1".to_string(), "1".to_string()), 1);
}
