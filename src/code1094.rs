#![allow(dead_code)]

// 1094. Car Pooling
// https://leetcode.com/problems/car-pooling/
// https://leetcode.cn/problems/car-pooling/
//
// There is a car with capacity empty seats. The vehicle only drives east (i.e., it cannot turn around and drive west).
//
// You are given the integer capacity and an array trips where trips[i] = [numPassengersi, fromi, toi] indicates that the
// ith trip has numPassengersi passengers and the locations to pick them up and drop them off are fromi and toi respectively.
// The locations are given as the number of kilometers due east from the car's initial location.
//
// Return true if it is possible to pick up and drop off all passengers for all the given trips, or false otherwise.
//
// Example 1:
//
// Input: trips = [[2,1,5],[3,3,7]], capacity = 4
// Output: false
//
// Example 2:
//
// Input: trips = [[2,1,5],[3,3,7]], capacity = 5
// Output: true
//
// Constraints:
//
// - 1 <= trips.length <= 1000
// - trips[i].length == 3
// - 1 <= numPassengersi <= 100
// - 0 <= fromi < toi <= 1000
// - 1 <= capacity <= 10^5
//

struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut pick = vec![0; 1001];
        let mut drop = vec![0; 1001];

        for trip in trips {
            pick[trip[1] as usize] += trip[0];
            drop[trip[2] as usize] += trip[0];
        }

        let mut capacity = capacity;

        for i in 0usize..=1000 {
            capacity += drop[i];
            capacity -= pick[i];

            if capacity < 0 {
                return false;
            }
        }

        true
    }
}

#[test]
fn test() {
    let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
    let capacity = 4;
    assert_eq!(Solution::car_pooling(trips, capacity), false);

    let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
    let capacity = 5;
    assert_eq!(Solution::car_pooling(trips, capacity), true);
}
