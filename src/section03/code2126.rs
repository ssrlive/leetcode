#![allow(dead_code)]

/*

// 2126. Destroying Asteroids
// https://leetcode.com/problems/destroying-asteroids/
// https://leetcode.cn/problems/destroying-asteroids/
//
// Medium
//
// You are given an integer mass, which represents the original mass of a planet. You are further given an integer array asteroids, where asteroids[i] is the mass of the ith asteroid.

You can arrange for the planet to collide with the asteroids in any arbitrary order. If the mass of the planet is greater than or equal to the mass of the asteroid, the asteroid is destroyed and the planet gains the mass of the asteroid. Otherwise, the planet is destroyed.

Return true if all asteroids can be destroyed. Otherwise, return false.

Example 1:

Input: mass = 10, asteroids = [3,9,19,5,21]
Output: true
Explanation: One way to order the asteroids is [9,19,5,3,21]:
- The planet collides with the asteroid with a mass of 9. New planet mass: 10 + 9 = 19
- The planet collides with the asteroid with a mass of 19. New planet mass: 19 + 19 = 38
- The planet collides with the asteroid with a mass of 5. New planet mass: 38 + 5 = 43
- The planet collides with the asteroid with a mass of 3. New planet mass: 43 + 3 = 46
- The planet collides with the asteroid with a mass of 21. New planet mass: 46 + 21 = 67
All asteroids are destroyed.

Example 2:

Input: mass = 5, asteroids = [4,9,23,4]
Output: false
Explanation:
The planet cannot ever gain enough mass to destroy the asteroid with a mass of 23.
After the planet destroys the other asteroids, it will have a mass of 5 + 4 + 9 + 4 = 22.
This is less than 23, so a collision would not destroy the last asteroid.

Constraints:

    1 <= mass <= 10^5
    1 <= asteroids.length <= 10^5
    1 <= asteroids[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut asteroids = asteroids;
        asteroids.sort();
        let mut m: i64 = mass as i64;
        for a in asteroids {
            let b = a as i64;
            if b <= m {
                m += b;
            } else {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
    assert!(!Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]));
}
