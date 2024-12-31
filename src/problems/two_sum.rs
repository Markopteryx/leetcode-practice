use std::collections::HashMap;

// Problem: Two Sum
// Given an array of integers nums and a target, return indices of two numbers that sum to target.
// Each input has exactly one solution and same element cannot be used twice.
// Answer can be returned in any order.
//
// Example: nums = [2,7,11,15], target = 9 -> [0,1] (nums[0] + nums[1] = 9)
// Example: nums = [3,2,4], target = 6 -> [1,2]
// Example: nums = [3,3], target = 6 -> [0,1]
//
// Constraints:
// * 2 <= nums.length <= 10^4
// * -10^9 <= nums[i] <= 10^9
// * -10^9 <= target <= 10^9
// * Only one valid answer exists
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_to_index = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = num_to_index.get(&complement) {
            return vec![complement_index as i32, i as i32];
        }
        num_to_index.insert(num, i);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_different_order() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_same_number() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
