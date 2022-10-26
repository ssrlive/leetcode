#![allow(dead_code)]

// 204. Count Primes
// https://leetcode.com/problems/count-primes/
//
// Given an integer n, return the number of prime numbers that are strictly less than n.
//

struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        // If an index in vector is marked false it's a prime.
        //   Whenever a number is encountered, whose index value is false,
        //  it's index in vector is set to true and the counter is incremented.
        let mut is_multiple_of_prime = vec![false; n as usize];

        let mut count = 0;

        for i in 2..n {
            if !is_multiple_of_prime[i as usize] {
                count += 1;

                let mut j = i;

                //Then this loops all multiples of this index are marked as true.
                // ie 3+3 = 6, 3+3+3 =9, and so on
                while j < n {
                    is_multiple_of_prime[j as usize] = true;
                    j += i;
                }
            }
        }

        count
    }
}

#[test]
fn test_count_primes() {
    assert_eq!(Solution::count_primes(10), 4);
    assert_eq!(Solution::count_primes(0), 0);
    assert_eq!(Solution::count_primes(1), 0);
}
