#![allow(dead_code)]

/*

// 1916. Count Ways to Build Rooms in an Ant Colony
// https://leetcode.com/problems/count-ways-to-build-rooms-in-an-ant-colony/
// https://leetcode.cn/problems/count-ways-to-build-rooms-in-an-ant-colony/
//
// Hard
//
// You are an ant tasked with adding n new rooms numbered 0 to n-1 to your colony. You are given the expansion plan as a 0-indexed integer array of length n, prevRoom, where prevRoom[i] indicates that you must build room prevRoom[i] before building room i, and these two rooms must be connected directly. Room 0 is already built, so prevRoom[0] = -1. The expansion plan is given such that once all the rooms are built, every room will be reachable from room 0.

You can only build one room at a time, and you can travel freely between rooms you have already built only if they are connected. You can choose to build any room as long as its previous room is already built.

Return the number of different orders you can build all the rooms in. Since the answer may be large, return it modulo 109 + 7.

Example 1:

Input: prevRoom = [-1,0,1]
Output: 1
Explanation: There is only one way to build the additional rooms: 0 → 1 → 2

Example 2:

Input: prevRoom = [-1,0,0,1,2]
Output: 6
Explanation:
The 6 ways are:
0 → 1 → 3 → 2 → 4
0 → 2 → 4 → 1 → 3
0 → 1 → 2 → 3 → 4
0 → 1 → 2 → 4 → 3
0 → 2 → 1 → 3 → 4
0 → 2 → 1 → 4 → 3

Constraints:

    n == prevRoom.length
    2 <= n <= 10^5
    prevRoom[0] == -1
    0 <= prevRoom[i] < n for all 1 <= i < n
    Every room is reachable from room 0 once all the rooms are built.
*/

struct Solution;

const MOD: i64 = 1e9 as i64 + 7;

struct Combination {
    fact: Vec<i64>,
    infact: Vec<i64>,
}

impl Combination {
    const N: usize = 1e5 as usize + 10;
    fn qpow(mut x: i64, mut n: i64) -> i64 {
        let mut res = 1;
        while n > 0 {
            if n & 1 == 1 {
                res = res * x % MOD;
            }
            x = x * x % MOD;
            n >>= 1;
        }
        res
    }

    pub fn new() -> Self {
        let mut fact = vec![0; Self::N];
        let mut infact = vec![0; Self::N];
        fact[0] = 1;
        infact[0] = 1;
        for i in 1..Self::N {
            fact[i] = fact[i - 1] * i as i64 % MOD;
            infact[i] = infact[i - 1] * Self::qpow(i as i64, MOD - 2) % MOD;
        }
        Combination { fact, infact }
    }

    pub fn get(&self, a: i64, b: i64) -> i64 {
        let (a, b) = (a as usize, b as usize);
        self.fact[a] * self.infact[a - b] % MOD * self.infact[b] % MOD
    }
}

impl Solution {
    fn solve(u: i64, edges: &Vec<Vec<i64>>, combination: &Combination) -> (i64, i64) {
        let mut ul: i64 = 0;
        let mut uw: i64 = 1;
        for v in edges[u as usize].iter() {
            let (vl, vw) = Self::solve(*v, edges, combination);
            ul += vl;
            if uw == 0 {
                uw = vw;
            } else {
                uw = uw * vw % MOD * combination.get(ul, vl) % MOD;
            }
        }
        ul += 1;
        (ul, uw)
    }

    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        let combination = Combination::new();
        let n = prev_room.len();
        let mut edges = vec![vec![]; n];
        for i in 1..n {
            edges[prev_room[i] as usize].push(i as i64);
        }
        let (_, rw) = Self::solve(0, &edges, &combination);
        rw as i32
    }
}

#[test]
fn test() {
    let prev_room = vec![-1, 0, 1];
    let res = Solution::ways_to_build_rooms(prev_room);
    assert_eq!(res, 1);

    let prev_room = vec![-1, 0, 0, 1, 2];
    let res = Solution::ways_to_build_rooms(prev_room);
    assert_eq!(res, 6);
}
