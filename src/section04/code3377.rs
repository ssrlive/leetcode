#![allow(dead_code)]

// 3377. Digit Operations to Make Two Integers Equal
// https://leetcode.com/problems/digit-operations-to-make-two-integers-equal/
// https://leetcode.cn/problems/digit-operations-to-make-two-integers-equal/
//
// Medium
//
// You are given two integers n and m that consist of the same number of digits.
//
// You can perform the following operations any number of times:
//
// Choose any digit from n that is not 9 and increase it by 1.
// Choose any digit from n that is not 0 and decrease it by 1.
// The integer n must not be a prime number at any point, including its original value and after each operation.
//
// The cost of a transformation is the sum of all values that n takes throughout the operations performed.
//
// Return the minimum cost to transform n into m. If it is impossible, return -1.
//
// A prime number is a natural number greater than 1 with only two factors, 1 and itself.
//
// Example 1:
//
// Input: n = 10, m = 12
//
// Output: 85
//
// Explanation:
//
// We perform the following operations:
//
// Increase the first digit, now n = 20.
// Increase the second digit, now n = 21.
// Increase the second digit, now n = 22.
// Decrease the first digit, now n = 12.
//
// Example 2:
//
// Input: n = 4, m = 8
//
// Output: -1
//
// Explanation:
//
// It is impossible to make n equal to m.
//
// Example 3:
//
// Input: n = 6, m = 2
//
// Output: -1
//
// Explanation:
//
// Since 2 is already a prime, we can't make n equal to m.
//
// Constraints:
//
// 1 <= n, m < 10^4
// n and m consist of the same number of digits.
//

struct Solution;

impl Solution {
    pub fn min_operations(n: i32, m: i32) -> i32 {
        fn sieve(maxi: i32) -> Vec<bool> {
            let mut prime = vec![true; (maxi + 1) as usize];
            prime[0] = false;
            prime[1] = false;
            let mut i = 2;
            while i * i <= maxi {
                if prime[i as usize] {
                    let mut j = i * i;
                    while j <= maxi {
                        prime[j as usize] = false;
                        j += i;
                    }
                }
                i += 1;
            }
            prime
        }

        let prime = sieve(9999);
        if prime[n as usize] || prime[m as usize] {
            return -1;
        }
        let mut mini = vec![i32::MAX as i64; 10000];
        mini[n as usize] = n as i64;
        let mut minh = std::collections::BinaryHeap::new();
        minh.push(std::cmp::Reverse((n as i64, n)));
        while let Some(std::cmp::Reverse((sum, curr))) = minh.pop() {
            if curr == m {
                return sum as i32;
            }
            let mut nums = curr.to_string().into_bytes();
            for i in 0..nums.len() {
                let a = nums[i];
                if a < b'9' {
                    nums[i] = a + 1;
                    let nxt = String::from_utf8(nums.clone()).unwrap().parse::<i32>().unwrap();
                    if !prime[nxt as usize] && sum + (nxt as i64) < mini[nxt as usize] {
                        mini[nxt as usize] = sum + nxt as i64;
                        minh.push(std::cmp::Reverse((sum + nxt as i64, nxt)));
                    }
                }
                if a > b'0' {
                    nums[i] = a - 1;
                    let nxt = String::from_utf8(nums.clone()).unwrap().parse::<i32>().unwrap();
                    if !prime[nxt as usize] && sum + (nxt as i64) < mini[nxt as usize] {
                        mini[nxt as usize] = sum + nxt as i64;
                        minh.push(std::cmp::Reverse((sum + nxt as i64, nxt)));
                    }
                }
                nums[i] = a;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations(10, 12), 85);
    assert_eq!(Solution::min_operations(4, 8), -1);
    assert_eq!(Solution::min_operations(6, 2), -1);
}
