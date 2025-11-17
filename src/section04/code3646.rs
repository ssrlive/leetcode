#![allow(dead_code)]

// 3646. Next Special Palindrome Number
// https://leetcode.com/problems/next-special-palindrome-number/
// https://leetcode.cn/problems/next-special-palindrome-number/
//
// Hard
//
// You are given an integer n.
//
// A number is called special if:
//
// It is a palindrome.
// Every digit k in the number appears exactly k times.
// Return the smallest special number strictly greater than n.
//
// Example 1:
//
// Input: n = 2
//
// Output: 22
//
// Explanation:
//
// 22 is the smallest special number greater than 2, as it is a palindrome and the digit 2 appears exactly 2 times.
//
// Example 2:
//
// Input: n = 33
//
// Output: 212
//
// Explanation:
//
// 212 is the smallest special number greater than 33, as it is a palindrome and the digits 1 and 2 appear exactly 1 and 2 times respectively.
//
// Constraints:
//
// 0 <= n <= 10^15

struct Solution;

impl Solution {
    pub fn special_palindrome(n: i64) -> i64 {
        fn get_bit(n: i32, pos: i32) -> bool {
            n & (1 << pos) != 0
        }

        fn generate_palindromes(candidates: &mut Vec<i64>, chosen_elements: &[i32]) {
            let mut half = Vec::new();
            let mut odd_element = -1;
            for &val in chosen_elements.iter() {
                if val % 2 == 1 {
                    odd_element = val;
                }
                let count = val / 2;
                for _ in 0..count {
                    half.push(val);
                }
            }
            let mut half_permutations = Vec::new();
            fn backtrack(half: &mut Vec<i32>, start: usize, half_permutations: &mut Vec<Vec<i32>>) {
                if start == half.len() {
                    half_permutations.push(half.clone());
                    return;
                }
                let mut seen = std::collections::HashSet::new();
                for i in start..half.len() {
                    if seen.insert(half[i]) {
                        half.swap(start, i);
                        backtrack(half, start + 1, half_permutations);
                        half.swap(start, i);
                    }
                }
            }
            backtrack(&mut half, 0, &mut half_permutations);
            for perm in half_permutations {
                let mut candidate = 0i64;
                for &val in perm.iter() {
                    candidate = 10 * candidate + val as i64;
                }
                if odd_element != -1 {
                    candidate = 10 * candidate + odd_element as i64;
                }
                for &val in perm.iter().rev() {
                    candidate = 10 * candidate + val as i64;
                }
                candidates.push(candidate);
            }
        }

        let mut candidates = Vec::new();
        let elements = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 0..(1 << 9) {
            let mut chosen_elements = Vec::new();
            for pos in 0..9 {
                if get_bit(i, pos) {
                    chosen_elements.push(elements[pos as usize]);
                }
            }
            let mut odd_count = 0;
            let mut total_length = 0;
            for &val in chosen_elements.iter() {
                if val % 2 == 1 {
                    odd_count += 1;
                }
                total_length += val;
            }
            if odd_count > 1 {
                continue;
            }
            if total_length >= 17 {
                continue;
            }
            generate_palindromes(&mut candidates, &chosen_elements);
        }
        candidates.sort_unstable();
        match candidates.iter().find(|&&x| x > n) {
            Some(&res) => res,
            None => -1,
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::special_palindrome(2), 22);
    assert_eq!(Solution::special_palindrome(33), 212);
}
