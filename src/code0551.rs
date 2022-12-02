#![allow(dead_code)]

// 551. Student Attendance Record I
// https://leetcode.com/problems/student-attendance-record-i/
//
// You are given a string s representing an attendance record for a student where each character signifies whether
// the student was absent, late, or present on that day. The record only contains the following three characters:
//
// - 'A': Absent.
// - 'L': Late.
// - 'P': Present.
//
// The student is eligible for an attendance award if they meet both of the following criteria:
//
// - The student was absent ('A') for strictly fewer than 2 days total.
// - The student was never late ('L') for 3 or more consecutive days.
//
// Return true if the student is eligible for an attendance award, or false otherwise.
//
// Example 1:
//
// Input: s = "PPALLP"
// Output: true
// Explanation: The student has fewer than 2 absences and was never late 3 or more consecutive days.
//
// Example 2:
//
// Input: s = "PPALLL"
// Output: false
// Explanation: The student was late 3 consecutive days in the last 3 days, so is not eligible for the award.
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s[i] is either 'A', 'L', or 'P'.
//

struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent = 0;
        let mut late = 0;
        for c in s.chars() {
            match c {
                'A' => {
                    absent += 1;
                    late = 0;
                }
                'L' => {
                    late += 1;
                }
                _ => {
                    late = 0;
                }
            }
            if absent >= 2 || late >= 3 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        ("PPALLP", true),
        ("PPALLL", false),
        ("LALL", true),
        ("LALLL", false),
        ("LALLLL", false),
        ("LALLLLL", false),
        ("LALLLLLL", false),
        ("LALLLLLLLLLLLLLL", false),
        ("LALLLLLLLLLLLLLLL", false),
        ("LALLLLLLLLLLLLLLLLLL", false),
        ("LALLLLLLLLLLLLLLLLLLL", false),
        ("LALLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL", false),
        ("LALLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL", false),
        ("LALLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL", false),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::check_record(s.to_string()), expected);
    }
}
