#![allow(dead_code)]

/*

// 2081. Sum of k-Mirror Numbers
// https://leetcode.com/problems/sum-of-k-mirror-numbers/
// https://leetcode.cn/problems/sum-of-k-mirror-numbers/
//
// Hard
//
// A k-mirror number is a positive integer without leading zeros that reads the same both forward and backward in base-10 as well as in base-k.

    For example, 9 is a 2-mirror number. The representation of 9 in base-10 and base-2 are 9 and 1001 respectively, which read the same both forward and backward.
    On the contrary, 4 is not a 2-mirror number. The representation of 4 in base-2 is 100, which does not read the same both forward and backward.

Given the base k and the number n, return the sum of the n smallest k-mirror numbers.

Example 1:

Input: k = 2, n = 5
Output: 25
Explanation:
The 5 smallest 2-mirror numbers and their representations in base-2 are listed as follows:
  base-10    base-2
    1          1
    3          11
    5          101
    7          111
    9          1001
Their sum = 1 + 3 + 5 + 7 + 9 = 25.

Example 2:

Input: k = 3, n = 7
Output: 499
Explanation:
The 7 smallest 3-mirror numbers are and their representations in base-3 are listed as follows:
  base-10    base-3
    1          1
    2          2
    4          11
    8          22
    121        11111
    151        12121
    212        21212
Their sum = 1 + 2 + 4 + 8 + 121 + 151 + 212 = 499.

Example 3:

Input: k = 7, n = 17
Output: 20379000
Explanation: The 17 smallest 7-mirror numbers are:
1, 2, 3, 4, 5, 6, 8, 121, 171, 242, 292, 16561, 65656, 2137312, 4602064, 6597956, 6958596

Constraints:

    2 <= k <= 9
    1 <= n <= 30
*/

struct Solution;

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        fn to_number(vec: &[i32], base: i32) -> i64 {
            vec.iter().rev().fold(0, |acc, &v| acc * base as i64 + v as i64)
        }

        fn next(vec: &mut Vec<i32>, base: i32) {
            let mut i = (vec.len() as i32 - 1) / 2;
            let mut j = vec.len() as i32 / 2;
            while j != vec.len() as i32 {
                if vec[i as usize] + 1 == base {
                    vec[i as usize] = 0;
                    vec[j as usize] = 0;
                    i -= 1;
                    j += 1;
                } else {
                    vec[i as usize] += 1;
                    vec[j as usize] = vec[i as usize];
                    return;
                }
            }
            vec.push(1);
            vec[0] = 1;
        }

        let mut basek = vec![1];
        let mut base10 = vec![1];
        let mut ret = 0;
        let mut i = 0;
        while i != n {
            let bk = to_number(&basek, k);
            let b10 = to_number(&base10, 10);
            match bk.cmp(&b10) {
                std::cmp::Ordering::Less => next(&mut basek, k),
                std::cmp::Ordering::Greater => next(&mut base10, 10),
                std::cmp::Ordering::Equal => {
                    ret += bk;
                    next(&mut basek, k);
                    i += 1;
                }
            }
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::k_mirror(2, 5), 25);
    assert_eq!(Solution::k_mirror(3, 7), 499);
    assert_eq!(Solution::k_mirror(7, 17), 20379000);
}
