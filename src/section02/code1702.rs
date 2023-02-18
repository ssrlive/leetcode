#![allow(dead_code)]

/*

// 1702. Maximum Binary String After Change
Medium
419
42
Companies

You are given a binary string binary consisting of only 0's or 1's. You can apply each of the following operations any number of times:

    Operation 1: If the number contains the substring "00", you can replace it with "10".
        For example, "00010" -> "10010"
    Operation 2: If the number contains the substring "10", you can replace it with "01".
        For example, "00010" -> "00001"

Return the maximum binary string you can obtain after any number of operations. Binary string x is greater than binary string y if x's decimal representation is greater than y's decimal representation.

Example 1:

Input: binary = "000110"
Output: "111011"
Explanation: A valid transformation sequence can be:
"000110" -> "000101"
"000101" -> "100101"
"100101" -> "110101"
"110101" -> "110011"
"110011" -> "111011"

Example 2:

Input: binary = "01"
Output: "01"
Explanation: "01" cannot be transformed any further.

Constraints:

    1 <= binary.length <= 10^5
    binary consist of '0' and '1'.
*/

struct Solution;

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let s = binary.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut stack = vec![];
        let mut temp = vec![];

        let mut i = 0;
        while i < n {
            let c = s[i];
            if temp.is_empty() {
                if c == '1' {
                    stack.push(c);
                } else {
                    temp.push(c);
                }
            } else if c == '1' {
                temp.push('1');
            } else {
                stack.push('1');
            }
            i += 1;
        }

        for c in temp {
            stack.push(c);
        }
        stack.into_iter().map(|v| v.to_string()).collect::<_>()
    }
}

#[test]
fn test() {
    let cases = vec![("000110", "111011"), ("01", "01"), ("00010", "11101")];
    for (binary, expected) in cases {
        assert_eq!(Solution::maximum_binary_string(binary.to_string()), expected);
    }
}
