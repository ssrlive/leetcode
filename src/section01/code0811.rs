#![allow(dead_code)]

// 811. Subdomain Visit Count
// https://leetcode.com/problems/subdomain-visit-count/
// https://leetcode.cn/problems/subdomain-visit-count/
//
// A website domain "discuss.leetcode.com" consists of various subdomains. At the top level, we have "com", at the next level, we have "leetcode.com" and at the lowest level,
// "discuss.leetcode.com". When we visit a domain like "discuss.leetcode.com", we will also visit the parent domains "leetcode.com" and "com" implicitly.
//
// A count-paired domain is a domain that has one of the two formats "rep d1.d2.d3" or "rep d1.d2" where rep is the number of visits to the domain and d1.d2.d3 is the domain itself.
//
// - For example, "9001 discuss.leetcode.com" is a count-paired domain that indicates that discuss.leetcode.com was visited 9001 times.
//
// Given an array of count-paired domains cpdomains, return an array of the count-paired domains of each subdomain in the input. You may return the answer in any order.
//
// Example 1:
//
// Input: cpdomains = ["9001 discuss.leetcode.com"]
// Output: ["9001 leetcode.com","9001 discuss.leetcode.com","9001 com"]
// Explanation: We only have one website domain: "discuss.leetcode.com".
// As discussed above, the subdomain "leetcode.com" and "com" will also be visited. So they will all be visited 9001 times.
//
// Example 2:
//
// Input: cpdomains = ["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"]
// Output: ["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1 intel.mail.com","951 com"]
// Explanation: We will visit "google.mail.com" 900 times, "yahoo.com" 50 times, "intel.mail.com" once and "wiki.org" 5 times.
// For the subdomains, we will visit "mail.com" 900 + 1 = 901 times, "com" 900 + 50 + 1 = 951 times, and "org" 5 times.
//
// Constraints:
//
// - 1 <= cpdomain.length <= 100
// - 1 <= cpdomain[i].length <= 100
// - cpdomain[i] follows either the "rep[i] d1[i].d2[i].d3[i]" format or the "rep[i] d1[i].d2[i]" format.
// - repi is an integer in the range [1, 10^4].
// - d1[i], d2[i], and d3[i] consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        fn _subdomain_visits(cpdomains: Vec<String>) -> Option<Vec<String>> {
            let mut map = std::collections::HashMap::new();
            for cpdomain in cpdomains {
                let mut iter = cpdomain.split_ascii_whitespace();
                let count = iter.next()?.parse::<i32>().ok()?;
                let mut domain = iter.next()?.to_string();
                loop {
                    *map.entry(domain.clone()).or_insert(0) += count;
                    if let Some(index) = domain.find('.') {
                        domain = domain[index + 1..].to_string();
                    } else {
                        break;
                    }
                }
            }
            let f = |(domain, count): (&String, &i32)| format!("{count} {domain}");
            let v = map.iter().map(f).collect();
            Some(v)
        }

        _subdomain_visits(cpdomains).unwrap_or_default()
    }
}

#[test]
fn test() {
    let cpdomains = vec!["9001 discuss.leetcode.com".to_string()];
    let expected = vec![
        "9001 com".to_string(),
        "9001 discuss.leetcode.com".to_string(),
        "9001 leetcode.com".to_string(),
    ];
    let mut result = Solution::subdomain_visits(cpdomains);
    result.sort();
    assert_eq!(result, expected);

    let cpdomains = vec![
        "900 google.mail.com".to_string(),
        "50 yahoo.com".to_string(),
        "1 intel.mail.com".to_string(),
        "5 wiki.org".to_string(),
    ];
    let expected = vec![
        "1 intel.mail.com".to_string(),
        "5 org".to_string(),
        "5 wiki.org".to_string(),
        "50 yahoo.com".to_string(),
        "900 google.mail.com".to_string(),
        "901 mail.com".to_string(),
        "951 com".to_string(),
    ];
    let mut result = Solution::subdomain_visits(cpdomains);
    result.sort();
    assert_eq!(result, expected);
}
