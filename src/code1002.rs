#![allow(dead_code)]

// 1002. Find Common Characters
// https://leetcode.com/problems/find-common-characters/
// https://leetcode.cn/problems/find-common-characters/
//
// Given a string array words, return an array of all characters that show up in all strings
// within the words (including duplicates). You may return the answer in any order.
//
// Example 1:
//
// Input: words = ["bella","label","roller"]
// Output: ["e","l","l"]
//
// Example 2:
//
// Input: words = ["cool","lock","cook"]
// Output: ["c","o"]
//
// Constraints:
//
// - 1 <= words.length <= 100
// - 1 <= words[i].length <= 100
// - words[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        fn get_arr_couter(word: &str) -> Vec<i32> {
            let mut v = vec![0; 26];
            word.as_bytes().iter().for_each(|&b| {
                v[(b - b'a') as usize] += 1;
            });
            v
        }

        let mut res_counter = get_arr_couter(&words[0]);
        words[1..].iter().for_each(|w| {
            let cur_counter = get_arr_couter(w);
            res_counter
                .iter_mut()
                .zip(cur_counter.iter())
                .for_each(|(res, cur)| *res = (*res).min(*cur));
        });

        res_counter
            .into_iter()
            .enumerate()
            .filter(|(_, num)| *num > 0)
            .flat_map(|(ind, num)| vec![((ind as u8 + b'a') as char).to_string(); num as usize])
            .collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["bella", "label", "roller"], vec!["e", "l", "l"]),
        (vec!["cool", "lock", "cook"], vec!["c", "o"]),
    ];
    for (words, expect) in cases {
        let words = words.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect = expect.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Solution::common_chars(words), expect);
    }
}
