#![allow(dead_code)]

// 3376. Minimum Time to Break Locks I
// https://leetcode.com/problems/minimum-time-to-break-locks-i/
// https://leetcode.cn/problems/minimum-time-to-break-locks-i/
//
// Medium
//
// Bob is stuck in a dungeon and must break n locks, each requiring some amount of energy to break.
// The required energy for each lock is stored in an array called strength
// where strength[i] indicates the energy needed to break the ith lock.
//
// To break a lock, Bob uses a sword with the following characteristics:
//
// The initial energy of the sword is 0.
// The initial factor X by which the energy of the sword increases is 1.
// Every minute, the energy of the sword increases by the current factor X.
// To break the ith lock, the energy of the sword must reach at least strength[i].
// After breaking a lock, the energy of the sword resets to 0, and the factor X increases by a given value K.
// Your task is to determine the minimum time in minutes required for Bob to break all n locks and escape the dungeon.
//
// Return the minimum time required for Bob to break all n locks.
//
// Example 1:
//
// Input: strength = [3,4,1], K = 1
//
// Output: 4
//
// Explanation:
//
// Time	Energy	X	Action	Updated X
// 0	0	1	Nothing	1
// 1	1	1	Break 3rd Lock	2
// 2	2	2	Nothing	2
// 3	4	2	Break 2nd Lock	3
// 4	3	3	Break 1st Lock	3
// The locks cannot be broken in less than 4 minutes; thus, the answer is 4.
//
// Example 2:
//
// Input: strength = [2,5,4], K = 2
//
// Output: 5
//
// Explanation:
//
// Time	Energy	X	Action	Updated X
// 0	0	1	Nothing	1
// 1	1	1	Nothing	1
// 2	2	1	Break 1st Lock	3
// 3	3	3	Nothing	3
// 4	6	3	Break 2nd Lock	5
// 5	5	5	Break 3rd Lock	7
// The locks cannot be broken in less than 5 minutes; thus, the answer is 5.
//
// Constraints:
//
// n == strength.length
// 1 <= n <= 8
// 1 <= K <= 10
// 1 <= strength[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
        let mut strength = strength;
        strength.sort();
        let mut res = i32::MAX;
        let size = strength.len() as i32;
        let mut permutation = vec![0; size as usize];
        for i in 0..size {
            permutation[i as usize] = i;
        }
        loop {
            let mut time = 0;
            let mut x = 1;
            for i in 0..size {
                let s = strength[permutation[i as usize] as usize];
                time += (s + x - 1) / x;
                x += k;
            }
            res = res.min(time);
            if !permutation.next_permutation() {
                break;
            }
        }
        res
    }
}

pub trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T: std::cmp::PartialOrd> NextPermutation for Vec<T> {
    fn next_permutation(&mut self) -> bool {
        NextPermutation::next_permutation(&mut **self)
    }
}

impl<T: std::cmp::PartialOrd> NextPermutation for [T] {
    fn next_permutation(&mut self) -> bool {
        let n = self.len();
        if n < 2 {
            return false;
        }
        let mut i = n - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            self.reverse();
            return false;
        }
        let mut j = n - 1;
        while self[j] <= self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        self[i..].reverse();
        true
    }
}

#[test]
fn test() {
    let strength = vec![3, 4, 1];
    let k = 1;
    let res = 4;
    assert_eq!(Solution::find_minimum_time(strength, k), res);

    let strength = vec![2, 5, 4];
    let k = 2;
    let res = 5;
    assert_eq!(Solution::find_minimum_time(strength, k), res);
}
