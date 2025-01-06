// Problem: Merge Strings Alternately
// Given two strings word1 and word2, merge them by adding letters in alternating order,
// starting with word1. If a string is longer than the other, append the additional
// letters onto the end of the merged string.
//
// Example: Input: word1 = "abc", word2 = "pqr"
//         Output: "apbqcr"
//         Explanation: word1: a b c
//                     word2:  p q r
//                     merged: apbqcr
// Example: Input: word1 = "ab", word2 = "pqrs"
//         Output: "apbqrs"
// Example: Input: word1 = "abcd", word2 = "pq"
//         Output: "apbqcd"
//
// Constraints:
// * 1 <= word1.length, word2.length <= 100
// * word1 and word2 consist of lowercase English letters
pub fn merge_alternately(word1: String, word2: String) -> String {
    if word1.is_empty() {
        return word2;
    }

    if word2.is_empty() {
        return word1;
    }

    let mut result = String::new();
    for (c1, c2) in word1.chars().zip(word2.chars()) {
        result.push(c1);
        result.push(c2);
    }

    match word1.len().cmp(&word2.len()) {
        std::cmp::Ordering::Greater => result.push_str(&word1[word2.len()..]),
        std::cmp::Ordering::Less => result.push_str(&word2[word1.len()..]),
        std::cmp::Ordering::Equal => {}
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately_equal_length() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr"
        );
    }

    #[test]
    fn test_merge_alternately_first_shorter() {
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs"
        );
    }

    #[test]
    fn test_merge_alternately_second_shorter() {
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd"
        );
    }

    #[test]
    fn test_merge_alternately_minimum_length() {
        assert_eq!(merge_alternately("a".to_string(), "b".to_string()), "ab");
    }

    #[test]
    fn test_merge_alternately_one_empty() {
        assert_eq!(merge_alternately("abc".to_string(), "".to_string()), "abc");
    }

    #[test]
    fn test_merge_alternately_same_letters() {
        assert_eq!(
            merge_alternately("aaa".to_string(), "bbb".to_string()),
            "ababab"
        );
    }

    #[test]
    fn test_merge_alternately_special_case() {
        assert_eq!(
            merge_alternately("z".to_string(), "xyz".to_string()),
            "zxyz"
        );
    }
}
