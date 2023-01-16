#![allow(dead_code)]

// 989. Add to Array-Form of Integer
// https://leetcode.com/problems/add-to-array-form-of-integer/
// https://leetcode.cn/problems/add-to-array-form-of-integer/
//
// The array-form of an integer num is an array representing its digits in left to right order.
//
// For example, for num = 1321, the array form is [1,3,2,1].
// Given num, the array-form of an integer, and an integer k, return the array-form of the integer num + k.
//
// Example 1:
//
// Input: num = [1,2,0,0], k = 34
// Output: [1,2,3,4]
// Explanation: 1200 + 34 = 1234
//
// Example 2:
//
// Input: num = [2,7,4], k = 181
// Output: [4,5,5]
// Explanation: 274 + 181 = 455
//
// Example 3:
//
// Input: num = [2,1,5], k = 806
// Output: [1,0,2,1]
// Explanation: 215 + 806 = 1021
//
// Constraints:
//
// - 1 <= num.length <= 10^4
// - 0 <= num[i] <= 9
// - num does not contain any leading zeros except for the zero itself.
// - 1 <= k <= 10^4
//

struct Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        use std::iter::{from_fn, repeat};

        let mut k = k;
        let mut q = std::collections::VecDeque::with_capacity(num.len());
        let it_num = num.into_iter().rev();
        let it_k = from_fn(move || {
            if k > 0 {
                let dig = k % 10;
                k /= 10;
                Some(dig)
            } else {
                None
            }
        });

        let mut carry = 0;

        for (dig_num, dig_k) in it_num.chain(repeat(-1)).zip(it_k.chain(repeat(-1))) {
            if dig_num == -1 && dig_k == -1 {
                break;
            }
            let sum_dig = dig_num.max(0) + dig_k.max(0) + carry;
            q.push_front(sum_dig % 10);
            carry = i32::from(sum_dig >= 10);
        }

        if carry > 0 {
            q.push_front(1);
        }

        q.into_iter().collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 0, 0], 34, vec![1, 2, 3, 4]),
        (vec![2, 7, 4], 181, vec![4, 5, 5]),
        (vec![2, 1, 5], 806, vec![1, 0, 2, 1]),
        (
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            1,
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ),
        (vec![0], 23, vec![2, 3]),
    ];

    for (num, k, expect) in cases {
        assert_eq!(Solution::add_to_array_form(num, k), expect);
    }
}
