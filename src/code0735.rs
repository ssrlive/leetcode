#![allow(dead_code)]

// 735. Asteroid Collision
// https://leetcode.com/problems/asteroid-collision/
//
// We are given an array asteroids of integers representing asteroids in a row.
//
// For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right,
// negative meaning left). Each asteroid moves at the same speed.
//
// Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one will explode.
// If both are the same size, both will explode. Two asteroids moving in the same direction will never meet.
//
// Example 1:
//
// Input: asteroids = [5,10,-5]
// Output: [5,10]
// Explanation: The 10 and -5 collide resulting in 10. The 5 and 10 never collide.
//
// Example 2:
//
// Input: asteroids = [8,-8]
// Output: []
// Explanation: The 8 and -8 collide exploding each other.
//
// Example 3:
//
// Input: asteroids = [10,2,-5]
// Output: [10]
// Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide resulting in 10.
//
// Constraints:
//
// - 2 <= asteroids.length <= 10^4
// - -1000 <= asteroids[i] <= 1000
// - asteroids[i] != 0
//

struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();
        for &asteroid in asteroids.iter() {
            answer.push(asteroid);
            while answer.len() > 1 && answer[answer.len() - 2] > 0 && answer[answer.len() - 1] < 0 {
                let l = answer.pop().unwrap();
                let r = answer.pop().unwrap();
                match r.cmp(&-l) {
                    std::cmp::Ordering::Less => answer.push(l),
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => answer.push(r),
                }
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
    assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    assert_eq!(Solution::asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
    assert_eq!(Solution::asteroid_collision(vec![-2, -2, 1, -2]), vec![-2, -2, -2]);
}
