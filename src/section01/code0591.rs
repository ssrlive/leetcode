#![allow(dead_code)]

// 591. Tag Validation
// https://leetcode.com/problems/tag-validator/
// https://leetcode.cn/problems/tag-validator/
//
// Given a string representing a code snippet, implement a tag validator to parse the code and return whether it is valid.
//
// A code snippet is valid if all the following rules hold:
//
// The code must be wrapped in a valid closed tag. Otherwise, the code is invalid.
//
// A closed tag (not necessarily valid) has exactly the following format : <TAG_NAME>TAG_CONTENT</TAG_NAME>.
// Among them, <TAG_NAME> is the start tag, and </TAG_NAME> is the end tag. The TAG_NAME in start and end tags should be the same.
// A closed tag is valid if and only if the TAG_NAME and TAG_CONTENT are valid.
//
// A valid TAG_NAME only contain upper-case letters, and has length in range [1,9]. Otherwise, the TAG_NAME is invalid.
//
// A valid TAG_CONTENT may contain other valid closed tags, cdata and any characters (see note1) EXCEPT unmatched <,
// unmatched start and end tag, and unmatched or closed tags with invalid TAG_NAME. Otherwise, the TAG_CONTENT is invalid.
//
// A start tag is unmatched if no end tag exists with the same TAG_NAME, and vice versa. However,
// you also need to consider the issue of unbalanced when tags are nested.
//
// A < is unmatched if you cannot find a subsequent >. And when you find a < or </,
// all the subsequent characters until the next > should be parsed as TAG_NAME (not necessarily valid).
//
// The cdata has the following format : <![CDATA[CDATA_CONTENT]]>. The range of CDATA_CONTENT is defined
// as the characters between <![CDATA[ and the first subsequent ]]>.
//
// CDATA_CONTENT may contain any characters. The function of cdata is to forbid the validator to parse CDATA_CONTENT,
// so even it has some characters that can be parsed as tag (no matter valid or invalid), you should treat it as regular characters.
//
// Example 1:
//
// Input: code = "<DIV>This is the first line <![CDATA[<div>]]></DIV>"
// Output: true
// Explanation:
// The code is wrapped in a closed tag : <DIV> and </DIV>.
// The TAG_NAME is valid, the TAG_CONTENT consists of some characters and cdata.
// Although CDATA_CONTENT has an unmatched start tag with invalid TAG_NAME, it should be considered as plain text, not parsed as a tag.
// So TAG_CONTENT is valid, and then the code is valid. Thus return true.
//
// Example 2:
//
// Input: code = "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>"
// Output: true
// Explanation:
// We first separate the code into : start_tag|tag_content|end_tag.
// start_tag -> "<DIV>"
// end_tag -> "</DIV>"
// tag_content could also be separated into : text1|cdata|text2.
// text1 -> ">>  ![cdata[]] "
// cdata -> "<![CDATA[<div>]>]]>", where the CDATA_CONTENT is "<div>]>"
// text2 -> "]]>>]"
// The reason why start_tag is NOT "<DIV>>>" is because of the rule 6.
// The reason why cdata is NOT "<![CDATA[<div>]>]]>]]>" is because of the rule 7.
//
// Example 3:
//
// Input: code = "<A>  <B> </A>   </B>"
// Output: false
// Explanation: Unbalanced. If "<A>" is closed, then "<B>" must be unmatched, and vice versa.
//
// Constraints:
//
// - 1 <= code.length <= 500
// - code consists of English letters, digits, '<', '>', '/', '!', '[', ']', '.', and ' '.
//

struct Solution;

impl Solution {
    pub fn is_valid(code: String) -> bool {
        Self::_is_valid(code.as_bytes()).unwrap_or(false)
    }

    fn _is_valid(code: &[u8]) -> Option<bool> {
        let mut s = String::new();
        let mut i = 0;
        while i < code.len() {
            if code[i] == b'<' && code.get(i + 1)? == &b'!' {
                let mut t = String::new();
                for (j, &item) in code.iter().enumerate().skip(i + 2) {
                    if j - i == 9 {
                        break;
                    }
                    t.push(item as char);
                }
                if t == "[CDATA[" || t == "[cdata[" {
                    i += 7;
                    while i < code.len() {
                        if code[i - 2] == b']' && code[i - 1] == b']' && code[i] == b'>' {
                            break;
                        }
                        i += 1;
                    }
                    s.push('#');
                } else {
                    return Some(false);
                }
            } else {
                s.push(code[i] as char);
            }
            i += 1;
        }

        let s = s.as_bytes();

        if s.is_empty() || s[0] != b'<' {
            return Some(false);
        }

        let mut st = Vec::new();
        let mut k = 0;
        let mut n = 0;
        i = 0;
        while i < s.len() {
            if s[i] == b'<' {
                let mut t = String::new();
                n = 0;
                if i == s.len() {
                    return Some(false);
                }

                if s.get(i + 1)? == &b'/' {
                    i += 2;
                    while i < s.len() {
                        let &s_i_byte = s.get(i)?;
                        if s_i_byte == b'>' {
                            break;
                        }
                        if !s_i_byte.is_ascii_uppercase() {
                            return Some(false);
                        }
                        t.push(s_i_byte as char);
                        if t.len() == 10 {
                            return Some(false);
                        }
                        i += 1;
                    }

                    if st.is_empty() || st.pop()? != t {
                        return Some(false);
                    }
                } else {
                    i += 1;
                    while i < s.len() {
                        let &s_i_byte = s.get(i)?;
                        if s_i_byte == b'>' {
                            break;
                        }
                        if !s_i_byte.is_ascii_uppercase() {
                            return Some(false);
                        }
                        t.push(s_i_byte as char);
                        if t.len() == 10 {
                            return Some(false);
                        }
                        i += 1;
                    }

                    if t.is_empty()
                        || (st.is_empty() && {
                            k += 1;
                            k == 2
                        })
                    {
                        return Some(false);
                    }
                    st.push(t);
                }
            } else {
                n = 1;
            }
            i += 1;
        }
        Some(st.is_empty() && n == 0)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid("<".to_string()), false);
    assert_eq!(Solution::is_valid(">A<".to_string()), false);
    assert_eq!(
        Solution::is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string()),
        true
    );
    assert_eq!(
        Solution::is_valid("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_string()),
        true
    );
    assert_eq!(Solution::is_valid("<A>  <B> </A>   </B>".to_string()), false);
    assert_eq!(
        Solution::is_valid("<DIV>  div tag is not closed  <DIV>".to_string()),
        false
    );
    assert_eq!(Solution::is_valid("<DIV>  unmatched <  </DIV>".to_string()), false);
    assert_eq!(
        Solution::is_valid("<DIV> closed tags with invalid tag name  <b>123</b> </DIV>".to_string()),
        false
    );
    assert_eq!(
        Solution::is_valid(
            "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>".to_string()
        ),
        false
    );
    assert_eq!(
        Solution::is_valid("<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>".to_string()),
        false
    );
    assert_eq!(
        Solution::is_valid("<![CDATA[wahaha]]]><![CDATA[]> wahaha]]>".to_string()),
        false
    );
}
