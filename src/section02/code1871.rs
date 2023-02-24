#![allow(dead_code)]

/*

// 1871. Jump Game VII
// https://leetcode.com/problems/jump-game-vii/
// https://leetcode.cn/problems/jump-game-vii/
//
// Medium
//
// You are given a 0-indexed binary string s and two integers minJump and maxJump. In the beginning, you are standing at index 0, which is equal to '0'. You can move from index i to index j if the following conditions are fulfilled:

    i + minJump <= j <= min(i + maxJump, s.length - 1), and
    s[j] == '0'.

Return true if you can reach index s.length - 1 in s, or false otherwise.

Example 1:

Input: s = "011010", minJump = 2, maxJump = 3
Output: true
Explanation:
In the first step, move from index 0 to index 3.
In the second step, move from index 3 to index 5.

Example 2:

Input: s = "01101110", minJump = 2, maxJump = 3
Output: false

Constraints:

    2 <= s.length <= 10^5
    s[i] is either '0' or '1'.
    s[0] == '0'
    1 <= minJump <= maxJump < s.length
*/

struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        use std::collections::VecDeque;
        assert!(min_jump >= 1);
        assert!(min_jump <= max_jump);
        let mn = min_jump as usize;
        let mx = max_jump as usize;

        let s = s.as_bytes();
        assert!((max_jump as usize) < s.len());
        if s[s.len() - 1] != b'0' {
            return false;
        }
        let mut last = 0;

        let mut queue = VecDeque::new();
        queue.push_back(0);

        while let Some(idx) = queue.pop_front() {
            let from = idx + mn;
            let to = idx + mx + 1;
            for pos in last.max(from)..s.len().min(to) {
                if s[pos] == b'0' {
                    if pos == s.len() - 1 {
                        return true;
                    }
                    queue.push_back(pos);
                }
            }
            last = last.max(idx + mx + 1);
        }
        false
    }
}

#[test]
fn test() {
    let cases = vec![
        ("011010", 2, 3, true),
        ("01101110", 2, 3, false),
        ("0000000000", 2, 3, true),
    ];
    for (s, mi, ma, ex) in cases {
        assert_eq!(Solution::can_reach(s.to_string(), mi, ma), ex);
    }
}
