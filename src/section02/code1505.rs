#![allow(dead_code)]

/*

// 1505. Minimum Possible Integer After at Most K Adjacent Swaps On Digits
// https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits/
// https://leetcode.cn/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits/
//
// Hard
//
// You are given a string num representing the digits of a very large integer and an integer k. You are allowed to swap any two adjacent digits of the integer at most k times.

Return the minimum integer you can obtain also as a string.

Example 1:

Input: num = "4321", k = 4
Output: "1342"
Explanation: The steps to obtain the minimum integer from 4321 with 4 adjacent swaps are shown.

Example 2:

Input: num = "100", k = 1
Output: "010"
Explanation: It's ok for the output to have leading zeros, but the input is guaranteed not to have any leading zeros.

Example 3:

Input: num = "36789", k = 1000
Output: "36789"
Explanation: We can keep the number without any swaps.

Constraints:

    1 <= num.length <= 3 * 10^4
    num consists of only digits and does not contain leading zeros.
    1 <= k <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_integer(num: String, k: i32) -> String {
        let mut k = k as usize;
        let num = num.into_bytes();
        let n = num.len();
        let mut res = Vec::with_capacity(n);
        let mut q = [n; 10];
        for (i, &item) in num.iter().enumerate() {
            let d = (item - b'0') as usize;
            if q[d] == n {
                q[d] = i;
            }
        }
        let mut used = vec![false; n];
        let mut q_used = [0; 10];
        for _ in 0..n {
            for d in 0..10_usize {
                if q[d] == n {
                    continue;
                }
                let c = q[d] - q_used[d];
                if c <= k {
                    k -= c;
                    res.push(b'0' + d as u8);
                    used[q[d]] = true;
                    for d1 in 0..10_usize {
                        if q[d1] > q[d] {
                            q_used[d1] += 1;
                        }
                    }
                    while q[d] < n {
                        if used[q[d]] {
                            q_used[d] += 1;
                        }
                        q[d] += 1;
                        let &c = num.get(q[d]).unwrap_or(&0_u8);
                        if c == b'0' + d as u8 {
                            break;
                        }
                    }
                    break;
                }
            }
        }
        String::from_utf8(res).unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("4321", 4, "1342"),
        ("100", 1, "010"),
        ("36789", 1000, "36789"),
        ("22", 22, "22"),
        ("9438957234785635408", 23, "0345989723478563548"),
    ];
    for (num, k, expected) in cases {
        assert_eq!(Solution::min_integer(num.to_string(), k), expected);
    }
}
