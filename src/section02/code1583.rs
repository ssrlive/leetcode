#![allow(dead_code)]

/*

// 1583. Count Unhappy Friends
Medium
231
748
Companies

You are given a list of preferences for n friends, where n is always even.

For each person i, preferences[i] contains a list of friends sorted in the order of preference. In other words, a friend earlier in the list is more preferred than a friend later in the list. Friends in each list are denoted by integers from 0 to n-1.

All the friends are divided into pairs. The pairings are given in a list pairs, where pairs[i] = [xi, yi] denotes xi is paired with yi and yi is paired with xi.

However, this pairing may cause some of the friends to be unhappy. A friend x is unhappy if x is paired with y and there exists a friend u who is paired with v but:

    x prefers u over y, and
    u prefers x over v.

Return the number of unhappy friends.

Example 1:

Input: n = 4, preferences = [[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]], pairs = [[0, 1], [2, 3]]
Output: 2
Explanation:
Friend 1 is unhappy because:
- 1 is paired with 0 but prefers 3 over 0, and
- 3 prefers 1 over 2.
Friend 3 is unhappy because:
- 3 is paired with 2 but prefers 1 over 2, and
- 1 prefers 3 over 0.
Friends 0 and 2 are happy.

Example 2:

Input: n = 2, preferences = [[1], [0]], pairs = [[1, 0]]
Output: 0
Explanation: Both friends 0 and 1 are happy.

Example 3:

Input: n = 4, preferences = [[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]], pairs = [[1, 3], [0, 2]]
Output: 4

Constraints:

    2 <= n <= 500
    n is even.
    preferences.length == n
    preferences[i].length == n - 1
    0 <= preferences[i][j] <= n - 1
    preferences[i] does not contain i.
    All values in preferences[i] are unique.
    pairs.length == n/2
    pairs[i].length == 2
    xi != yi
    0 <= xi, yi <= n - 1
    Each person is contained in exactly one pair.
*/

struct Solution;

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut memo = vec![0; n];
        for arr in &pairs {
            let a = arr[0] as usize;
            let b = arr[1] as usize;
            memo[a] = b;
            memo[b] = a;
        }

        let mut result = 0;
        for arr in pairs {
            let a = arr[0] as usize;
            let b = arr[1] as usize;

            let mut flag = false;
            for &u in &preferences[a] {
                let u = u as usize;
                if u == b {
                    break;
                }

                let v = memo[u];
                for &candidate in &preferences[u] {
                    let candidate = candidate as usize;
                    if candidate == a {
                        flag = true;
                        break;
                    } else if candidate == v {
                        break;
                    }
                }

                if flag {
                    result += 1;
                    break;
                }
            }

            let mut flag = false;
            for &u in &preferences[b] {
                let u = u as usize;
                if u == a {
                    break;
                }

                let v = memo[u];
                for &candidate in &preferences[u] {
                    let candidate = candidate as usize;
                    if candidate == b {
                        flag = true;
                        break;
                    } else if candidate == v {
                        break;
                    }
                }

                if flag {
                    result += 1;
                    break;
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            4,
            vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![1, 2, 0]],
            vec![vec![0, 1], vec![2, 3]],
            2,
        ),
        (2, vec![vec![1], vec![0]], vec![vec![1, 0]], 0),
        (
            4,
            vec![vec![1, 3, 2], vec![2, 3, 0], vec![1, 3, 0], vec![0, 2, 1]],
            vec![vec![1, 3], vec![0, 2]],
            4,
        ),
    ];
    for (n, p, pairs, e) in cases {
        assert_eq!(Solution::unhappy_friends(n, p, pairs), e);
    }
}
