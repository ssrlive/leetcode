#![allow(dead_code)]

// 535. Encode and Decode TinyURL
// https://leetcode.com/problems/encode-and-decode-tinyurl/
// https://leetcode.cn/problems/encode-and-decode-tinyurl/
//
// Note: This is a companion problem to the System Design problem: Design TinyURL.
//
// TinyURL is a URL shortening service where you enter a URL such as https://leetcode.com/problems/design-tinyurl
// and it returns a short URL such as http://tinyurl.com/4e9iAk. Design a class to encode a URL and decode a tiny URL.
//
// There is no restriction on how your encode/decode algorithm should work. You just need to ensure
// that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.
//
// Implement the Solution class:
//
// Solution() Initializes the object of the system.
// String encode(String longUrl) Returns a tiny URL for the given longUrl.
// String decode(String shortUrl) Returns the original long URL for the given shortUrl.
// It is guaranteed that the given shortUrl was encoded by the same object.
//
// Example 1:
//
// Input: url = "https://leetcode.com/problems/design-tinyurl"
// Output: "https://leetcode.com/problems/design-tinyurl"
//
// Explanation:
// Solution obj = new Solution();
// string tiny = obj.encode(url); // returns the encoded tiny url.
// string ans = obj.decode(tiny); // returns the original url after deconding it.
//
// Constraints:
//
// - 1 <= url.length <= 10^4
// - url is guranteed to be a valid URL.
//
// Follow up: What if we enter the same URL twice?

struct Codec;

impl Codec {
    pub fn new() -> Self {
        Self
    }

    // Encodes a URL to a shortened URL.
    pub fn encode(&self, long_url: String) -> String {
        long_url
    }

    // Decodes a shortened URL to its original URL.
    pub fn decode(&self, short_url: String) -> String {
        short_url
    }
}

#[test]
fn test() {
    let solution = Codec::new();
    let long_url = "https://leetcode.com/problems/design-tinyurl".to_string();
    let short_url = solution.encode(long_url.clone());
    assert_eq!(long_url, solution.decode(short_url));
}
