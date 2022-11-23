#![allow(dead_code)]

// 374. Guess Number Higher or Lower
// https://leetcode.com/problems/guess-number-higher-or-lower/
//
// We are playing the Guess Game. The game is as follows:
//
// I pick a number from 1 to n. You have to guess which number I picked.
//
// Every time you guess wrong, I'll tell you whether the number is higher or lower.
//
// You call a pre-defined API int guess(int num), which returns 3 possible results:
//
// -1: The number I picked is lower than your guess (i.e. pick < num).
// 1: The number I picked is higher than your guess (i.e. pick > num).
// 0: The number I picked is equal to your guess (i.e. pick == num).
// Return the number that I picked.
//
// Example 1:
//
// Input: n = 10, pick = 6
// Output: 6
//
// Example 2:
//
// Input: n = 1, pick = 1
// Output: 1
//
// Example 3:
//
// Input: n = 2, pick = 1
// Output: 1
//
// Example 4:
//
// Input: n = 2, pick = 2
// Output: 2
//
// Constraints:
//
// 1 <= n <= 231 - 1
// 1 <= pick <= n
//

struct Solution;

impl Solution {
    pub fn guess_number(n: i32) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = n;
        let mut mid: i32;
        loop {
            mid = left + (right - left) / 2;

            if guess(mid) == 0 {
                break;
            } else if guess(mid) == -1 {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        mid
    }
}

static mut PICK: i32 = 6;

fn guess(num: i32) -> i32 {
    unsafe {
        match PICK.cmp(&num) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::guess_number(10), unsafe { PICK });
}
