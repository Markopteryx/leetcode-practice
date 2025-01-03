// Problem: Product of Array Except Self
// Given an integer array nums, return an array answer such that answer[i]
// is equal to the product of all elements in nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
// Must run in O(n) time without using division.
//
// Example: Input: nums = [1,2,3,4]
//         Output: [24,12,8,6]
// Example: Input: nums = [-1,1,0,-3,3]
//         Output: [0,0,9,0,0]
//
// Constraints:
// * 2 <= nums.length <= 10^5
// * -30 <= nums[i] <= 30
// * Product of any prefix or suffix fits in a 32-bit integer
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];
    let mut l = 0;
    let mut r = nums.len() - 1;

    let mut left_values = 1;
    let mut right_values = 1;
    loop {
        result[l] *= left_values;
        result[r] *= right_values;

        right_values *= nums[r];
        left_values *= nums[l];

        if r == 0 {
            break;
        }
        l += 1;
        r -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self_basic() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_product_except_self_with_zero() {
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn test_product_except_self_with_negatives() {
        assert_eq!(product_except_self(vec![-1, -1]), vec![-1, -1]);
    }

    #[test]
    fn test_product_except_self_minimum_length() {
        assert_eq!(product_except_self(vec![2, 3]), vec![3, 2]);
    }

    #[test]
    fn test_product_except_self_with_multiple_zeros() {
        assert_eq!(product_except_self(vec![0, 0, 2]), vec![0, 0, 0]);
    }

    #[test]
    fn test_product_except_self_edge_values() {
        assert_eq!(product_except_self(vec![-30, 30]), vec![30, -30]);
    }
}
