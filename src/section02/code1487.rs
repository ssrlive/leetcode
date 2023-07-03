#![allow(dead_code)]

/*

// 1487. Making File Names Unique
// https://leetcode.com/problems/making-file-names-unique/
// https://leetcode.cn/problems/making-file-names-unique/
//
// Medium
//
// Given an array of strings names of size n. You will create n folders in your file system such that, at the ith minute, you will create a folder with the name names[i].

Since two files cannot have the same name, if you enter a folder name that was previously used, the system will have a suffix addition to its name in the form of (k), where, k is the smallest positive integer such that the obtained name remains unique.

Return an array of strings of length n where ans[i] is the actual name the system will assign to the ith folder when you create it.

Example 1:

Input: names = ["pes","fifa","gta","pes(2019)"]
Output: ["pes","fifa","gta","pes(2019)"]
Explanation: Let's see how the file system creates folder names:
"pes" --> not assigned before, remains "pes"
"fifa" --> not assigned before, remains "fifa"
"gta" --> not assigned before, remains "gta"
"pes(2019)" --> not assigned before, remains "pes(2019)"

Example 2:

Input: names = ["gta","gta(1)","gta","avalon"]
Output: ["gta","gta(1)","gta(2)","avalon"]
Explanation: Let's see how the file system creates folder names:
"gta" --> not assigned before, remains "gta"
"gta(1)" --> not assigned before, remains "gta(1)"
"gta" --> the name is reserved, system adds (k), since "gta(1)" is also reserved, systems put k = 2. it becomes "gta(2)"
"avalon" --> not assigned before, remains "avalon"

Example 3:

Input: names = ["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece"]
Output: ["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece(4)"]
Explanation: When the last folder is created, the smallest positive valid k is 4, and it becomes "onepiece(4)".

Constraints:

    1 <= names.length <= 5 * 10^4
    1 <= names[i].length <= 20
    names[i] consists of lowercase English letters, digits, and/or round brackets.
*/

struct Solution;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut m = std::collections::HashMap::<String, usize>::new();
        let mut ans = Vec::new();
        for name in names.iter() {
            if let Some(k) = m.get(name) {
                let mut k = *k;
                #[allow(unused_assignments)]
                let mut str2 = String::new();
                loop {
                    str2 = format!("{name}({k})");
                    *m.entry(name.clone()).or_insert(0) += 1;
                    k += 1;
                    if !m.contains_key(&str2) {
                        break;
                    }
                }
                *m.entry(str2.clone()).or_insert(0) += 1;
                ans.push(str2);
            } else {
                *m.entry(name.clone()).or_insert(0) += 1;
                ans.push(name.clone());
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["pes", "fifa", "gta", "pes(2019)"], vec!["pes", "fifa", "gta", "pes(2019)"]),
        (vec!["gta", "gta(1)", "gta", "avalon"], vec!["gta", "gta(1)", "gta(2)", "avalon"]),
        (
            vec!["onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece"],
            vec!["onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece(4)"],
        ),
    ];
    for (names, expected) in cases {
        let names = names.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::get_folder_names(names), expected);
    }
}
