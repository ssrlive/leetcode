#![allow(dead_code)]

// 481. Magical String
// https://leetcode.com/problems/magical-string/
// https://leetcode.cn/problems/magical-string/
//
// A magical string s consists of only '1' and '2' and obeys the following rules:
//
// The string s is magical because concatenating the number of contiguous occurrences of characters '1' and '2' generates the string s itself.
// The first few elements of s is s = "1221121221221121122……". If we group the consecutive 1's and 2's in s, it will be "1 22 11 2 1 22 1 22 11 2 11 22 ......" and the occurrences of 1's or 2's in each group are "1 2 2 1 1 2 1 2 2 1 2 2 ......". You can see that the occurrence sequence is s itself.
//
// Given an integer n, return the number of 1's in the first n number in the magical string s.
//
// Example 1:
//
// Input: n = 6
// Output: 3
// Explanation: The first 6 elements of magical string s is "122112" and it contains three 1's, so return 3.
//
// Example 2:
//
// Input: n = 1
// Output: 1
//
// Constraints:
//
// - 1 <= n <= 10^5
//
// Follow up: Could you solve it in O(n) time?

struct Solution;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let mut ms = vec![1, 2, 2];
        let mut index = 2;
        let mut first = true;
        let mut one = 1;
        while ms.len() < n as usize {
            if ms[index] == 1 {
                ms.push(3 - ms[ms.len() - 1]);
                first = true;
                index += 1;
            } else {
                if first {
                    ms.push(3 - ms[ms.len() - 1]);
                } else {
                    ms.push(ms[ms.len() - 1]);
                    index += 1;
                }
                first = !first;
            }
            if ms[ms.len() - 1] == 1 {
                one += 1;
            }
        }
        one
    }
}

#[test]
fn test() {
    assert_eq!(Solution::magical_string(6), 3);
    assert_eq!(Solution::magical_string(1), 1);
}
