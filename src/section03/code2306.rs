#![allow(dead_code)]

/*

// 2306. Naming a Company
// https://leetcode.com/problems/naming-a-company/
// https://leetcode.cn/problems/naming-a-company/
//
// Hard
//
// You are given an array of strings ideas that represents a list of names to be used in the process of naming a company. The process of naming a company is as follows:

    Choose 2 distinct names from ideas, call them ideaA and ideaB.
    Swap the first letters of ideaA and ideaB with each other.
    If both of the new names are not found in the original ideas, then the name ideaA ideaB (the concatenation of ideaA and ideaB, separated by a space) is a valid company name.
    Otherwise, it is not a valid name.

Return the number of distinct valid names for the company.

Example 1:

Input: ideas = ["coffee","donuts","time","toffee"]
Output: 6
Explanation: The following selections are valid:
- ("coffee", "donuts"): The company name created is "doffee conuts".
- ("donuts", "coffee"): The company name created is "conuts doffee".
- ("donuts", "time"): The company name created is "tonuts dime".
- ("donuts", "toffee"): The company name created is "tonuts doffee".
- ("time", "donuts"): The company name created is "dime tonuts".
- ("toffee", "donuts"): The company name created is "doffee tonuts".
Therefore, there are a total of 6 distinct company names.

The following are some examples of invalid selections:
- ("coffee", "time"): The name "toffee" formed after swapping already exists in the original array.
- ("time", "toffee"): Both names are still the same after swapping and exist in the original array.
- ("coffee", "toffee"): Both names formed after swapping already exist in the original array.

Example 2:

Input: ideas = ["lack","back"]
Output: 0
Explanation: There are no valid selections. Therefore, 0 is returned.

Constraints:

    2 <= ideas.length <= 5 * 10^4
    1 <= ideas[i].length <= 10
    ideas[i] consists of lowercase English letters.
    All the strings in ideas are unique.
*/

struct Solution;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        use std::collections::HashMap;
        let mut ans = 0;
        let mut count = [0; 26];
        let mut tab = vec![vec![0; 26]; 26];
        let mut suffix: HashMap<&str, usize> = HashMap::new();
        for word in ideas.iter() {
            let c = word.chars().next().unwrap() as usize - 'a' as usize;
            count[c] += 1;
            let suf = &word[1..];
            let letters = suffix.entry(suf).or_insert(0);
            for i in (0..26).filter(|&i| (1_usize << i) & *letters > 0) {
                tab[i][c] += 1;
                tab[c][i] += 1;
            }
            *letters |= 1 << c;
        }
        for i in 0..26 {
            for j in 0..i {
                ans += 2 * (count[i] - tab[i][j]) * (count[j] - tab[j][i]);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["coffee", "donuts", "time", "toffee"], 6),
        (vec!["lack", "back"], 0),
        (vec!["a", "b", "c", "d", "e", "f", "g", "x", "y", "z"], 0),
    ];
    for (ideas, expect) in cases {
        let ideas = ideas.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::distinct_names(ideas), expect);
    }
}
