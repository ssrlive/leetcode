#![allow(dead_code)]

/*

// 1982. Find Array Given Subset Sums
// https://leetcode.com/problems/find-array-given-subset-sums/
// https://leetcode.cn/problems/find-array-given-subset-sums/
//
// Hard
//
// You are given an integer n representing the length of an unknown array that you are trying to recover. You are also given an array sums containing the values of all 2n subset sums of the unknown array (in no particular order).

Return the array ans of length n representing the unknown array. If multiple answers exist, return any of them.

An array sub is a subset of an array arr if sub can be obtained from arr by deleting some (possibly zero or all) elements of arr. The sum of the elements in sub is one possible subset sum of arr. The sum of an empty array is considered to be 0.

Note: Test cases are generated such that there will always be at least one correct answer.

Example 1:

Input: n = 3, sums = [-3,-2,-1,0,0,1,2,3]
Output: [1,2,-3]
Explanation: [1,2,-3] is able to achieve the given subset sums:
- []: sum is 0
- [1]: sum is 1
- [2]: sum is 2
- [1,2]: sum is 3
- [-3]: sum is -3
- [1,-3]: sum is -2
- [2,-3]: sum is -1
- [1,2,-3]: sum is 0
Note that any permutation of [1,2,-3] and also any permutation of [-1,-2,3] will also be accepted.

Example 2:

Input: n = 2, sums = [0,0,0,0]
Output: [0,0]
Explanation: The only correct answer is [0,0].

Example 3:

Input: n = 4, sums = [0,0,5,5,4,-1,4,9,9,-1,4,3,4,8,3,8]
Output: [0,-1,4,5]
Explanation: [0,-1,4,5] is able to achieve the given subset sums.

Constraints:

    1 <= n <= 15
    sums.length == 2n
    -10^4 <= sums[i] <= 10^4
*/

struct Solution;

impl Solution {
    pub fn recover_array(_n: i32, sums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut s = sums;
        s.sort();
        while s.len() > 1 {
            let mut l = vec![];
            let mut r = vec![];
            let num = s[1] - s[0];
            let mut l_zero = false;
            for i in 0..s.len() {
                if s[i] != i32::MIN {
                    l_zero |= s[i] == 0;
                    l.push(s[i]);
                    r.push(s[i] + num);
                    for j in i + 1..s.len() {
                        if s[j] == s[i] + num {
                            s[j] = i32::MIN;
                            break;
                        }
                    }
                }
            }
            res.push(num * if l_zero { 1 } else { -1 });
            s = if l_zero { l } else { r };
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, vec![-3, -2, -1, 0, 0, 1, 2, 3], vec![1, 2, -3]),
        (2, vec![0, 0, 0, 0], vec![0, 0]),
        (4, vec![0, 0, 5, 5, 4, -1, 4, 9, 9, -1, 4, 3, 4, 8, 3, 8], vec![0, -1, 4, 5]),
    ];
    for (n, sums, expect) in cases {
        assert_eq!(Solution::recover_array(n, sums), expect);
    }
}
