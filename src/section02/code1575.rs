#![allow(dead_code)]

/*

// 1575. Count All Possible Routes
Hard
465
26
Companies

You are given an array of distinct positive integers locations where locations[i] represents the position of city i. You are also given integers start, finish and fuel representing the starting city, ending city, and the initial amount of fuel you have, respectively.

At each step, if you are at city i, you can pick any city j such that j != i and 0 <= j < locations.length and move to city j. Moving from city i to city j reduces the amount of fuel you have by |locations[i] - locations[j]|. Please notice that |x| denotes the absolute value of x.

Notice that fuel cannot become negative at any point in time, and that you are allowed to visit any city more than once (including start and finish).

Return the count of all possible routes from start to finish. Since the answer may be too large, return it modulo 109 + 7.

Example 1:

Input: locations = [2,3,6,8,4], start = 1, finish = 3, fuel = 5
Output: 4
Explanation: The following are all possible routes, each uses 5 units of fuel:
1 -> 3
1 -> 2 -> 3
1 -> 4 -> 3
1 -> 4 -> 2 -> 3

Example 2:

Input: locations = [4,3,1], start = 1, finish = 0, fuel = 6
Output: 5
Explanation: The following are all possible routes:
1 -> 0, used fuel = 1
1 -> 2 -> 0, used fuel = 5
1 -> 2 -> 1 -> 0, used fuel = 5
1 -> 0 -> 1 -> 0, used fuel = 3
1 -> 0 -> 1 -> 0 -> 1 -> 0, used fuel = 5

Example 3:

Input: locations = [5,2,1], start = 0, finish = 2, fuel = 3
Output: 0
Explanation: It is impossible to get from 0 to 2 using only 3 units of fuel since the shortest route needs 4 units of fuel.

Constraints:

    2 <= locations.length <= 100
    1 <= locations[i] <= 10^9
    All integers in locations are distinct.
    0 <= start, finish < locations.length
    1 <= fuel <= 200
*/

struct Solution;

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let locations = locations.iter().map(|&x| x as i64).collect();
        let mut dp = vec![vec![-1; 201]; 101];
        Self::helper(
            &locations,
            locations.len(),
            start as usize,
            finish as usize,
            fuel as i64,
            &mut dp,
        ) as _
    }

    fn helper(locations: &Vec<i64>, n: usize, s: usize, e: usize, f: i64, dp: &mut Vec<Vec<i64>>) -> i64 {
        let m = 1_000_000_007;
        if dp[s][f as usize] != -1 {
            return dp[s][f as usize];
        }
        let mut ans = 0;
        if s == e {
            ans += 1;
        }
        for i in 0..n {
            if i != s && (locations[s] - locations[i]).abs() <= f {
                let v = Self::helper(locations, n, i, e, f - (locations[s] - locations[i]).abs(), dp);
                ans = (ans + v % m) % m;
            }
        }
        let v = ans % m;
        dp[s][f as usize] = v;
        v
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 3, 6, 8, 4], 1, 3, 5, 4),
        (vec![4, 3, 1], 1, 0, 6, 5),
        (vec![5, 2, 1], 0, 2, 3, 0),
        (vec![2, 1, 5], 0, 0, 3, 2),
    ];
    for (locations, start, finish, fuel, expected) in cases {
        assert_eq!(Solution::count_routes(locations, start, finish, fuel), expected);
    }
}
