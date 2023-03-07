#![allow(dead_code)]

/*

// 2327. Number of People Aware of a Secret
// https://leetcode.com/problems/number-of-people-aware-of-a-secret/
// https://leetcode.cn/problems/number-of-people-aware-of-a-secret/
//
// Medium
//
// On day 1, one person discovers a secret.

You are given an integer delay, which means that each person will share the secret with a new person every day, starting from delay days after discovering the secret. You are also given an integer forget, which means that each person will forget the secret forget days after discovering it. A person cannot share the secret on the same day they forgot it, or on any day afterwards.

Given an integer n, return the number of people who know the secret at the end of day n. Since the answer may be very large, return it modulo 109 + 7.

Example 1:

Input: n = 6, delay = 2, forget = 4
Output: 5
Explanation:
Day 1: Suppose the first person is named A. (1 person)
Day 2: A is the only person who knows the secret. (1 person)
Day 3: A shares the secret with a new person, B. (2 people)
Day 4: A shares the secret with a new person, C. (3 people)
Day 5: A forgets the secret, and B shares the secret with a new person, D. (3 people)
Day 6: B shares the secret with E, and C shares the secret with F. (5 people)

Example 2:

Input: n = 4, delay = 1, forget = 3
Output: 6
Explanation:
Day 1: The first person is named A. (1 person)
Day 2: A shares the secret with B. (2 people)
Day 3: A and B share the secret with 2 new people, C and D. (4 people)
Day 4: A forgets the secret. B, C, and D share the secret with 3 new people. (6 people)

Constraints:

    2 <= n <= 1000
    1 <= delay < forget <= n
*/

struct Solution;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let mut tot = 1i64;

        let mut p = vec![0i64; n as usize + 1];

        let mut sharable = 0i64;
        p[0] += 1;
        for d in delay..n {
            if d >= forget {
                sharable = (sharable + MOD - p[(d - forget) as usize]) % MOD;
                tot = (tot + MOD - p[(d - forget) as usize]) % MOD;
            }
            sharable = (sharable + p[(d - delay) as usize]) % MOD;
            p[d as usize] = sharable;
            tot = (tot + sharable) % MOD;
        }
        tot as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
    assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
}
