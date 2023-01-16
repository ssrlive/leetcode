#![allow(dead_code)]

// 830. Positions of Large Groups
// https://leetcode.com/problems/positions-of-large-groups/
// https://leetcode.cn/problems/positions-of-large-groups/
//
// In a string s of lowercase letters, these letters form consecutive groups of the same character.
//
// For example, a string like s = "abbxxxxzyy" has the groups "a", "bb", "xxxx", "z", and "yy".
//
// A group is identified by an interval [start, end], where start and end denote the start and end indices (inclusive) of the group.
// In the above example, "xxxx" has the interval [3,6].
//
// A group is considered large if it has 3 or more characters.
//
// Return the intervals of every large group sorted in increasing order by start index.
//
// Example 1:
//
// Input: s = "abbxxxxzzy"
// Output: [[3,6]]
// Explanation: "xxxx" is the only large group with start index 3 and end index 6.
//
// Example 2:
//
// Input: s = "abc"
// Output: []
// Explanation: We have groups "a", "b", and "c", none of which are large groups.
//
// Example 3:
//
// Input: s = "abcdddeeeeaabbbcd"
// Output: [[3,5],[6,9],[12,14]]
// Explanation: The large groups are "ddd", "eeee", and "bbb".
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s contains lowercase English letters only.
//

struct Solution;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        fn _large_group_positions(s: String) -> Option<Vec<Vec<i32>>> {
            let mut ret = vec![];
            let mut i = 0;
            let mut j = 0;
            let mut c = s.chars().next()?;
            while j < s.len() {
                if s.chars().nth(j)? == c {
                    j += 1;
                } else {
                    if j - i >= 3 {
                        ret.push(vec![i as i32, (j - 1) as i32]);
                    }
                    i = j;
                    c = s.chars().nth(j)?;
                }
            }
            if j - i >= 3 {
                ret.push(vec![i as i32, (j - 1) as i32]);
            }
            Some(ret)
        }

        _large_group_positions(s).unwrap_or_default()
    }
}

#[test]
fn test() {
    let s = "abbxxxxzzy".to_string();
    assert_eq!(Solution::large_group_positions(s), vec![vec![3, 6]]);

    let s = "abc".to_string();
    assert_eq!(Solution::large_group_positions(s), Vec::<Vec<i32>>::new());

    let s = "abcdddeeeeaabbbcd".to_string();
    let expected = vec![vec![3, 5], vec![6, 9], vec![12, 14]];
    assert_eq!(Solution::large_group_positions(s), expected);
}
