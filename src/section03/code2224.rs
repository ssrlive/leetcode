#![allow(dead_code)]

/*

// 2224. Minimum Number of Operations to Convert Time
// https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/
// https://leetcode.cn/problems/minimum-number-of-operations-to-convert-time/
//
// Easy
//
// You are given two strings current and correct representing two 24-hour times.

24-hour times are formatted as "HH:MM", where HH is between 00 and 23, and MM is between 00 and 59. The earliest 24-hour time is 00:00, and the latest is 23:59.

In one operation you can increase the time current by 1, 5, 15, or 60 minutes. You can perform this operation any number of times.

Return the minimum number of operations needed to convert current to correct.

Example 1:

Input: current = "02:30", correct = "04:35"
Output: 3
Explanation:
We can convert current to correct in 3 operations as follows:
- Add 60 minutes to current. current becomes "03:30".
- Add 60 minutes to current. current becomes "04:30".
- Add 5 minutes to current. current becomes "04:35".
It can be proven that it is not possible to convert current to correct in fewer than 3 operations.

Example 2:

Input: current = "11:00", correct = "11:01"
Output: 1
Explanation: We only have to add one minute to current, so the minimum number of operations needed is 1.

Constraints:

    current and correct are in the format "HH:MM"
    current <= correct
*/

struct Solution;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        fn to_minutes(time: String) -> i32 {
            let mut time = time.split(':');
            let hours: i32 = time.next().unwrap().parse().unwrap();
            let minutes: i32 = time.next().unwrap().parse().unwrap();
            hours * 60 + minutes
        }

        let mut current = to_minutes(current);
        let correct = to_minutes(correct);
        let mut counter = 0;
        while current < correct {
            if current + 60 <= correct {
                current += 60;
            } else if current + 15 <= correct {
                current += 15;
            } else if current + 5 <= correct {
                current += 5;
            } else {
                current += 1;
            }
            counter += 1;
        }
        counter
    }
}

#[test]
fn test() {
    let cases = vec![
        ("02:30", "04:35", 3),
        ("11:00", "11:01", 1),
        ("00:00", "23:59", 32),
        ("00:00", "00:00", 0),
    ];
    for (c, correct, ex) in cases {
        assert_eq!(Solution::convert_time(c.to_string(), correct.to_string()), ex);
    }
}
