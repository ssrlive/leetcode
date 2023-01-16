#![allow(dead_code)]

// 881. Boats to Save People
// https://leetcode.com/problems/boats-to-save-people/
// https://leetcode.cn/problems/boats-to-save-people/
//
// You are given an array people where people[i] is the weight of the ith person, and an infinite number of boats
// where each boat can carry a maximum weight of limit. Each boat carries at most two people at the same time,
// provided the sum of the weight of those people is at most limit.
//
// Return the minimum number of boats to carry every given person.
//
// Example 1:
//
// Input: people = [1,2], limit = 3
// Output: 1
// Explanation: 1 boat (1, 2)
//
// Example 2:
//
// Input: people = [3,2,2,1], limit = 3
// Output: 3
// Explanation: 3 boats (1, 2), (2) and (3)
//
// Example 3:
//
// Input: people = [3,5,3,4], limit = 5
// Output: 4
// Explanation: 4 boats (3), (3), (4), (5)
//
// Constraints:
//
// - 1 <= people.length <= 5 * 10^4
// - 1 <= people[i] <= limit <= 3 * 10^4
//

struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let (mut l, mut r) = (0, people.len() - 1);
        let mut answer = 0;
        while l < r {
            if people[l] + people[r] <= limit {
                l += 1;
            }
            r -= 1;
            answer += 1;
        }
        answer + i32::from(l == r)
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2], 3, 1), (vec![3, 2, 2, 1], 3, 3), (vec![3, 5, 3, 4], 5, 4)];
    for (people, limit, answer) in cases {
        assert_eq!(Solution::num_rescue_boats(people, limit), answer);
    }
}
