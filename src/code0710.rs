#![allow(dead_code)]

// 710. Random Pick with Blacklist
// https://leetcode.com/problems/random-pick-with-blacklist/
//
// You are given an integer n and an array of unique integers blacklist. Design an algorithm to pick a random integer
// in the range [0, n - 1] that is not in blacklist. Any integer that is in the mentioned range and not in blacklist
// should be equally likely to be returned.
//
// Optimize your algorithm such that it minimizes the number of calls to the built-in random function of your language.
//
// Implement the Solution class:
//
// Solution(int n, int[] blacklist) Initializes the object with the integer n and the blacklisted integers blacklist.
// int pick() Returns a random integer in the range [0, n - 1] and not in blacklist.
//
// Example 1:
//
// Input
// ["Solution", "pick", "pick", "pick", "pick", "pick", "pick", "pick"]
// [[7, [2, 3, 5]], [], [], [], [], [], [], []]
// Output
// [null, 0, 4, 1, 6, 1, 0, 4]
//
// Explanation
// Solution solution = new Solution(7, [2, 3, 5]);
// solution.pick(); // return 0, any integer from [0,1,4,6] should be ok. Note that for every call of pick,
//                  // 0, 1, 4, and 6 must be equally likely to be returned (i.e., with probability 1/4).
// solution.pick(); // return 4
// solution.pick(); // return 1
// solution.pick(); // return 6
// solution.pick(); // return 1
// solution.pick(); // return 0
// solution.pick(); // return 4
//
// Constraints:
//
// - 1 <= n <= 10^9
// - 0 <= blacklist.length <= min(10^5, n - 1)
// - 0 <= blacklist[i] < n
// - All the values of blacklist are unique.
// - At most 2 * 10^4 calls will be made to pick.
//

use rand::prelude::*;
use std::collections::HashMap;

struct Solution {
    sz: usize,
    mapping: HashMap<usize, usize>,
    black_list: Vec<usize>,
    last: usize,
    rng: ThreadRng,
}

impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let n = n as usize;
        let sz = n - blacklist.len();
        let mut black_list = blacklist.iter().map(|&x| x as usize).collect::<Vec<_>>();
        black_list.sort();

        let mut mapping = black_list.iter().map(|&v| (v, usize::MAX)).collect::<HashMap<_, _>>();

        let mut last = n - 1;
        for &i in black_list.iter() {
            if i >= sz {
                continue;
            }
            while mapping.contains_key(&last) {
                last -= 1;
            }
            mapping.insert(i, last);
            last -= 1;
        }
        Solution {
            sz,
            mapping,
            black_list,
            last,
            rng: thread_rng(),
        }
    }

    fn pick(&mut self) -> i32 {
        let picked = self.rng.gen_range(0..self.sz);
        if let Some(&v) = self.mapping.get(&picked) {
            return v as i32;
        }
        picked as i32
    }
}

#[test]
fn test() {
    let black_list = vec![2, 3, 5];
    let mut solution = Solution::new(7, black_list.clone());
    for _ in 0..100 {
        let picked = solution.pick();
        assert!(!black_list.contains(&picked));
    }
}
