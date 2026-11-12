use dsa::arrays::two_sum::two_sum;

#[test]
fn integration_two_sum() {
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(two_sum(&[1, 2], 100), None);
}
