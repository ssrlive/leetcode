#![allow(dead_code)]

// 949. Largest Time for Given Digits
// https://leetcode.com/problems/largest-time-for-given-digits/
// https://leetcode.cn/problems/largest-time-for-given-digits/
//
// Given an array arr of 4 digits, find the latest 24-hour time that can be made using each digit exactly once.
//
// 24-hour times are formatted as "HH:MM", where HH is between 00 and 23, and MM is between 00 and 59. The earliest 24-hour time is 00:00, and the latest is 23:59.
//
// Return the latest 24-hour time in "HH:MM" format. If no valid time can be made, return an empty string.
//
// Example 1:
//
// Input: arr = [1,2,3,4]
// Output: "23:41"
// Explanation: The valid 24-hour times are "12:34", "12:43", "13:24", "13:42", "14:23", "14:32", "21:34", "21:43", "23:14", and "23:41". Of these times, "23:41" is the latest.
//
// Example 2:
//
// Input: arr = [5,5,5,5]
// Output: ""
// Explanation: There are no valid 24-hour times as "55:55" is not valid.
//
// Constraints:
//
// - arr.length == 4
// - 0 <= arr[i] <= 9
//

struct Solution;

impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        fn helper(a: &mut [i32], i: usize, answers: &mut Vec<(i32, i32)>) {
            if i == 4 {
                let (h, m) = (a[0] * 10 + a[1], a[2] * 10 + a[3]);
                if h < 24 && m < 60 {
                    answers.push((h, m));
                }
                return;
            }
            for j in i..4 {
                a.swap(i, j);
                helper(a, i + 1, answers);
                a.swap(i, j);
            }
        }

        let mut arr = arr;
        let mut answers: Vec<(i32, i32)> = Vec::new();
        helper(&mut arr, 0, &mut answers);
        if let Some(max) = answers.iter().max() {
            format!("{:02}:{:02}", max.0, max.1)
        } else {
            String::new()
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::largest_time_from_digits(vec![1, 2, 3, 4]),
        "23:41".to_string()
    );
    assert_eq!(Solution::largest_time_from_digits(vec![5, 5, 5, 5]), "".to_string());
}
