#![allow(dead_code)]

// 1733. Minimum Number of People to Teach
// https://leetcode.com/problems/minimum-number-of-people-to-teach/
// https://leetcode.cn/problems/minimum-number-of-people-to-teach/
//
// Medium
//
// On a social network consisting of m users and some friendships between users, two users can communicate with each other if they know a common language.
//
// You are given an integer n, an array languages, and an array friendships where:
//
//     There are n languages numbered 1 through n,
//     languages[i] is the set of languages the i​​​​​​th​​​​ user knows, and
//     friendships[i] = [u​​​​​​i​​​, v​​​​​​i] denotes a friendship between the users u​​​​​​​​​​​i​​​​​ and vi.
//
// You can choose one language and teach it to some users so that all friends can communicate with each other. Return the minimum number of users you need to teach.
// Note that friendships are not transitive, meaning if x is a friend of y and y is a friend of z, this doesn't guarantee that x is a friend of z.
//
// Example 1:
//
// Input: n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
// Output: 1
// Explanation: You can either teach user 1 the second language or user 2 the first language.
//
// Example 2:
//
// Input: n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
// Output: 2
// Explanation: Teach the third language to users 1 and 3, yielding two users to teach.
//
// Constraints:
//
// -    2 <= n <= 500
// -    languages.length == m
// -    1 <= m <= 500
// -    1 <= languages[i].length <= n
// -    1 <= languages[i][j] <= n
// -    1 <= u​​​​​​i < v​​​​​​i <= languages.length
// -    1 <= friendships.length <= 500
// -    All tuples (u​​​​​i, v​​​​​​i) are unique
// -    languages[i] contains only unique values

struct Solution;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        fn check(a: &[Vec<i32>], u: usize, v: usize, mem: &mut [Vec<i32>]) -> bool {
            if mem[u][v] != 0 {
                return mem[u][v] == 1;
            }
            for i in 0..a[v].len() {
                if a[u].binary_search(&a[v][i]).is_ok() {
                    mem[v][u] = 1;
                    mem[u][v] = 1;
                    return true;
                }
            }
            mem[u][v] = -1;
            mem[v][u] = -1;
            false
        }

        let mut languages = languages;
        let mut mem = vec![vec![0; languages.len() + 1]; languages.len() + 1];
        let mut ret = languages.len() as i32;
        for language in languages.iter_mut() {
            language.sort();
        }
        for lang in 1..=n {
            let mut count = 0;
            let mut teach = vec![false; languages.len()];
            for friendship in friendships.iter() {
                let u = friendship[0] as usize - 1;
                let v = friendship[1] as usize - 1;
                if check(&languages, u, v, &mut mem) {
                    continue;
                }
                if languages[u].binary_search(&lang).is_err() {
                    if !teach[u] {
                        count += 1;
                    }
                    teach[u] = true;
                }
                if languages[v].binary_search(&lang).is_err() {
                    if !teach[v] {
                        count += 1;
                    }
                    teach[v] = true;
                }
            }
            ret = ret.min(count);
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, vec![vec![1], vec![2], vec![1, 2]], vec![vec![1, 2], vec![1, 3], vec![2, 3]], 1),
        (
            3,
            vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
            vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]],
            2,
        ),
    ];
    for (n, languages, friendships, expected) in cases {
        assert_eq!(Solution::minimum_teachings(n, languages, friendships), expected);
    }
}
