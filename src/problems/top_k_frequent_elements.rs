use ahash::AHashMap;

// Problem: Top K Frequent Elements
// Given an array of integers and a number k, find the k most frequent elements in the array.
// Return these elements in any order.
//
// Example: Input: nums = [1,1,1,2,2,3], k = 2
//         Output: [1,2]
//         Explanation: 1 appears 3 times, 2 appears 2 times, and 3 appears once
//                     So the 2 most frequent elements are 1 and 2
//
// Example: Input: nums = [1], k = 1
//         Output: [1]
//         Explanation: Only one element exists, so it's the most frequent
//
// Constraints:
// * 1 <= nums.length <= 105
// * -104 <= nums[i] <= 104
// * k is in range [1, number of unique elements]
// * The answer is guaranteed to be unique
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map = AHashMap::new();

    for num in nums {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let mut freq_vec: Vec<_> = freq_map.into_iter().collect();
    freq_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    freq_vec
        .into_iter()
        .take(k as usize)
        .map(|(num, _)| num)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        assert_eq!(
            top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
                .into_iter()
                .collect::<std::collections::HashSet<_>>(),
            vec![1, 2].into_iter().collect()
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(top_k_frequent(vec![1, 2, 3, 4], 2).len(), 2);
    }

    #[test]
    fn test_same_frequency() {
        // When elements have same frequency, any order is acceptable
        let result = top_k_frequent(vec![1, 1, 2, 2, 3, 3], 2);
        assert_eq!(result.len(), 2);
        assert!(result.into_iter().all(|x| [1, 2, 3].contains(&x)));
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(
            top_k_frequent(vec![-1, -1, -1, 2, 2, 3], 2)
                .into_iter()
                .collect::<std::collections::HashSet<_>>(),
            vec![-1, 2].into_iter().collect()
        );
    }

    #[test]
    fn test_k_equals_unique_count() {
        let nums = vec![1, 1, 2, 2, 3];
        let k = 3;
        let result = top_k_frequent(nums, k);
        assert_eq!(result.len(), 3);
        assert!(result.into_iter().all(|x| [1, 2, 3].contains(&x)));
    }
}
