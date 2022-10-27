#![allow(dead_code)]

// 171. Excel Sheet Column Number
// https://leetcode.com/problems/excel-sheet-column-number/
//
// Given a string columnTitle that represents the column title as appears in an Excel sheet, return its corresponding column number.
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
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;
        for c in column_title.chars() {
            result = result * 26 + (c as u8 - b'A' + 1) as i32;
        }
        result
    }
}

#[test]
fn test_title_to_number() {
    assert_eq!(Solution::title_to_number("A".to_string()), 1);
    assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    assert_eq!(Solution::title_to_number("FXSHRXW".to_string()), 2147483647);
}
