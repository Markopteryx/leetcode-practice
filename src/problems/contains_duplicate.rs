use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let set_of_nums: HashSet<i32> = HashSet::from_iter(nums.clone());
    nums.len() != set_of_nums.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]))
    }
}
