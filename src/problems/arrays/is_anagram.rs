use ahash::AHashMap;

// Problem: Valid Anagram
// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//
// Example: Input: s = "anagram", t = "nagaram", Output: true
// Example: Input: s = "rat", t = "car", Output: false
//
// Constraints
// * 1 <= s.length, t.length <= 5 * 10^4
// * s & t consist of lowercase English letters
pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = AHashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.into_values().all(|v| v == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()))
    }

    #[test]
    fn test_is_anagram_false() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn test_is_anagram_different_lengths() {
        assert!(!is_anagram("hello".to_string(), "world!".to_string()));
    }
}
