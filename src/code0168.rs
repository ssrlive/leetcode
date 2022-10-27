#![allow(dead_code)]

// 168. Excel Sheet Column Title
// https://leetcode.com/problems/excel-sheet-column-title/
//
// Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
//
// For example:
// A -> 1
// B -> 2
// C -> 3
// ...
// Z -> 26
// AA -> 27
// AB -> 28
// ...
//

struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        let mut result = String::new();
        while column_number > 0 {
            column_number -= 1;
            let c = (column_number % 26) as u8 + b'A';
            result.insert(0, c as char);
            column_number /= 26;
        }
        result
    }
}

#[test]
fn test_convert_to_title() {
    assert_eq!(Solution::convert_to_title(1), "A");
    assert_eq!(Solution::convert_to_title(28), "AB");
    assert_eq!(Solution::convert_to_title(701), "ZY");
    assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW");
}
