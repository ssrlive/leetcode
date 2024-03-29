#![allow(dead_code)]

/*

// 2227. Encrypt and Decrypt Strings
// https://leetcode.com/problems/encrypt-and-decrypt-strings/
// https://leetcode.cn/problems/encrypt-and-decrypt-strings/
//
// Hard
//
// You are given a character array keys containing unique characters and a string array values containing strings of length 2. You are also given another string array dictionary that contains all permitted original strings after decryption. You should implement a data structure that can encrypt or decrypt a 0-indexed string.

A string is encrypted with the following process:

    For each character c in the string, we find the index i satisfying keys[i] == c in keys.
    Replace c with values[i] in the string.

Note that in case a character of the string is not present in keys, the encryption process cannot be carried out, and an empty string "" is returned.

A string is decrypted with the following process:

    For each substring s of length 2 occurring at an even index in the string, we find an i such that values[i] == s. If there are multiple valid i, we choose any one of them. This means a string could have multiple possible strings it can decrypt to.
    Replace s with keys[i] in the string.

Implement the Encrypter class:

    Encrypter(char[] keys, String[] values, String[] dictionary) Initializes the Encrypter class with keys, values, and dictionary.
    String encrypt(String word1) Encrypts word1 with the encryption process described above and returns the encrypted string.
    int decrypt(String word2) Returns the number of possible strings word2 could decrypt to that also appear in dictionary.

Example 1:

Input
["Encrypter", "encrypt", "decrypt"]
[[['a', 'b', 'c', 'd'], ["ei", "zf", "ei", "am"], ["abcd", "acbd", "adbc", "badc", "dacb", "cadb", "cbda", "abad"]], ["abcd"], ["eizfeiam"]]
Output
[null, "eizfeiam", 2]

Explanation
Encrypter encrypter = new Encrypter([['a', 'b', 'c', 'd'], ["ei", "zf", "ei", "am"], ["abcd", "acbd", "adbc", "badc", "dacb", "cadb", "cbda", "abad"]);
encrypter.encrypt("abcd"); // return "eizfeiam".
                           // 'a' maps to "ei", 'b' maps to "zf", 'c' maps to "ei", and 'd' maps to "am".
encrypter.decrypt("eizfeiam"); // return 2.
                              // "ei" can map to 'a' or 'c', "zf" maps to 'b', and "am" maps to 'd'.
                              // Thus, the possible strings after decryption are "abad", "cbad", "abcd", and "cbcd".
                              // 2 of those strings, "abad" and "abcd", appear in dictionary, so the answer is 2.

Constraints:

    1 <= keys.length == values.length <= 26
    values[i].length == 2
    1 <= dictionary.length <= 100
    1 <= dictionary[i].length <= 100
    All keys[i] and dictionary[i] are unique.
    1 <= word1.length <= 2000
    1 <= word2.length <= 200
    All word1[i] appear in keys.
    word2.length is even.
    keys, values[i], dictionary[i], word1, and word2 only contain lowercase English letters.
    At most 200 calls will be made to encrypt and decrypt in total.
*/

use std::collections::HashMap;

struct Encrypter {
    value_map: [String; 26],
    dict_map: HashMap<String, i32>,
}

impl Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mut value_map: [String; 26] = Default::default();
        keys.into_iter().zip(values).for_each(
            #[inline]
            |(key, value)| value_map[(key as u8 - b'a') as usize] = value,
        );
        let mut res = Self {
            value_map,
            dict_map: HashMap::with_capacity(5),
        };
        for word in dictionary {
            let key = res.encrypt(word);
            *res.dict_map.entry(key).or_insert(0) += 1;
        }
        res
    }

    fn encrypt(&self, word1: String) -> String {
        let mut res: String = String::with_capacity(word1.len() * 2);
        for b in word1.into_bytes() {
            if !self.value_map[(b - b'a') as usize].is_empty() {
                res.push_str(&self.value_map[(b - b'a') as usize]);
            } else {
                return String::new();
            }
        }
        res
    }

    fn decrypt(&self, word2: String) -> i32 {
        *self.dict_map.get(&word2).unwrap_or(&0)
    }
}

#[test]
fn test() {
    let encrypter = Encrypter::new(
        vec!['a', 'b', 'c', 'd'],
        vec!["ei".to_string(), "zf".to_string(), "ei".to_string(), "am".to_string()],
        vec![
            "abcd".to_string(),
            "acbd".to_string(),
            "adbc".to_string(),
            "badc".to_string(),
            "dacb".to_string(),
            "cadb".to_string(),
            "cbda".to_string(),
            "abad".to_string(),
        ],
    );
    assert_eq!(encrypter.encrypt("abcd".to_string()), "eizfeiam".to_string());
    assert_eq!(encrypter.decrypt("eizfeiam".to_string()), 2);
}
