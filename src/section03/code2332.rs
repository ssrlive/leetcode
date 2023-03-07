#![allow(dead_code)]

/*

// 2332. The Latest Time to Catch a Bus
// https://leetcode.com/problems/the-latest-time-to-catch-a-bus/
// https://leetcode.cn/problems/the-latest-time-to-catch-a-bus/
//
// Medium
//
// You are given a 0-indexed integer array buses of length n, where buses[i] represents the departure time of the ith bus. You are also given a 0-indexed integer array passengers of length m, where passengers[j] represents the arrival time of the jth passenger. All bus departure times are unique. All passenger arrival times are unique.

You are given an integer capacity, which represents the maximum number of passengers that can get on each bus.

When a passenger arrives, they will wait in line for the next available bus. You can get on a bus that departs at x minutes if you arrive at y minutes where y <= x, and the bus is not full. Passengers with the earliest arrival times get on the bus first.

More formally when a bus arrives, either:

    If capacity or fewer passengers are waiting for a bus, they will all get on the bus, or
    The capacity passengers with the earliest arrival times will get on the bus.

Return the latest time you may arrive at the bus station to catch a bus. You cannot arrive at the same time as another passenger.

Note: The arrays buses and passengers are not necessarily sorted.

Example 1:

Input: buses = [10,20], passengers = [2,17,18,19], capacity = 2
Output: 16
Explanation: Suppose you arrive at time 16.
At time 10, the first bus departs with the 0th passenger.
At time 20, the second bus departs with you and the 1st passenger.
Note that you may not arrive at the same time as another passenger, which is why you must arrive before the 1st passenger to catch the bus.

Example 2:

Input: buses = [20,30,10], passengers = [19,13,26,4,25,11,21], capacity = 2
Output: 20
Explanation: Suppose you arrive at time 20.
At time 10, the first bus departs with the 3rd passenger.
At time 20, the second bus departs with the 5th and 1st passengers.
At time 30, the third bus departs with the 0th passenger and you.
Notice if you had arrived any later, then the 6th passenger would have taken your seat on the third bus.

Constraints:

    n == buses.length
    m == passengers.length
    1 <= n, m, capacity <= 10^5
    2 <= buses[i], passengers[i] <= 10^9
    Each element in buses is unique.
    Each element in passengers is unique.
*/

struct Solution;

impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let mut b = buses;
        let mut p = passengers;
        let n = p.len();
        let (mut cnt, mut last, mut ret) = (0i32, 0i32, 1i32);
        b.sort();
        b.reverse();
        p.sort();
        for i in 0..n {
            while !b.is_empty() && p[i] > b[b.len() - 1] {
                ret = b[b.len() - 1];
                b.pop();
            }
            if b.is_empty() {
                break;
            }
            if last < p[i] - 1 {
                ret = p[i] - 1;
            }
            last = p[i];
            cnt += 1;
            if i == n - 1 || cnt == capacity || p[i + 1] > b[b.len() - 1] {
                if cnt < capacity && p[i] < b[b.len() - 1] {
                    ret = b[b.len() - 1];
                }
                b.pop();
                cnt = 0;
            }
        }
        if !b.is_empty() {
            return b[0];
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![10, 20], vec![2, 17, 18, 19], 2, 16),
        (vec![20, 30, 10], vec![19, 13, 26, 4, 25, 11, 21], 2, 20),
    ];
    for (b, p, c, ex) in cases {
        assert_eq!(Solution::latest_time_catch_the_bus(b, p, c), ex);
    }
}
