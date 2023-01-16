#![allow(dead_code)]

// 1154. Day of the Year
// https://leetcode.com/problems/day-of-the-year/
// https://leetcode.cn/problems/day-of-the-year/
//
// Given a string date representing a Gregorian calendar date formatted as YYYY-MM-DD, return the day number of the year.
//
// Example 1:
//
// Input: date = "2019-01-09"
// Output: 9
// Explanation: Given date is the 9th day of the year in 2019.
//
// Example 2:
//
// Input: date = "2019-02-10"
// Output: 41
//
// Constraints:
//
// - date.length == 10
// - date[4] == date[7] == '-', and all other date[i]'s are digits
// - date represents a calendar date between Jan 1st, 1900 and Dec 31th, 2019.
//

struct Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let date_parts = date.split('-').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

        let (year, month, day) = (date_parts[0], date_parts[1], date_parts[2]);
        let leap_year_day = month > 2 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let days_in_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        days_in_months[..month as usize - 1].iter().sum::<i32>() + day + leap_year_day as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
    assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
    assert_eq!(Solution::day_of_year("2003-03-01".to_string()), 60);
    assert_eq!(Solution::day_of_year("2004-03-01".to_string()), 61);
}
