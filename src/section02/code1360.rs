#![allow(dead_code)]

// 1360. Number of Days Between Two Dates
// https://leetcode.com/problems/number-of-days-between-two-dates/
// https://leetcode.cn/problems/number-of-days-between-two-dates/
//
// Easy
//
// Write a program to count the number of days between two dates.
//
// The two dates are given as strings, their format is YYYY-MM-DD as shown in the examples.
//
// Example 1:
//
// Input: date1 = "2019-06-29", date2 = "2019-06-30"
// Output: 1
//
// Example 2:
//
// Input: date1 = "2020-01-15", date2 = "2019-12-31"
// Output: 15
//
// Constraints:
//
// -    The given dates are valid dates between the years 1971 and 2100.
//

struct Solution;

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        if date1 == date2 {
            return 0;
        }

        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap_year = |x: i32| -> bool { x % 4 == 0 && (x % 100 != 0 || x % 400 == 0) };
        let month_day_calc = |year: i32, month: i32, day: i32| -> i32 {
            (0..month - 1)
                .map(|x| months[x as usize] + (x == 1 && is_leap_year(year)) as i32)
                .sum::<i32>()
                + day
        };

        let str_to_int = |s: &str| -> i32 { s.parse().unwrap() };
        let mut min_date_vec = date1.split('-').map(str_to_int).collect::<Vec<_>>();
        let mut max_date_vec = date2.split('-').map(str_to_int).collect::<Vec<_>>();
        if date1 > date2 {
            std::mem::swap(&mut min_date_vec, &mut max_date_vec);
        }

        let (min_year, min_month, min_day) = (min_date_vec[0], min_date_vec[1], min_date_vec[2]);
        let (max_year, max_month, max_day) = (max_date_vec[0], max_date_vec[1], max_date_vec[2]);

        (min_year..max_year)
            .map(|year| if !is_leap_year(year) { 365 } else { 366 })
            .sum::<i32>()
            - month_day_calc(min_year, min_month, min_day)
            + month_day_calc(max_year, max_month, max_day)
    }
}

#[test]
fn test() {
    let date1 = "2019-06-29".to_string();
    let date2 = "2019-06-30".to_string();
    assert_eq!(Solution::days_between_dates(date1, date2), 1);

    let date1 = "2020-01-15".to_string();
    let date2 = "2019-12-31".to_string();
    assert_eq!(Solution::days_between_dates(date1, date2), 15);

    let date1 = "1971-06-29".to_string();
    let date2 = "2010-09-23".to_string();
    assert_eq!(Solution::days_between_dates(date1, date2), 14331);
}
