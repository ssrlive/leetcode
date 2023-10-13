#![allow(dead_code)]

// 14. Longest Common Prefix
// https://leetcode.com/problems/longest-common-prefix/
// https://leetcode.cn/problems/longest-common-prefix/
//
// Write a function to find the longest common prefix string amongst an array of strings.
//
// If there is no common prefix, return an empty string "".
//
// Example 1:
//
// Input: strs = ["flower","flow","flight"]
// Output: "fl"
//
// Example 2:
//
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
//
// Constraints:
//
// - 1 <= strs.length <= 200
// - 0 <= strs[i].length <= 200
// - strs[i] consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut prefix = strs[0].clone();
        for item in strs.iter().skip(1) {
            let mut j = 0;
            while j < prefix.len() && j < item.len() && prefix.as_bytes()[j] == item.as_bytes()[j] {
                j += 1;
            }
            prefix = prefix[..j].to_string();
        }
        prefix
    }
}

#[test]
fn test() {
    let strs = ["flower", "flow", "flight"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());

    let strs = ["dog", "racecar", "car"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
}
