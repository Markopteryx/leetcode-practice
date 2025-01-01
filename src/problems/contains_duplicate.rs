use ahash::AHashSet;
use std::collections::HashSet;

// Problem: Contains Duplicate
// Given an integer array nums, return true if any value appears at least twice
// in the array, and return false if every element is distinct.
//
// Example: nums = [1,2,3,1] -> true (1 appears at indices 0 and 3)
// Example: nums = [1,2,3,4] -> false (all elements distinct)
// Example: nums = [1,1,1,3,3,4,3,2,4,2] -> true (multiple duplicates)
//
// Constraints:
// * 1 <= nums.length <= 10^5
// * -10^9 <= nums[i] <= 10^9
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let set_of_nums: HashSet<i32> = HashSet::from_iter(nums.clone());
    nums.len() != set_of_nums.len()
}

pub fn contains_duplicate_v2(nums: Vec<i32>) -> bool {
    let set: AHashSet<_> = nums.iter().collect();
    set.len() != nums.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_exists() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(contains_duplicate_v2(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_all_distinct() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(!contains_duplicate_v2(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_multiple_duplicates() {
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
        assert!(contains_duplicate_v2(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
