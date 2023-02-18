#![allow(dead_code)]

/*

// 1704. Determine if String Halves Are Alike
Easy
1.6K
79
Companies

You are given a string s of even length. Split this string into two halves of equal lengths, and let a be the first half and b be the second half.

Two strings are alike if they have the same number of vowels ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'). Notice that s contains uppercase and lowercase letters.

Return true if a and b are alike. Otherwise, return false.

Example 1:

Input: s = "book"
Output: true
Explanation: a = "bo" and b = "ok". a has 1 vowel and b has 1 vowel. Therefore, they are alike.

Example 2:

Input: s = "textbook"
Output: false
Explanation: a = "text" and b = "book". a has 1 vowel whereas b has 2. Therefore, they are not alike.
Notice that the vowel o is counted twice.

Constraints:

    2 <= s.length <= 1000
    s.length is even.
    s consists of uppercase and lowercase letters.
*/

struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        fn is_vowel(c: u8) -> bool {
            matches!(c, b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U')
        }

        let mut count = 0;
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if is_vowel(s.as_bytes()[i]) {
                count += 1;
            }
            if is_vowel(s.as_bytes()[j]) {
                count -= 1;
            }
            i += 1;
            j -= 1;
        }
        count == 0
    }
}

#[test]
fn test() {
    let cases = vec![
        ("book", true),
        ("textbook", false),
        ("MerryChristmas", false),
        ("AbCdEfGh", true),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::halves_are_alike(s.to_string()), expected);
    }
}
