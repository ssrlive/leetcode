#![allow(dead_code)]

// 990. Satisfiability of Equality Equations
// https://leetcode.com/problems/satisfiability-of-equality-equations/
// https://leetcode.cn/problems/satisfiability-of-equality-equations/
//
// You are given an array of strings equations that represent relationships between variables where each string equations[i] is of length 4 and takes one of two different forms: "xi==yi" or "xi!=yi".Here, xi and yi are lowercase letters (not necessarily different) that represent one-letter variable names.
//
// Return true if it is possible to assign integers to variable names so as to satisfy all the given equations, or false otherwise.
//
// Example 1:
//
// Input: equations = ["a==b","b!=a"]
// Output: false
// Explanation: If we assign say, a = 1 and b = 1, then the first equation is satisfied, but not the second.
// There is no way to assign the variables to satisfy both equations.
//
// Example 2:
//
// Input: equations = ["b==a","a==b"]
// Output: true
// Explanation: We could assign a = 1 and b = 1 to satisfy both equations.
//
// Constraints:
//
// - 1 <= equations.length <= 500
// - equations[i].length == 4
// - equations[i][0] is a lowercase letter.
// - equations[i][1] is either '=' or '!'.
// - equations[i][2] is '='.
// - equations[i][3] is a lowercase letter.
//

struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        pub struct Union {
            pub conns: Vec<usize>,
        }

        impl Union {
            pub fn new(size: usize) -> Self {
                let mut conns = vec![0; size];
                for (i, conn) in conns.iter_mut().enumerate() {
                    *conn = i;
                }
                Self { conns }
            }

            pub fn union(&mut self, a: usize, b: usize) {
                let temp = self.conns[a];
                for i in 0..self.conns.len() {
                    if self.conns[i] == temp {
                        self.conns[i] = self.conns[b];
                    }
                }
            }

            pub fn find(&mut self, a: usize, b: usize) -> bool {
                self.conns[a] == self.conns[b]
            }
        }

        let index = |ch: u8| (ch - b'a') as usize;
        let mut conns = Union::new(26);

        for equation in &equations {
            let chars = equation.as_bytes();
            if chars[1] == b'=' {
                let (a, b) = (index(chars[0]), index(chars[3]));
                conns.union(a, b);
            }
        }

        for equation in &equations {
            let chars = equation.as_bytes();
            let (a, b) = (index(chars[0]), index(chars[3]));
            if chars[1] == b'!' && conns.find(a, b) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["a==b", "b!=a"], false),
        (vec!["b==a", "a==b"], true),
        (vec!["a==b", "b==c", "a==c"], true),
        (vec!["a==b", "b!=c", "c==a"], false),
        (vec!["c==c", "b==d", "x!=z"], true),
    ];
    for (equations, expected) in cases {
        let equations = equations.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::equations_possible(equations), expected);
    }
}
