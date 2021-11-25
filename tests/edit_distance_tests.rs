mod edit_distance_tests {
    use tidy::edit_distance::find_edit_distance;

    #[test]
    fn equal_strings() {
        assert_eq!(0, find_edit_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, find_edit_distance("Test_Case_#1", "Test_Case_#1"));
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(1, find_edit_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, find_edit_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, find_edit_distance("Test_Case_#1", "Test_Case_#10"));
    }

    #[test]
    fn several_differences() {
        assert_eq!(2, find_edit_distance("My Cat", "My Case"));
        assert_eq!(7, find_edit_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, find_edit_distance("Test_Case_#3", "Case #3"))
    }
}
