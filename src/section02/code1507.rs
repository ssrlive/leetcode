#![allow(dead_code)]

/*

// 1507. Reformat Date
// https://leetcode.com/problems/reformat-date/
// https://leetcode.cn/problems/reformat-date/
//
// Easy
//
// Given a date string in the form Day Month Year, where:

    Day is in the set {"1st", "2nd", "3rd", "4th", ..., "30th", "31st"}.
    Month is in the set {"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"}.
    Year is in the range [1900, 2100].

Convert the date string to the format YYYY-MM-DD, where:

    YYYY denotes the 4 digit year.
    MM denotes the 2 digit month.
    DD denotes the 2 digit day.

Example 1:

Input: date = "20th Oct 2052"
Output: "2052-10-20"

Example 2:

Input: date = "6th Jun 1933"
Output: "1933-06-06"

Example 3:

Input: date = "26th May 1960"
Output: "1960-05-26"

Constraints:

    The given dates are guaranteed to be valid, so no error handling is necessary.
*/

struct Solution;

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let mut parts = date.split_whitespace();
        let day = parts.next().unwrap();
        let month = parts.next().unwrap();
        let year = parts.next().unwrap();
        let day = day[..day.len() - 2].parse::<i32>().unwrap();
        let month = match month {
            "Jan" => 1,
            "Feb" => 2,
            "Mar" => 3,
            "Apr" => 4,
            "May" => 5,
            "Jun" => 6,
            "Jul" => 7,
            "Aug" => 8,
            "Sep" => 9,
            "Oct" => 10,
            "Nov" => 11,
            "Dec" => 12,
            _ => unreachable!(),
        };
        format!("{year}-{month:02}-{day:02}")
    }
}

#[test]
fn test() {
    let cases = vec![
        ("20th Oct 2052", "2052-10-20"),
        ("6th Jun 1933", "1933-06-06"),
        ("26th May 1960", "1960-05-26"),
    ];
    for (date, expected) in cases {
        assert_eq!(Solution::reformat_date(date.to_string()), expected);
    }
}
