// Problem: Can Place Flowers
//
// Given a flowerbed represented as an array of 0s and 1s where:
// - 0 means the plot is empty
// - 1 means the plot has a flower
// Return true if n new flowers can be planted without violating the no-adjacent-flowers rule
//
// Key points:
// - No two flowers can be in adjacent plots
// - Edge plots follow same rules (e.g. [1,0,0,1] - middle plots can't have flowers)
// - Only need to verify if n flowers CAN be planted, not how to plant them
// - Input array already follows no-adjacent rule
//
// Example:
// flowerbed = [1,0,0,0,1], n = 1
// Index 0: Has flower
// Index 1: Adjacent to flower, can't plant
// Index 2: Empty with no adjacent flowers, can plant
// Index 3: Adjacent to flower, can't plant
// Index 4: Has flower
// Result: true (can plant 1 flower at index 2)
//
// Constraints:
// * 1 <= flowerbed.length <= 2 * 10^4
// * flowerbed[i] is 0 or 1
// * No two adjacent flowers in input
// * 0 <= n <= flowerbed.length
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut count = 0;
    let mut fb = flowerbed.clone();

    for i in 0..fb.len() {
        let prev = if i == 0 { 0 } else { fb[i - 1] };
        let next = if i == fb.len() - 1 { 0 } else { fb[i + 1] };

        if fb[i] == 0 && prev == 0 && next == 0 {
            fb[i] = 1;
            count += 1;
        }
    }

    count >= n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn test_example_2() {
        assert!(!can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn test_minimum_length() {
        assert!(can_place_flowers(vec![0], 1));
        assert!(can_place_flowers(vec![1], 0));
        assert!(!can_place_flowers(vec![0], 2));
    }

    #[test]
    fn test_no_flowers_needed() {
        assert!(can_place_flowers(vec![1, 0, 1], 0));
        assert!(can_place_flowers(vec![0, 0, 0], 0));
    }

    #[test]
    fn test_all_empty() {
        assert!(can_place_flowers(vec![0, 0, 0, 0, 0], 3));
        assert!(can_place_flowers(vec![0, 0, 0, 0], 2));
        assert!(can_place_flowers(vec![0, 0, 0], 2));
    }

    #[test]
    fn test_all_planted() {
        assert!(!can_place_flowers(vec![1, 0, 1, 0, 1], 1));
    }

    #[test]
    fn test_edges() {
        assert!(can_place_flowers(vec![0, 0, 1], 1));
        assert!(can_place_flowers(vec![1, 0, 0], 1));
        assert!(can_place_flowers(vec![0, 0, 1, 0, 0], 2));
    }

    #[test]
    fn test_maximum_n() {
        assert!(!can_place_flowers(vec![0, 0, 0, 0], 4));
        assert!(can_place_flowers(vec![0, 0, 0, 0, 0], 3));
    }

    #[test]
    fn test_longer_sequence() {
        assert!(can_place_flowers(vec![1, 0, 0, 0, 0, 1, 0, 0], 2));
        assert!(!can_place_flowers(vec![1, 0, 0, 0, 0, 1, 0, 0], 3));
    }
}
