#![allow(dead_code)]

/*

// 1520. Maximum Number of Non-Overlapping Substrings
// https://leetcode.com/problems/maximum-number-of-non-overlapping-substrings/
// https://leetcode.cn/problems/maximum-number-of-non-overlapping-substrings/
//
// Hard
//
// Given a string s of lowercase letters, you need to find the maximum number of non-empty substrings of s that meet the following conditions:

    The substrings do not overlap, that is for any two substrings s[i..j] and s[x..y], either j < x or i > y is true.
    A substring that contains a certain character c must also contain all occurrences of c.

Find the maximum number of substrings that meet the above conditions. If there are multiple solutions with the same number of substrings, return the one with minimum total length. It can be shown that there exists a unique solution of minimum total length.

Notice that you can return the substrings in any order.

Example 1:

Input: s = "adefaddaccc"
Output: ["e","f","ccc"]
Explanation: The following are all the possible substrings that meet the conditions:
[
  "adefaddaccc"
  "adefadda",
  "ef",
  "e",
  "f",
  "ccc",
]
If we choose the first string, we cannot choose anything else and we'd get only 1. If we choose "adefadda", we are left with "ccc" which is the only one that doesn't overlap, thus obtaining 2 substrings. Notice also, that it's not optimal to choose "ef" since it can be split into two. Therefore, the optimal way is to choose ["e","f","ccc"] which gives us 3 substrings. No other solution of the same number of substrings exist.

Example 2:

Input: s = "abbaccd"
Output: ["d","bb","cc"]
Explanation: Notice that while the set of substrings ["d","abba","cc"] also has length 3, it's considered incorrect since it has larger total length.

Constraints:

    1 <= s.length <= 10^5
    s contains only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        use std::cmp::{max, min};
        let char_array = s.as_bytes();

        let mut fc = [usize::MAX; 123];
        let mut lc = [0usize; 123];
        // step 1: collect indices
        for (index, &chr) in char_array.iter().enumerate() {
            let chr = chr as usize;
            fc[chr] = min(fc[chr], index);
            lc[chr] = index;
        }
        // update intervals to include all the inner characters
        let mut intervals = Vec::new();

        for (index, i) in fc.iter().enumerate() {
            let mut r = lc[index];
            let mut j = *i;
            while j < r {
                // interval of character at j starts before the current interval
                if fc[char_array[j] as usize] < *i {
                    // interval is duplicated
                    break;
                }

                // update the interval with the new character's rightmost occurence
                r = max(r, lc[char_array[j] as usize]);
                j += 1;
            }
            if j == r {
                intervals.push((*i, r));
            }
        }

        // Interval scheduling optimization
        intervals.sort_unstable_by_key(|x| x.1);
        let mut res = Vec::new();
        let mut i = 0;
        for (st, f) in intervals {
            if st < i {
                continue;
            }
            i = f;
            let mut sbs = String::new();
            for &item in char_array.iter().take(f + 1).skip(st) {
                sbs.push(char::from(item));
            }
            res.push(sbs);
        }

        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("adefaddaccc", vec!["e", "f", "ccc"]),
        ("abbaccd", vec!["bb", "cc", "d"]),
    ];
    for (s, expect) in cases {
        let ans = Solution::max_num_of_substrings(s.to_string());
        assert_eq!(ans, expect);
    }
}
