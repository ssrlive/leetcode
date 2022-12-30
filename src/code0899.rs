#![allow(dead_code)]

// 899. Orderly Queue
// https://leetcode.com/problems/orderly-queue/
// https://leetcode.cn/problems/orderly-queue/
//
// You are given a string s and an integer k. You can choose one of the first k letters of s and append it at the end of the string..
//
// Return the lexicographically smallest string you could have after applying the mentioned step any number of moves.
//
// Example 1:
//
// Input: s = "cba", k = 1
// Output: "acb"
// Explanation:
// In the first move, we move the 1st character 'c' to the end, obtaining the string "bac".
// In the second move, we move the 1st character 'b' to the end, obtaining the final result "acb".
//
// Example 2:
//
// Input: s = "baaca", k = 3
// Output: "aaabc"
// Explanation:
// In the first move, we move the 1st character 'b' to the end, obtaining the string "aacab".
// In the second move, we move the 3rd character 'c' to the end, obtaining the final result "aaabc".
//
// Constraints:
//
// - 1 <= k <= s.length <= 1000
// - s consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        if k == 1 {
            let n = s.len();
            let ss: Vec<_> = s.iter().chain(s.iter()).copied().collect();
            let min_i = ss.windows(n).enumerate().min_by_key(|p| p.1).unwrap().0;
            s.rotate_left(min_i);
        } else {
            s.sort_unstable();
        }
        String::from_utf8(s).unwrap()
    }
}

#[test]
fn test() {
    let s = "cba".to_string();
    let k = 1;
    let res = "acb".to_string();
    assert_eq!(Solution::orderly_queue(s, k), res);

    let s = "baaca".to_string();
    let k = 3;
    let res = "aaabc".to_string();
    assert_eq!(Solution::orderly_queue(s, k), res);
}
