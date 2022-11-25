#![allow(dead_code)]

// 386. Lexicographical Numbers
// https://leetcode.com/problems/lexicographical-numbers/
//
// Given an integer n, return 1 - n in lexicographical order.
//
// Example 1:
//
// Input: n = 13
// Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
//
// Example 2:
//
// Input: n = 2
// Output: [1,2]
//
// Constraints:
//
// 1 <= n <= 5 * 10^4

struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut set = std::collections::BTreeSet::new();
        for i in 1..=n {
            set.insert(i.to_string());
        }
        set.iter().map(|x| x.parse::<i32>().unwrap()).collect()
    }

    pub fn lexical_order2(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = 1;
        for _ in 0..n {
            result.push(current);
            if current * 10 <= n {
                current *= 10;
            } else if current % 10 != 9 && current < n {
                current += 1;
            } else {
                while (current / 10) % 10 == 9 {
                    current /= 10;
                }
                current = current / 10 + 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::lexical_order(13),
        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    assert_eq!(Solution::lexical_order(2), vec![1, 2]);
}
