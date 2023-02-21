#![allow(dead_code)]

/*

// 1776. Car Fleet II
Hard
744
21
Companies

There are n cars traveling at different speeds in the same direction along a one-lane road. You are given an array cars of length n, where cars[i] = [positioni, speedi] represents:

    positioni is the distance between the ith car and the beginning of the road in meters. It is guaranteed that positioni < positioni+1.
    speedi is the initial speed of the ith car in meters per second.

For simplicity, cars can be considered as points moving along the number line. Two cars collide when they occupy the same position. Once a car collides with another car, they unite and form a single car fleet. The cars in the formed fleet will have the same position and the same speed, which is the initial speed of the slowest car in the fleet.

Return an array answer, where answer[i] is the time, in seconds, at which the ith car collides with the next car, or -1 if the car does not collide with the next car. Answers within 10-5 of the actual answers are accepted.

Example 1:

Input: cars = [[1,2],[2,1],[4,3],[7,2]]
Output: [1.00000,-1.00000,3.00000,-1.00000]
Explanation: After exactly one second, the first car will collide with the second car, and form a car fleet with speed 1 m/s. After exactly 3 seconds, the third car will collide with the fourth car, and form a car fleet with speed 2 m/s.

Example 2:

Input: cars = [[3,4],[5,4],[6,3],[9,1]]
Output: [2.00000,1.00000,1.50000,-1.00000]

Constraints:

    1 <= cars.length <= 10^5
    1 <= positioni, speedi <= 10^6
    positioni < position[i+1]
*/

struct Solution;

impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        pub fn collision_time(cars: &[Vec<i32>], curr: usize, next: usize) -> f64 {
            (cars[next][0] - cars[curr][0]) as f64 / (cars[curr][1] - cars[next][1]) as f64
        }

        let mut stack: Vec<usize> = Vec::new();
        let mut v = vec![-1.0; cars.len()];
        for i in (0..cars.len()).rev() {
            while !stack.is_empty()
                && (cars[i][1] <= cars[stack[stack.len() - 1]][1]
                    || (stack.len() > 1
                        && collision_time(&cars, i, stack[stack.len() - 1]) >= v[stack[stack.len() - 1]]))
            {
                stack.pop();
            }
            let time = if stack.is_empty() {
                -1.0
            } else {
                collision_time(&cars, i, stack[stack.len() - 1])
            };
            v[i] = time;
            stack.push(i);
        }
        v
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]],
            vec![1.00000, -1.00000, 3.00000, -1.00000],
        ),
        (
            vec![vec![3, 4], vec![5, 4], vec![6, 3], vec![9, 1]],
            vec![2.00000, 1.00000, 1.50000, -1.00000],
        ),
    ];
    for (cars, expected) in cases {
        assert_eq!(Solution::get_collision_times(cars), expected);
    }
}
