#![allow(dead_code)]

// 6. Zigzag Conversion
// https://leetcode.com/problems/zigzag-conversion/
// https://leetcode.cn/problems/zigzag-conversion/
//
// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
// (you may want to display this pattern in a fixed font for better legibility)
//
// P   A   H   N
// A P L S I I G
// Y   I   R
//
// And then read line by line: "PAHNAPLSIIGYIR"
//
// Write the code that will take a string and make this conversion given a number of rows:
//
// string convert(string s, int numRows);
//
// Example 1:
//
// Input: s = "PAYPALISHIRING", numRows = 3
// Output: "PAHNAPLSIIGYIR"
//
// Example 2:
//
// Input: s = "PAYPALISHIRING", numRows = 4
// Output: "PINALSIGYAHRPI"
// Explanation:
// P     I    N
// A   L S  I G
// Y A   H R
// P     I
//
// Example 3:
//
// Input: s = "A", numRows = 1
// Output: "A"
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s consists of English letters (lower-case and upper-case), ',' and '.'.
// - 1 <= numRows <= 1000
//

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut res = String::new();
        let n = s.len() as i32;
        let cycle_len = 2 * num_rows - 2;
        for i in 0..num_rows {
            let mut j = 0;
            while j + i < n {
                res.push(s.chars().nth((j + i) as usize).unwrap());
                if i != 0 && i != num_rows - 1 && j + cycle_len - i < n {
                    res.push(s.chars().nth((j + cycle_len - i) as usize).unwrap());
                }
                j += cycle_len;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("PAYPALISHIRING".to_string(), 3, "PAHNAPLSIIGYIR".to_string()),
        ("PAYPALISHIRING".to_string(), 4, "PINALSIGYAHRPI".to_string()),
        ("A".to_string(), 1, "A".to_string()),
    ];
    for (s, num_rows, expected) in cases {
        assert_eq!(Solution::convert(s, num_rows), expected);
    }
}
