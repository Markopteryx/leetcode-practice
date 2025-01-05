use ahash::AHashSet;

// Problem: Longest Consecutive Sequence
// Given an unsorted array of integers nums, return the length of the longest
// consecutive elements sequence.
// Must write an algorithm that runs in O(n) time.
//
// Example: Input: nums = [100,4,200,1,3,2]
//         Output: 4
//         Explanation: Longest consecutive sequence is [1,2,3,4]
// Example: Input: nums = [0,3,7,2,5,8,4,6,0,1]
//         Output: 9
//
// Constraints:
// * 0 <= nums.length <= 10^5
// * -10^9 <= nums[i] <= 10^9
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: AHashSet<_> = nums.iter().collect();
    set.iter()
        .filter(|&&x| !set.contains(&(x - 1)))
        .map(|&&x| (x..).take_while(|x| set.contains(x)).count())
        .max()
        .unwrap_or(0) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive_basic() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_longest_consecutive_with_longer_sequence() {
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }

    #[test]
    fn test_longest_consecutive_empty() {
        assert_eq!(longest_consecutive(vec![]), 0);
    }

    #[test]
    fn test_longest_consecutive_single_element() {
        assert_eq!(longest_consecutive(vec![1]), 1);
    }

    #[test]
    fn test_longest_consecutive_duplicate_elements() {
        assert_eq!(longest_consecutive(vec![1, 2, 0, 1]), 3);
    }

    #[test]
    fn test_longest_consecutive_negative_numbers() {
        assert_eq!(longest_consecutive(vec![-2, -3, -4, 100, 4]), 3);
    }

    #[test]
    fn test_longest_consecutive_no_sequence() {
        assert_eq!(longest_consecutive(vec![2, 4, 6, 8]), 1);
    }

    #[test]
    fn test_longest_consecutive_large_numbers() {
        assert_eq!(longest_consecutive(vec![1000000000, -1000000000]), 1);
    }
}
