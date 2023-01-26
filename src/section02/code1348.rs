#![allow(dead_code)]

// 1348. Tweet Counts Per Frequency
// https://leetcode.com/problems/tweet-counts-per-frequency/
// https://leetcode.cn/problems/tweet-counts-per-frequency/
//
// Medium
//
// A social media company is trying to monitor activity on their site by analyzing the number of tweets that occur in select periods of time.
// These periods can be partitioned into smaller time chunks based on a certain frequency (every minute, hour, or day).
//
// For example, the period [10, 10000] (in seconds) would be partitioned into the following time chunks with these frequencies:
//
//     Every minute (60-second chunks): [10,69], [70,129], [130,189], ..., [9970,10000]
//     Every hour (3600-second chunks): [10,3609], [3610,7209], [7210,10000]
//     Every day (86400-second chunks): [10,10000]
//
// Notice that the last chunk may be shorter than the specified frequency's chunk size and will always end
// with the end time of the period (10000 in the above example).
//
// Design and implement an API to help the company with their analysis.
//
// Implement the TweetCounts class:
//
//     TweetCounts() Initializes the TweetCounts object.
//     void recordTweet(String tweetName, int time) Stores the tweetName at the recorded time (in seconds).
//     List<Integer> getTweetCountsPerFrequency(String freq, String tweetName, int startTime, int endTime) Returns a list of integers
//       representing the number of tweets with tweetName in each time chunk for the given period of time [startTime, endTime] (in seconds) and frequency freq.
//       freq is one of "minute", "hour", or "day" representing a frequency of every minute, hour, or day respectively.
//
// Example:
//
// Input
// ["TweetCounts","recordTweet","recordTweet","recordTweet","getTweetCountsPerFrequency","getTweetCountsPerFrequency","recordTweet","getTweetCountsPerFrequency"]
// [[],["tweet3",0],["tweet3",60],["tweet3",10],["minute","tweet3",0,59],["minute","tweet3",0,60],["tweet3",120],["hour","tweet3",0,210]]
//
// Output
// [null,null,null,null,[2],[2,1],null,[4]]
//
// Explanation
// TweetCounts tweetCounts = new TweetCounts();
// tweetCounts.recordTweet("tweet3", 0);                              // New tweet "tweet3" at time 0
// tweetCounts.recordTweet("tweet3", 60);                             // New tweet "tweet3" at time 60
// tweetCounts.recordTweet("tweet3", 10);                             // New tweet "tweet3" at time 10
// tweetCounts.getTweetCountsPerFrequency("minute", "tweet3", 0, 59); // return [2]; chunk [0,59] had 2 tweets
// tweetCounts.getTweetCountsPerFrequency("minute", "tweet3", 0, 60); // return [2,1]; chunk [0,59] had 2 tweets, chunk [60,60] had 1 tweet
// tweetCounts.recordTweet("tweet3", 120);                            // New tweet "tweet3" at time 120
// tweetCounts.getTweetCountsPerFrequency("hour", "tweet3", 0, 210);  // return [4]; chunk [0,210] had 4 tweets
//
// Constraints:
//
// -    0 <= time, startTime, endTime <= 10^9
// -    0 <= endTime - startTime <= 10^4
// -    There will be at most 104 calls in total to recordTweet and getTweetCountsPerFrequency.
//

use std::collections::HashMap;

struct TweetCounts {
    tweets: HashMap<String, Vec<i32>>,
}

impl TweetCounts {
    fn new() -> Self {
        Self { tweets: HashMap::new() }
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        self.tweets.entry(tweet_name).or_insert_with(Vec::new).push(time);
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let mut result = Vec::new();
        let mut chunk_size = 0;
        match freq.as_str() {
            "minute" => chunk_size = 60,
            "hour" => chunk_size = 3600,
            "day" => chunk_size = 86400,
            _ => {}
        }
        let mut chunk_start = start_time;
        let mut chunk_end = chunk_start + chunk_size - 1;
        while chunk_start <= end_time {
            if chunk_end > end_time {
                chunk_end = end_time;
            }
            let mut count = 0;
            if let Some(tweets) = self.tweets.get(&tweet_name) {
                for tweet in tweets {
                    if *tweet >= chunk_start && *tweet <= chunk_end {
                        count += 1;
                    }
                }
            }
            result.push(count);
            chunk_start += chunk_size;
            chunk_end += chunk_size;
        }
        result
    }
}

#[test]
fn test() {
    let mut tweet_counts = TweetCounts::new();
    tweet_counts.record_tweet("tweet3".to_string(), 0);
    tweet_counts.record_tweet("tweet3".to_string(), 60);
    tweet_counts.record_tweet("tweet3".to_string(), 10);
    assert_eq!(
        tweet_counts.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 59),
        vec![2]
    );
    assert_eq!(
        tweet_counts.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 60),
        vec![2, 1]
    );
    tweet_counts.record_tweet("tweet3".to_string(), 120);
    assert_eq!(
        tweet_counts.get_tweet_counts_per_frequency("hour".to_string(), "tweet3".to_string(), 0, 210),
        vec![4]
    );
}
