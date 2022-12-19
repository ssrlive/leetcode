#![allow(dead_code)]

// 728. Self Dividing Numbers
// https://leetcode.com/problems/self-dividing-numbers/
// https://leetcode.cn/problems/self-dividing-numbers/
//
// A self-dividing number is a number that is divisible by every digit it contains.
//
// For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.
// A self-dividing number is not allowed to contain the digit zero.
//
// Given two integers left and right, return a list of all the self-dividing numbers in the range [left, right].
//
// Example 1:
//
// Input: left = 1, right = 22
// Output: [1,2,3,4,5,6,7,8,9,11,12,15,22]
//
// Example 2:
//
// Input: left = 47, right = 85
// Output: [48,55,66,77]
//
// Constraints:
//
// - 1 <= left <= right <= 10^4
//

struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = vec![];
        for i in left..=right {
            let mut j = i;
            let mut ok = true;
            while j > 0 {
                let d = j % 10;
                if d == 0 || i % d != 0 {
                    ok = false;
                    break;
                }
                j /= 10;
            }
            if ok {
                result.push(i);
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
    assert_eq!(Solution::self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
}
