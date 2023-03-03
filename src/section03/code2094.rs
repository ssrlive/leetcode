#![allow(dead_code)]

/*

// 2094. Finding 3-Digit Even Numbers
// https://leetcode.com/problems/finding-3-digit-even-numbers/
// https://leetcode.cn/problems/finding-3-digit-even-numbers/
//
// Easy
//
// You are given an integer array digits, where each element is a digit. The array may contain duplicates.

You need to find all the unique integers that follow the given requirements:

    The integer consists of the concatenation of three elements from digits in any arbitrary order.
    The integer does not have leading zeros.
    The integer is even.

For example, if the given digits were [1, 2, 3], integers 132 and 312 follow the requirements.

Return a sorted array of the unique integers.

Example 1:

Input: digits = [2,1,3,0]
Output: [102,120,130,132,210,230,302,310,312,320]
Explanation: All the possible integers that follow the requirements are in the output array.
Notice that there are no odd integers or integers with leading zeros.

Example 2:

Input: digits = [2,2,8,8,2]
Output: [222,228,282,288,822,828,882]
Explanation: The same digit can be used as many times as it appears in digits.
In this example, the digit 8 is used twice each time in 288, 828, and 882.

Example 3:

Input: digits = [3,7,5]
Output: []
Explanation: No even integers can be formed using the given digits.

Constraints:

    3 <= digits.length <= 100
    0 <= digits[i] <= 9
*/

struct Solution;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut digits = digits;
        digits.sort_unstable();
        let mut prev_x = -1;
        for (i, &x) in digits.iter().enumerate().filter(|(_, &x)| x % 2 == 0) {
            match x == prev_x {
                true => continue,
                false => prev_x = x,
            }
            let mut prev_y = -1;
            for (j, &y) in digits.iter().enumerate().filter(|(j, _)| *j != i) {
                match y == prev_y {
                    true => continue,
                    false => prev_y = y,
                }
                let mut prev_z = -1;
                for (_, &z) in digits.iter().enumerate().filter(|&(k, z)| k != i && k != j && *z > 0) {
                    match z == prev_z {
                        true => continue,
                        false => {
                            res.push(x + 10 * y + 100 * z);
                            prev_z = z;
                        }
                    }
                }
            }
        }
        res.sort_unstable();
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 1, 3, 0], vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320]),
        (vec![2, 2, 8, 8, 2], vec![222, 228, 282, 288, 822, 828, 882]),
        (vec![3, 7, 5], vec![]),
    ];
    for (digits, expect) in cases {
        assert_eq!(Solution::find_even_numbers(digits), expect);
    }
}
