#![allow(dead_code)]

// 443. String Compression
// https://leetcode.com/problems/string-compression/
// https://leetcode.cn/problems/string-compression/
//
// Given an array of characters chars, compress it using the following algorithm:
//
// Begin with an empty string s. For each group of consecutive repeating characters in chars:
//
// If the group's length is 1, append the character to s.
// Otherwise, append the character followed by the group's length.
// The compressed string s should not be returned separately, but instead be stored in the input character array chars.
// Note that group lengths that are 10 or longer will be split into multiple characters in chars.
//
// After you are done modifying the input array, return the new length of the array.
//
// You must write an algorithm that uses only constant extra space.
//
// Example 1:
//
// Input: chars = ["a","a","b","b","c","c","c"]
// Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
// Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".
//
// Example 2:
//
// Input: chars = ["a"]
// Output: Return 1, and the first character of the input array should be: ["a"]
// Explanation: The only group is "a", which remains uncompressed since it's a single character.
//
// Example 3:
//
// Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
// Output: Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
// Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".
//
// Example 4:
//
// Input: chars = ["a","a","a","b","b","a","a"]
// Output: Return 6, and the first 6 characters of the input array should be: ["a","3","b","2","a","2"].
// Explanation: The groups are "aaa", "bb", and "aa". This compresses to "a3b2a2". Note that each group is independent
// even if two groups have the same character.
//
// Constraints:
//
// - 1 <= chars.length <= 2000
// - chars[i] is a lower-case English letter, upper-case English letter, digit, or symbol.
//

struct Solution;

impl Solution {
    pub fn compress(chars: &mut [char]) -> i32 {
        fn assign_count(target: &mut [char], count: i32) -> usize {
            if count == 1 {
                return 0;
            }
            let count_str = count.to_string();
            for (i, v) in count_str.chars().enumerate() {
                target[i] = v;
            }
            count_str.len()
        }

        let mut count = 1;
        let mut res_index = 0;
        chars[res_index] = chars[0];
        for i in 1..chars.len() {
            if chars[res_index] != chars[i] {
                res_index += 1;
                res_index = res_index + assign_count(&mut chars[res_index..], count);
                chars[res_index] = chars[i];
                count = 1;
            } else {
                count += 1;
            }
        }
        res_index += 1;
        res_index = res_index + assign_count(&mut chars[res_index..], count);
        res_index as i32
    }
}

#[test]
fn test() {
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    assert_eq!(Solution::compress(&mut chars), 6);
    assert_eq!(&chars[..6], vec!['a', '2', 'b', '2', 'c', '3']);

    let mut chars = vec!['a'];
    assert_eq!(Solution::compress(&mut chars), 1);
    assert_eq!(chars, vec!['a']);

    let mut chars = vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'];
    assert_eq!(Solution::compress(&mut chars), 4);
    assert_eq!(&chars[..4], vec!['a', 'b', '1', '2']);

    let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
    assert_eq!(Solution::compress(&mut chars), 6);
    assert_eq!(&chars[..6], vec!['a', '3', 'b', '2', 'a', '2']);
}
