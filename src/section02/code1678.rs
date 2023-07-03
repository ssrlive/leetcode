#![allow(dead_code)]

/*

// 1678. Goal Parser Interpretation
// https://leetcode.com/problems/goal-parser-interpretation/
// https://leetcode.cn/problems/goal-parser-interpretation/
//
// Easy
//
// You own a Goal Parser that can interpret a string command. The command consists of an alphabet of "G", "()" and/or "(al)" in some order. The Goal Parser will interpret "G" as the string "G", "()" as the string "o", and "(al)" as the string "al". The interpreted strings are then concatenated in the original order.

Given the string command, return the Goal Parser's interpretation of command.

Example 1:

Input: command = "G()(al)"
Output: "Goal"
Explanation: The Goal Parser interprets the command as follows:
G -> G
() -> o
(al) -> al
The final concatenated result is "Goal".

Example 2:

Input: command = "G()()()()(al)"
Output: "Gooooal"

Example 3:

Input: command = "(al)G(al)()()G"
Output: "alGalooG"

Constraints:

    1 <= command.length <= 100
    command consists of "G", "()", and/or "(al)" in some order.
*/

struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut res = String::new();
        let mut i = 0;
        while i < command.len() {
            if command[i..].starts_with('G') {
                res.push('G');
                i += 1;
            } else if command[i..].starts_with("()") {
                res.push('o');
                i += 2;
            } else if command[i..].starts_with("(al)") {
                res.push_str("al");
                i += 4;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![("G()(al)", "Goal"), ("G()()()()(al)", "Gooooal"), ("(al)G(al)()()G", "alGalooG")];
    for (command, expected) in cases {
        assert_eq!(Solution::interpret(command.to_string()), expected);
    }
}
