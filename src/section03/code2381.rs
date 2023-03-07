#![allow(dead_code)]

/*

// 2381. Shifting Letters II
// https://leetcode.com/problems/shifting-letters-ii/
// https://leetcode.cn/problems/shifting-letters-ii/
//
// Medium
//
// You are given a string s of lowercase English letters and a 2D integer array shifts where shifts[i] = [starti, endi, directioni]. For every i, shift the characters in s from the index starti to the index endi (inclusive) forward if directioni = 1, or shift the characters backward if directioni = 0.

Shifting a character forward means replacing it with the next letter in the alphabet (wrapping around so that 'z' becomes 'a'). Similarly, shifting a character backward means replacing it with the previous letter in the alphabet (wrapping around so that 'a' becomes 'z').

Return the final string after all such shifts to s are applied.

Example 1:

Input: s = "abc", shifts = [[0,1,0],[1,2,1],[0,2,1]]
Output: "ace"
Explanation: Firstly, shift the characters from index 0 to index 1 backward. Now s = "zac".
Secondly, shift the characters from index 1 to index 2 forward. Now s = "zbd".
Finally, shift the characters from index 0 to index 2 forward. Now s = "ace".

Example 2:

Input: s = "dztz", shifts = [[0,0,0],[1,1,1]]
Output: "catz"
Explanation: Firstly, shift the characters from index 0 to index 0 backward. Now s = "cztz".
Finally, shift the characters from index 1 to index 1 forward. Now s = "catz".

Constraints:

    1 <= s.length, shifts.length <= 5 * 10^4
    shifts[i].length == 3
    0 <= starti <= endi < s.length
    0 <= directioni <= 1
    s consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        use std::collections::BinaryHeap;
        let s = s.chars().collect::<Vec<char>>();
        let mut ret: Vec<char> = vec![];
        let mut shifts = shifts;
        let mut pq = BinaryHeap::<(i32, i32)>::new();
        let mut j = 0;
        let mut count = 0;
        shifts.sort();
        for (i, &s_i) in s.iter().enumerate() {
            while j < shifts.len() && shifts[j][0] as usize <= i {
                pq.push((-shifts[j][1], shifts[j][2]));
                count += if shifts[j][2] == 1 { 1 } else { -1 };
                j += 1;
            }
            while let Some((a, b)) = pq.peek() {
                if a + i as i32 <= 0 {
                    break;
                }
                count -= if *b == 1 { 1 } else { -1 };
                pq.pop();
            }
            count %= 26;
            if count < 0 {
                count += 26;
            }
            let temp = (s_i as u8 - b'a' + count as u8) % 26;
            ret.push((b'a' + temp) as char);
        }
        ret.iter().collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("abc", vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]], "ace"),
        ("dztz", vec![vec![0, 0, 0], vec![1, 1, 1]], "catz"),
    ];
    for (s, shifts, expect) in cases {
        assert_eq!(Solution::shifting_letters(s.to_string(), shifts), expect.to_string());
    }
}
