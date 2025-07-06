#![allow(dead_code)]

/*

// 1797. Design Authentication Manager
Medium
254
37
Companies

There is an authentication system that works with authentication tokens. For each session, the user will receive a new authentication token that will expire timeToLive seconds after the currentTime. If the token is renewed, the expiry time will be extended to expire timeToLive seconds after the (potentially different) currentTime.

Implement the AuthenticationManager class:

    AuthenticationManager(int timeToLive) constructs the AuthenticationManager and sets the timeToLive.
    generate(string tokenId, int currentTime) generates a new token with the given tokenId at the given currentTime in seconds.
    renew(string tokenId, int currentTime) renews the unexpired token with the given tokenId at the given currentTime in seconds. If there are no unexpired tokens with the given tokenId, the request is ignored, and nothing happens.
    countUnexpiredTokens(int currentTime) returns the number of unexpired tokens at the given currentTime.

Note that if a token expires at time t, and another action happens on time t (renew or countUnexpiredTokens), the expiration takes place before the other actions.

Example 1:

Input
["AuthenticationManager", "renew", "generate", "countUnexpiredTokens", "generate", "renew", "renew", "countUnexpiredTokens"]
[[5], ["aaa", 1], ["aaa", 2], [6], ["bbb", 7], ["aaa", 8], ["bbb", 10], [15]]
Output
[null, null, null, 1, null, null, null, 0]

Explanation
AuthenticationManager authenticationManager = new AuthenticationManager(5); // Constructs the AuthenticationManager with timeToLive = 5 seconds.
authenticationManager.renew("aaa", 1); // No token exists with tokenId "aaa" at time 1, so nothing happens.
authenticationManager.generate("aaa", 2); // Generates a new token with tokenId "aaa" at time 2.
authenticationManager.countUnexpiredTokens(6); // The token with tokenId "aaa" is the only unexpired one at time 6, so return 1.
authenticationManager.generate("bbb", 7); // Generates a new token with tokenId "bbb" at time 7.
authenticationManager.renew("aaa", 8); // The token with tokenId "aaa" expired at time 7, and 8 >= 7, so at time 8 the renew request is ignored, and nothing happens.
authenticationManager.renew("bbb", 10); // The token with tokenId "bbb" is unexpired at time 10, so the renew request is fulfilled and now the token will expire at time 15.
authenticationManager.countUnexpiredTokens(15); // The token with tokenId "bbb" expires at time 15, and the token with tokenId "aaa" expired at time 7, so currently no token is unexpired, so return 0.

Constraints:

    1 <= timeToLive <= 10^8
    1 <= currentTime <= 10^8
    1 <= tokenId.length <= 5
    tokenId consists only of lowercase letters.
    All calls to generate will contain unique values of tokenId.
    The values of currentTime across all the function calls will be strictly increasing.
    At most 2000 calls will be made to all functions combined.
*/

use std::collections::HashMap;

struct AuthenticationManager {
    time_to_live: i32,
    tokens: HashMap<String, i32>,
}

impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self {
            time_to_live,
            tokens: HashMap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.tokens.insert(token_id, current_time + self.time_to_live);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(expiry) = self.tokens.get_mut(&token_id)
            && *expiry > current_time
        {
            *expiry = current_time + self.time_to_live;
        }
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        self.tokens.values().filter(|&&expiry| expiry > current_time).count() as i32
    }
}

#[test]
fn test() {
    let mut authentication_manager = AuthenticationManager::new(5);
    authentication_manager.renew("aaa".to_string(), 1);
    authentication_manager.generate("aaa".to_string(), 2);
    assert_eq!(authentication_manager.count_unexpired_tokens(6), 1);
    authentication_manager.generate("bbb".to_string(), 7);
    authentication_manager.renew("aaa".to_string(), 8);
    authentication_manager.renew("bbb".to_string(), 10);
    assert_eq!(authentication_manager.count_unexpired_tokens(15), 0);
}
