#![allow(dead_code)]

/*

// 1472. Design Browser History
// https://leetcode.com/problems/design-browser-history/
// https://leetcode.cn/problems/design-browser-history/
//
// Medium
//
You have a browser of one tab where you start on the homepage and you can visit another url, get back in the history number of steps or move forward in the history number of steps.

Implement the BrowserHistory class:

    BrowserHistory(string homepage) Initializes the object with the homepage of the browser.
    void visit(string url) Visits url from the current page. It clears up all the forward history.
    string back(int steps) Move steps back in history. If you can only return x steps in the history and steps > x, you will return only x steps. Return the current url after moving back in history at most steps.
    string forward(int steps) Move steps forward in history. If you can only forward x steps in the history and steps > x, you will forward only x steps. Return the current url after forwarding in history at most steps.

Example:

Input:
["BrowserHistory","visit","visit","visit","back","back","forward","visit","forward","back","back"]
[["leetcode.com"],["google.com"],["facebook.com"],["youtube.com"],[1],[1],[1],["linkedin.com"],[2],[2],[7]]
Output:
[null,null,null,null,"facebook.com","google.com","facebook.com",null,"linkedin.com","google.com","leetcode.com"]

Explanation:
BrowserHistory browserHistory = new BrowserHistory("leetcode.com");
browserHistory.visit("google.com");       // You are in "leetcode.com". Visit "google.com"
browserHistory.visit("facebook.com");     // You are in "google.com". Visit "facebook.com"
browserHistory.visit("youtube.com");      // You are in "facebook.com". Visit "youtube.com"
browserHistory.back(1);                   // You are in "youtube.com", move back to "facebook.com" return "facebook.com"
browserHistory.back(1);                   // You are in "facebook.com", move back to "google.com" return "google.com"
browserHistory.forward(1);                // You are in "google.com", move forward to "facebook.com" return "facebook.com"
browserHistory.visit("linkedin.com");     // You are in "facebook.com". Visit "linkedin.com"
browserHistory.forward(2);                // You are in "linkedin.com", you cannot move forward any steps.
browserHistory.back(2);                   // You are in "linkedin.com", move back two steps to "facebook.com" then to "google.com". return "google.com"
browserHistory.back(7);                   // You are in "google.com", you can move back only one step to "leetcode.com". return "leetcode.com"

Constraints:

    1 <= homepage.length <= 20
    1 <= url.length <= 20
    1 <= steps <= 100
    homepage and url consist of  '.' or lower case English letters.
    At most 5000 calls will be made to visit, back, and forward.
*/

struct BrowserHistory {
    stack: Vec<String>,
    ci: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            stack: vec![homepage],
            ci: 0,
        }
    }

    fn visit(&mut self, url: String) {
        for _ in self.ci + 1..self.stack.len() {
            self.stack.pop();
        }
        self.stack.push(url);
        self.ci += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.ci = self.ci.saturating_sub(steps as usize);
        self.stack[self.ci].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let ni = std::cmp::min(self.stack.len() - 1, self.ci + steps as usize);
        self.ci = ni;
        self.stack[self.ci].clone()
    }
}

#[test]
fn test() {
    let mut bh = BrowserHistory::new("leetcode.com".to_string());
    bh.visit("google.com".to_string());
    bh.visit("facebook.com".to_string());
    bh.visit("youtube.com".to_string());
    assert_eq!(bh.back(1), "facebook.com".to_string());
    assert_eq!(bh.back(1), "google.com".to_string());
    assert_eq!(bh.forward(1), "facebook.com".to_string());
    bh.visit("linkedin.com".to_string());
    assert_eq!(bh.forward(2), "linkedin.com".to_string());
    assert_eq!(bh.back(2), "google.com".to_string());
    assert_eq!(bh.back(7), "leetcode.com".to_string());
}
