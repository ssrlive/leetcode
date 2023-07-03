#![allow(dead_code)]

/*

// 1647. Minimum Deletions to Make Character Frequencies Unique
// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/
// https://leetcode.cn/problems/minimum-deletions-to-make-character-frequencies-unique/
//
// Medium
//
// A string s is called good if there are no two different characters in s that have the same frequency.

Given a string s, return the minimum number of characters you need to delete to make s good.

The frequency of a character in a string is the number of times it appears in the string. For example, in the string "aab", the frequency of 'a' is 2, while the frequency of 'b' is 1.

Example 1:

Input: s = "aab"
Output: 0
Explanation: s is already good.

Example 2:

Input: s = "aaabbbcc"
Output: 2
Explanation: You can delete two 'b's resulting in the good string "aaabcc".
Another way it to delete one 'b' and one 'c' resulting in the good string "aaabbc".

Example 3:

Input: s = "ceabaacb"
Output: 2
Explanation: You can delete both 'c's resulting in the good string "eabaab".
Note that we only care about characters that are still in the string at the end (i.e. frequency of 0 is ignored).

Constraints:

    1 <= s.length <= 10^5
    s contains only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        use std::collections::HashSet;
        let counts = s.as_bytes().iter().fold([0; (b'z' - b'a' + 1) as usize], |mut counts, b| {
            counts[(*b - b'a') as usize] += 1;
            counts
        });
        let mut set = HashSet::new();
        let mut deletions = 0;
        for count in counts {
            let mut count = count;
            while count > 0 && !set.insert(count) {
                count -= 1;
                deletions += 1;
            }
        }
        deletions
    }
}

#[test]
fn test() {
    let cases = vec![("aab", 0), ("aaabbbcc", 2), ("ceabaacb", 2)];
    for (s, expected) in cases {
        assert_eq!(Solution::min_deletions(s.to_string()), expected);
    }
}
