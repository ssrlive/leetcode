#![allow(dead_code)]

// 1344. Angle Between Hands of a Clock
// https://leetcode.com/problems/angle-between-hands-of-a-clock/
// https://leetcode.cn/problems/angle-between-hands-of-a-clock/
//
// Medium
//
// Given two numbers, hour and minutes, return the smaller angle (in degrees) formed between the hour and the minute hand.
//
// Answers within 10-5 of the actual value will be accepted as correct.
//
// Example 1:
//
// Input: hour = 12, minutes = 30
// Output: 165
//
// Example 2:
//
// Input: hour = 3, minutes = 30
// Output: 75
//
// Example 3:
//
// Input: hour = 3, minutes = 15
// Output: 7.5
//
// Constraints:
//
// -    1 <= hour <= 12
// -    0 <= minutes <= 59
//

struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour = hour % 12;
        let hour_angle = (hour as f64) * 30.0 + (minutes as f64) * 0.5;
        let minute_angle = (minutes as f64) * 6.0;
        let angle = (hour_angle - minute_angle).abs();
        if angle > 180.0 {
            360.0 - angle
        } else {
            angle
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::angle_clock(12, 30), 165.0);
    assert_eq!(Solution::angle_clock(3, 30), 75.0);
    assert_eq!(Solution::angle_clock(3, 15), 7.5);
    assert_eq!(Solution::angle_clock(4, 50), 155.0);
    assert_eq!(Solution::angle_clock(12, 0), 0.0);
    assert_eq!(Solution::angle_clock(1, 57), 76.5);
}
