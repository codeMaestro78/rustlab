/// Returns indices of the first pair that sums to `target`, if one exists.
///
/// Pairs are considered in index order, and an element cannot be used twice.
/// This brute-force approach takes O(n²) time and O(1) extra space.
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            // Widen before adding so extreme i32 inputs cannot overflow.
            if i64::from(nums[i]) + i64::from(nums[j]) == i64::from(target) {
                return Some((i, j));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn finds_a_pair() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum(&[1, 4, 6], 10), Some((1, 2)));
    }

    #[test]
    fn handles_duplicates_and_distinct_indices() {
        assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
        assert_eq!(two_sum(&[3], 6), None);
    }

    #[test]
    fn handles_no_pair_and_empty_input() {
        assert_eq!(two_sum(&[1, 2], 100), None);
        assert_eq!(two_sum(&[], 0), None);
    }

    #[test]
    fn handles_negative_values_and_multiple_pairs() {
        assert_eq!(two_sum(&[-3, 4, 3, 0], 0), Some((0, 2)));
        assert_eq!(two_sum(&[1, 2, 3, 4], 5), Some((0, 3)));
    }

    #[test]
    fn handles_extreme_integers_without_overflow() {
        assert_eq!(
            two_sum(&[i32::MAX, i32::MAX, -1], i32::MAX - 1),
            Some((0, 2))
        );
        assert_eq!(two_sum(&[i32::MIN, i32::MIN], 0), None);
        assert_eq!(two_sum(&[i32::MIN, i32::MAX], -1), Some((0, 1)));
    }
}
