#![allow(dead_code)]

// 1405. Longest Happy String
// https://leetcode.com/problems/longest-happy-string/
// https://leetcode.cn/problems/longest-happy-string/
//
// Medium
//
// A string s is called happy if it satisfies the following conditions:
//
//     s only contains the letters 'a', 'b', and 'c'.
//     s does not contain any of "aaa", "bbb", or "ccc" as a substring.
//     s contains at most a occurrences of the letter 'a'.
//     s contains at most b occurrences of the letter 'b'.
//     s contains at most c occurrences of the letter 'c'.
//
// Given three integers a, b, and c, return the longest possible happy string.
// If there are multiple longest happy strings, return any of them.
// If there is no such string, return the empty string "".
//
// A substring is a contiguous sequence of characters within a string.
//
// Example 1:
//
// Input: a = 1, b = 1, c = 7
// Output: "ccaccbcc"
// Explanation: "ccbccacc" would also be a correct answer.
//
// Example 2:
//
// Input: a = 7, b = 1, c = 0
// Output: "aabaa"
// Explanation: It is the only correct answer in this case.
//
// Constraints:
//
// -    0 <= a, b, c <= 100
// -    a + b + c > 0
//

struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        use std::collections::BinaryHeap;
        let mut string = String::with_capacity((a + b + c) as usize);
        let mut heap = BinaryHeap::with_capacity(3);
        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }
        let mut total = a + b + c;
        let mut last = None;
        while let Some((mut count, ch)) = heap.pop() {
            string.push(ch);
            count -= 1;
            total -= 1;
            if count > (total - count) * 2 {
                count -= 1;
                total -= 1;
                string.push(ch);
            }
            if let Some(last) = last.take() {
                heap.push(last);
            }
            if count > 0 {
                last = Some((count, ch));
            }
        }
        string
    }
}

#[test]
fn test() {
    let cases = vec![(1, 1, 7, "ccbccacc"), (7, 1, 0, "aabaa"), (2, 2, 1, "bacba"), (7, 1, 1, "aacaabaa")];
    for (a, b, c, expect) in cases {
        assert_eq!(Solution::longest_diverse_string(a, b, c), expect);
    }
}
