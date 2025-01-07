// Problem: Greatest Common Divisor of Strings
// For two strings s and t, we say "t divides s" if and only if s = t + t + ... + t
// (i.e., t is concatenated with itself one or more times).
// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.
//
// Example 1: Input: str1 = "ABCABC", str2 = "ABC"
//           Output: "ABC"
//
// Example 2: Input: str1 = "ABABAB", str2 = "ABAB"
//           Output: "AB"
//
// Example 3: Input: str1 = "LEET", str2 = "CODE"
//           Output: ""
//
// Constraints:
// * 1 <= str1.length, str2.length <= 1000
// * str1 and str2 consist of English uppercase letters
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + &str2 != str2.clone() + &str1 {
        return "".to_string();
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    let gcd_length = gcd(str1.len(), str2.len());
    str1[..gcd_length].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        assert_eq!(
            gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC"
        );
    }

    #[test]
    fn test_partial_divisor() {
        assert_eq!(
            gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB"
        );
    }

    #[test]
    fn test_no_common_divisor() {
        assert_eq!(gcd_of_strings("LEET".to_string(), "CODE".to_string()), "");
    }

    #[test]
    fn test_same_strings() {
        assert_eq!(gcd_of_strings("AAA".to_string(), "AAA".to_string()), "AAA");
    }

    #[test]
    fn test_single_char_strings() {
        assert_eq!(gcd_of_strings("A".to_string(), "A".to_string()), "A");
    }

    #[test]
    fn test_different_lengths_no_divisor() {
        assert_eq!(
            gcd_of_strings("AAAAAA".to_string(), "AAA".to_string()),
            "AAA"
        );
    }

    #[test]
    fn test_no_common_pattern() {
        assert_eq!(gcd_of_strings("ABCDEF".to_string(), "ABC".to_string()), "");
    }

    #[test]
    fn test_repeated_pattern() {
        assert_eq!(
            gcd_of_strings(
                "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
                "TAUXXTAUXXTAUXX".to_string()
            ),
            "TAUXX"
        );
    }
}
