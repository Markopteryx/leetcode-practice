// Problem: Kids With the Greatest Number of Candies
//
// Given an array of candies where candies[i] represents the number of candies
// the ith kid has, and an integer extraCandies, return a boolean array result where:
// - result[i] is true if giving all extraCandies to kid i would result in them having
//   the greatest number of candies among all kids
// - result[i] is false otherwise
//
// Key points:
// - Multiple kids can have the greatest number of candies
// - Each kid gets all extraCandies when calculating their potential maximum
// - Must compare each kid's potential total against all other kids' current amounts
//
// Example:
// candies = [2,3,5,1,3], extraCandies = 3
// Kid 1: 2+3=5 candies (true, equals max of 5)
// Kid 2: 3+3=6 candies (true, exceeds max of 5)
// Kid 3: 5+3=8 candies (true, exceeds max of 5)
// Kid 4: 1+3=4 candies (false, below max of 5)
// Kid 5: 3+3=6 candies (true, exceeds max of 5)
//
// Constraints:
// * n == candies.length (2 <= n <= 100)
// * 1 <= candies[i] <= 100
// * 1 <= extraCandies <= 50

pub fn kids_with_candies(_candies: Vec<i32>, _extra_candies: i32) -> Vec<bool> {
    // TODO: Implement solution
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }

    #[test]
    fn test_minimum_length() {
        // Test with minimum allowed length (n=2)
        assert_eq!(kids_with_candies(vec![1, 2], 1), vec![true, true]);
    }

    #[test]
    fn test_all_equal() {
        // When all kids have same candies, all should be true with any extra
        assert_eq!(
            kids_with_candies(vec![5, 5, 5, 5], 1),
            vec![true, true, true, true]
        );
    }

    #[test]
    fn test_maximum_constraints() {
        // Test with maximum allowed values
        // Max candies = 100, max extra = 50
        assert_eq!(kids_with_candies(vec![100, 98], 50), vec![true, true]);
    }

    #[test]
    fn test_no_effect() {
        // Test where extra candies don't change the relative ordering
        assert_eq!(kids_with_candies(vec![10, 1], 1), vec![true, false]);
    }

    #[test]
    fn test_single_maximum() {
        // Test where only one kid can reach the maximum
        assert_eq!(
            kids_with_candies(vec![1, 10, 1], 8),
            vec![false, true, false]
        );
    }

    #[test]
    fn test_minimum_extra_candies() {
        // Test with minimum extra candies (1)
        assert_eq!(kids_with_candies(vec![5, 4, 3], 1), vec![true, true, false]);
    }

    #[test]
    fn test_large_differences() {
        // Test with large differences between kids
        assert_eq!(
            kids_with_candies(vec![1, 50, 100, 25], 50),
            vec![false, true, true, false]
        );
    }
}
