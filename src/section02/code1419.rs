#![allow(dead_code)]

// 1419. Minimum Number of Frogs Croaking
// https://leetcode.com/problems/minimum-number-of-frogs-croaking/
// https://leetcode.cn/problems/minimum-number-of-frogs-croaking/
//
// Medium
//
// You are given the string croakOfFrogs, which represents a combination of the string "croak" from different frogs,
// that is, multiple frogs can croak at the same time, so multiple "croak" are mixed.
//
// Return the minimum number of different frogs to finish all the croaks in the given string.
//
// A valid "croak" means a frog is printing five letters 'c', 'r', 'o', 'a', and 'k' sequentially.
// The frogs have to print all five letters to finish a croak. If the given string is not a combination of a valid "croak" return -1.
//
// Example 1:
//
// Input: croakOfFrogs = "croakcroak"
// Output: 1
// Explanation: One frog yelling "croak" twice.
//
// Example 2:
//
// Input: croakOfFrogs = "crcoakroak"
// Output: 2
// Explanation: The minimum number of frogs is two.
// The first frog could yell "crcoakroak".
// The second frog could yell later "crcoakroak".
//
// Example 3:
//
// Input: croakOfFrogs = "croakcrook"
// Output: -1
// Explanation: The given string is an invalid combination of "croak" from different frogs.
//
// Constraints:
//
// -    1 <= croakOfFrogs.length <= 10^5
// -    croakOfFrogs is either 'c', 'r', 'o', 'a', or 'k'.
//

struct Solution;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        use std::collections::*;
        let mut ques = vec![VecDeque::new(); 5];
        let s = croak_of_frogs.chars().collect::<Vec<char>>();
        let n = s.len();

        if n % 5 != 0 {
            return -1;
        }

        for (i, &item) in s.iter().enumerate().take(n) {
            match item {
                'c' => ques[0].push_back(i),
                'r' => ques[1].push_back(i),
                'o' => ques[2].push_back(i),
                'a' => ques[3].push_back(i),
                _ => ques[4].push_back(i),
            }
        }

        let mut temp = vec![0; 5];
        let mut ranges = vec![];
        let mut li = -1;
        for i in 0..n {
            let ti = i % 5;
            if ti == 0 {
                temp = vec![0; 5];
            }
            if let Some(j) = ques[ti].pop_front() {
                let j = j as i32;
                if j <= li {
                    return -1;
                }
                temp[ti] = j as usize;
                li = j;
            } else {
                return -1;
            }

            if ti == 4 {
                ranges.push((temp[0], temp[4]));
                li = 0;
            }
        }

        let mut memo = vec![(0, 0); n];
        for (a, b) in ranges {
            memo[a].0 += 1;
            memo[b].1 += 1;
        }

        let mut result = 0;
        let mut temp = 0;

        for item in memo.iter().take(n) {
            temp += item.0;
            result = result.max(temp);
            temp -= item.1;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_number_of_frogs("croakcroak".to_string()), 1);
    assert_eq!(Solution::min_number_of_frogs("crcoakroak".to_string()), 2);
    assert_eq!(Solution::min_number_of_frogs("croakcrook".to_string()), -1);
    assert_eq!(Solution::min_number_of_frogs("croakcroa".to_string()), -1);
}
