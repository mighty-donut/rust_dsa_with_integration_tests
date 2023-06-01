use rust_dsa_with_integration_tests::sorting::merge_sort::merge_sort_v2::merge;

#[test]
fn merge_sort_v2_test() {
    let sorted_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let unsorted_numbers = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
    assert_eq!(sorted_numbers, merge(&unsorted_numbers));

    let sorted_strings = vec!["airplane", "art", "beach", "car", "hotel", "house"];
    let unsorted_strings = vec!["beach", "hotel", "airplane", "car", "house", "art"];
    assert_eq!(sorted_strings, merge(&unsorted_strings));
}
