#![allow(dead_code)]

/*

// 2053. Kth Distinct String in an Array
// https://leetcode.com/problems/kth-distinct-string-in-an-array/
// https://leetcode.cn/problems/kth-distinct-string-in-an-array/
//
// Easy
//
// A distinct string is a string that is present only once in an array.

Given an array of strings arr, and an integer k, return the kth distinct string present in arr. If there are fewer than k distinct strings, return an empty string "".

Note that the strings are considered in the order in which they appear in the array.

Example 1:

Input: arr = ["d","b","c","b","c","a"], k = 2
Output: "a"
Explanation:
The only distinct strings in arr are "d" and "a".
"d" appears 1st, so it is the 1st distinct string.
"a" appears 2nd, so it is the 2nd distinct string.
Since k == 2, "a" is returned.

Example 2:

Input: arr = ["aaa","aa","a"], k = 1
Output: "aaa"
Explanation:
All strings in arr are distinct, so the 1st string "aaa" is returned.

Example 3:

Input: arr = ["a","b","a"], k = 3
Output: ""
Explanation:
The only distinct string is "b". Since there are fewer than 3 distinct strings, we return an empty string "".

Constraints:

    1 <= k <= arr.length <= 1000
    1 <= arr[i].length <= 5
    arr[i] consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut hm = std::collections::HashMap::new();
        arr.iter().enumerate().for_each(|(ind, s)| {
            let e = hm.entry(s).or_insert_with(|| vec![0, ind]);
            (*e)[0] += 1;
        });

        let mut v: Vec<(&String, Vec<usize>)> = hm.into_iter().filter(|(_, v)| v[0] == 1).collect();
        v.sort_unstable_by_key(|(_, v)| v[1]);
        v.iter()
            .map(|(s, _)| *s)
            .nth((k - 1) as usize)
            .unwrap_or(&"".to_string())
            .to_string()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["d", "b", "c", "b", "c", "a"], 2, "a"),
        (vec!["aaa", "aa", "a"], 1, "aaa"),
        (vec!["a", "b", "a"], 3, ""),
    ];
    for (arr, k, expect) in cases {
        let arr: Vec<String> = arr.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::kth_distinct(arr, k), expect);
    }
}
