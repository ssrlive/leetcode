#![allow(dead_code)]

// 825. Friends Of Appropriate Ages
// https://leetcode.com/problems/friends-of-appropriate-ages/
//
// There are n persons on a social media website. You are given an integer array ages where ages[i] is the age of the ith person.
//
// A Person x will not send a friend request to a person y (x != y) if any of the following conditions is true:
//
// age[y] <= 0.5 * age[x] + 7
// age[y] > age[x]
// age[y] > 100 && age[x] < 100
// Otherwise, x will send a friend request to y.
//
// Note that if x sends a request to y, y will not necessarily send a request to x. Also, a person will not send a friend request to themself.
//
// Return the total number of friend requests made.
//
// Example 1:
//
// Input: ages = [16,16]
// Output: 2
// Explanation: 2 people friend request each other.
//
// Example 2:
//
// Input: ages = [16,17,18]
// Output: 2
// Explanation: Friend requests are made 17 -> 16, 18 -> 17.
//
// Example 3:
//
// Input: ages = [20,30,100,110,120]
// Output: 3
// Explanation: Friend requests are made 110 -> 100, 120 -> 110, 120 -> 100.
//
// Constraints:
//
// - n == ages.length
// - 1 <= n <= 2 * 10^4
// - 1 <= ages[i] <= 120
//

struct Solution;

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut count = vec![0; 121];
        for age in ages {
            count[age as usize] += 1;
        }
        let mut result = 0;
        for age in 1..=120 {
            if count[age] == 0 {
                continue;
            }
            for other in (age / 2 + 8)..=age {
                if count[other] == 0 {
                    continue;
                }
                if age == other {
                    result += count[age] * (count[age] - 1);
                } else {
                    result += count[age] * count[other];
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_friend_requests(vec![16, 16]), 2);
    assert_eq!(Solution::num_friend_requests(vec![16, 17, 18]), 2);
    assert_eq!(Solution::num_friend_requests(vec![20, 30, 100, 110, 120]), 3);
}
