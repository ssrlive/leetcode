#![allow(dead_code)]

// 1109. Corporate Flight Bookings
// https://leetcode.com/problems/corporate-flight-bookings/
// https://leetcode.cn/problems/corporate-flight-bookings/
//
// There are n flights that are labeled from 1 to n.
//
// You are given an array of flight bookings bookings, where bookings[i] = [firsti, lasti, seatsi] represents a booking
// for flights firsti through lasti (inclusive) with seatsi seats reserved for each flight in the range.
//
// Return an array answer of length n, where answer[i] is the total number of seats reserved for flight i.
//
// Example 1:
//
// Input: bookings = [[1,2,10],[2,3,20],[2,5,25]], n = 5
// Output: [10,55,45,25,25]
// Explanation:
// Flight labels:        1   2   3   4   5
// Booking 1 reserved:  10  10
// Booking 2 reserved:      20  20
// Booking 3 reserved:      25  25  25  25
// Total seats:         10  55  45  25  25
// Hence, answer = [10,55,45,25,25]
//
// Example 2:
//
// Input: bookings = [[1,2,10],[2,2,15]], n = 2
// Output: [10,25]
// Explanation:
// Flight labels:        1   2
// Booking 1 reserved:  10  10
// Booking 2 reserved:      15
// Total seats:         10  25
// Hence, answer = [10,25]
//
// Constraints:
//
// - 1 <= n <= 2 * 10^4
// - 1 <= bookings.length <= 2 * 10^4
// - bookings[i].length == 3
// - 1 <= firsti <= lasti <= n
// - 1 <= seatsi <= 10^4
//

struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut memo = vec![(0, 0); n];
        let mut result = vec![0; n];
        let mut now = 0;

        for arr in bookings {
            memo[arr[0] as usize - 1].0 += arr[2];
            memo[arr[1] as usize - 1].1 += arr[2];
        }

        for i in 0..n {
            now += memo[i].0;
            result[i] = now;
            now -= memo[i].1;
        }

        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5, vec![10, 55, 45, 25, 25]),
        (vec![vec![1, 2, 10], vec![2, 2, 15]], 2, vec![10, 25]),
    ];
    for (bookings, n, expect) in cases {
        assert_eq!(Solution::corp_flight_bookings(bookings, n), expect);
    }
}
