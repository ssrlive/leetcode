#![allow(dead_code)]

// 423. Reconstruct Original Digits from English
// https://leetcode.com/problems/reconstruct-original-digits-from-english/
// https://leetcode.cn/problems/reconstruct-original-digits-from-english/
//
// Given a string s containing an out-of-order English representation of digits 0-9, return the digits in ascending order.
//
// Example 1:
//
// Input: s = "owoztneoer"
// Output: "012"
//
// Example 2:
//
// Input: s = "fviefuro"
// Output: "45"
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s[i] is one of the characters ["e","g","f","i","h","o","n","s","r","u","t","w","v","x","z"].
// - s is guaranteed to be valid.
//

struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let mut digits = vec![0; 10];
        digits[0] = *map.get(&'z').unwrap_or(&0);
        digits[2] = *map.get(&'w').unwrap_or(&0);
        digits[4] = *map.get(&'u').unwrap_or(&0);
        digits[6] = *map.get(&'x').unwrap_or(&0);
        digits[8] = *map.get(&'g').unwrap_or(&0);
        digits[1] = *map.get(&'o').unwrap_or(&0) - digits[0] - digits[2] - digits[4];
        digits[3] = *map.get(&'h').unwrap_or(&0) - digits[8];
        digits[5] = *map.get(&'f').unwrap_or(&0) - digits[4];
        digits[7] = *map.get(&'s').unwrap_or(&0) - digits[6];
        digits[9] = *map.get(&'i').unwrap_or(&0) - digits[5] - digits[6] - digits[8];

        let mut result = String::new();
        for (i, &count) in digits.iter().enumerate() {
            for _ in 0..count {
                result.push_str(&i.to_string());
            }
        }
        result
    }
}

#[test]
fn test_original_digits() {
    assert_eq!(Solution::original_digits("owoztneoer".to_string()), "012");
    assert_eq!(Solution::original_digits("fviefuro".to_string()), "45");
}
