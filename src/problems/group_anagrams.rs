use ahash::AHashMap;

// Problem: Group Anagrams
// Given an array of strings, group all anagrams together. Return the groups in any order.
// An anagram is a word formed by rearranging letters of another word.
//
// Example: Input: strs = ["eat","tea","tan","ate","nat","bat"]
//         Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
// Example: Input: strs = [""] -> [[""]]
// Example: Input: strs = ["a"] -> [["a"]]
//
// Constraints:
// * 1 <= strs.length <= 10^4
// * 0 <= strs[i].length <= 100
// * strs[i] consists of lowercase English letters
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result = AHashMap::new();

    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        result.entry(key).or_insert(vec![]).push(s);
    }

    result.values().cloned().collect()
}

pub fn group_anagrams_v2(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: AHashMap<[u8; 26], Vec<String>> = AHashMap::with_capacity(strs.len());
    let offset = 'a' as usize;

    for s in strs.into_iter() {
        let mut chars: [u8; 26] = [0; 26];

        for char in s.chars() {
            chars[char.to_ascii_lowercase() as usize - offset] += 1;
        }

        result
            .entry(chars)
            .and_modify(|c| c.push(s.clone()))
            .or_insert(vec![s]);
    }

    result.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams_basic() {
        let input: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(String::from)
            .collect();

        let mut result_v1 = group_anagrams(input.clone());
        let mut result_v2 = group_anagrams_v2(input);

        for group in &mut result_v1 {
            group.sort();
        }
        for group in &mut result_v2 {
            group.sort();
        }
        result_v1.sort();
        result_v2.sort();

        let mut expected = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];

        for group in &mut expected {
            group.iter_mut().for_each(|s| *s = s);
            group.sort();
        }
        expected.sort();

        assert_eq!(result_v1, expected);
        assert_eq!(result_v2, expected);
    }

    #[test]
    fn test_group_anagrams_empty_string() {
        let input = vec![""].into_iter().map(String::from).collect();
        let result = group_anagrams(input);
        assert_eq!(result, vec![vec!["".to_string()]]);
    }

    #[test]
    fn test_group_anagrams_single_char() {
        let input = vec!["a"].into_iter().map(String::from).collect();
        let result = group_anagrams(input);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }

    #[test]
    fn test_group_anagrams_no_groups() {
        let input = vec!["abc", "def", "ghi"]
            .into_iter()
            .map(String::from)
            .collect();

        let mut result = group_anagrams(input);

        for group in &mut result {
            group.sort();
        }
        result.sort();

        assert_eq!(
            result,
            vec![
                vec!["abc".to_string()],
                vec!["def".to_string()],
                vec!["ghi".to_string()]
            ]
        );
    }
}
