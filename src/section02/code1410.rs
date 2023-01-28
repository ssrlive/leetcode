#![allow(dead_code)]

// 1410. HTML Entity Parser
// https://leetcode.com/problems/html-entity-parser/
// https://leetcode.cn/problems/html-entity-parser/
//
// Medium
//
// HTML entity parser is the parser that takes HTML code as input and replace all
// the entities of the special characters by the characters itself.
//
// The special characters and their entities for HTML are:
//
//     Quotation Mark: the entity is &quot; and symbol character is ".
//     Single Quote Mark: the entity is &apos; and symbol character is '.
//     Ampersand: the entity is &amp; and symbol character is &.
//     Greater Than Sign: the entity is &gt; and symbol character is >.
//     Less Than Sign: the entity is &lt; and symbol character is <.
//     Slash: the entity is &frasl; and symbol character is /.
//
// Given the input text string to the HTML parser, you have to implement the entity parser.
//
// Return the text after replacing the entities by the special characters.
//
// Example 1:
//
// Input: text = "&amp; is an HTML entity but &ambassador; is not."
// Output: "& is an HTML entity but &ambassador; is not."
// Explanation: The parser will replace the &amp; entity by &
//
// Example 2:
//
// Input: text = "and I quote: &quot;...&quot;"
// Output: "and I quote: \"...\""
//
// Constraints:
//
// -    1 <= text.length <= 10^5
// -    The string may contain any possible characters out of all the 256 ASCII characters.
//

struct Solution;

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut s = String::new();
        let mut i = 0;
        while i < text.len() {
            if text[i..].starts_with("&quot;") {
                s.push('"');
                i += 6;
            } else if text[i..].starts_with("&apos;") {
                s.push('\'');
                i += 6;
            } else if text[i..].starts_with("&amp;") {
                s.push('&');
                i += 5;
            } else if text[i..].starts_with("&gt;") {
                s.push('>');
                i += 4;
            } else if text[i..].starts_with("&lt;") {
                s.push('<');
                i += 4;
            } else if text[i..].starts_with("&frasl;") {
                s.push('/');
                i += 7;
            } else {
                s.push(text[i..].chars().next().unwrap());
                i += 1;
            }
        }
        s
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            "&amp; is an HTML entity but &ambassador; is not.",
            "& is an HTML entity but &ambassador; is not.",
        ),
        ("and I quote: &quot;...&quot;", "and I quote: \"...\""),
    ];
    for (text, expect) in cases {
        assert_eq!(Solution::entity_parser(text.to_string()), expect);
    }
}
