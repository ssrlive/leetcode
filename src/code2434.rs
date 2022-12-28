#![allow(dead_code)]

// 2434. Using a Robot to Print the Lexicographically Smallest String
// https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/
// https://leetcode.cn/problems/using-a-robot-to-print-the-lexicographically-smallest-string/
//
// You are given a string s and a robot that currently holds an empty string t.
// Apply one of the following operations until s and t are both empty:
//
// Remove the first character of a string s and give it to the robot. The robot will append this character to the string t.
// Remove the last character of a string t and give it to the robot. The robot will write this character on paper.
//
// Return the lexicographically smallest string that can be written on the paper.
//
// Example 1:
//
// Input: s = "zza"
// Output: "azz"
// Explanation: Let p denote the written string.
// Initially p="", s="zza", t="".
// Perform first operation three times p="", s="", t="zza".
// Perform second operation three times p="azz", s="", t="".
//
// Example 2:
//
// Input: s = "bac"
// Output: "abc"
// Explanation: Let p denote the written string.
// Perform first operation twice p="", s="c", t="ba".
// Perform second operation twice p="ab", s="c", t="".
// Perform first operation p="ab", s="", t="c".
// Perform second operation p="abc", s="", t="".
//
// Example 3:
//
// Input: s = "bdda"
// Output: "addb"
// Explanation: Let p denote the written string.
// Initially p="", s="bdda", t="".
// Perform first operation four times p="", s="", t="bdda".
// Perform second operation four times p="addb", s="", t="".
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of only English lowercase letters.
//

struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        use std::collections::BTreeMap;
        let mut mp = BTreeMap::<char, i32>::new();

        for c in s.chars() {
            *mp.entry(c).or_default() += 1;
        }

        let mut stack = vec![];
        let mut ret = String::new();
        for c in s.chars() {
            stack.push(c);
            *mp.entry(c).or_default() -= 1;
            let cnt = mp.get(&c).unwrap();
            if *cnt == 0 {
                mp.remove(&c);
            }
            let mut ch = 'z';
            if !mp.is_empty() {
                ch = *mp.keys().next().unwrap();
            }

            while !stack.is_empty() && stack[stack.len() - 1] <= ch {
                ret.push(stack.pop().unwrap());
            }
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::robot_with_string("zza".to_string()), "azz".to_string());
    assert_eq!(Solution::robot_with_string("bac".to_string()), "abc".to_string());
    assert_eq!(Solution::robot_with_string("bdda".to_string()), "addb".to_string());
}
