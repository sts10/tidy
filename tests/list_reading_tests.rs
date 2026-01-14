mod list_reading_tests {
    use crate::list_reading_tests::file_readers::blend_lists;
    use crate::list_reading_tests::file_readers::concatenate_lists;
    use tidy::*;

    #[test]
    fn can_blend_multiple_lists() {
        let word_lists_by_file = vec![
            vec!["one".to_string(), "three".to_string(), "five".to_string()],
            vec![
                "two".to_string(),
                "four".to_string(),
                "six".to_string(),
                "eight".to_string(),
                "ten".to_string(),
            ],
        ];
        let blended_list = blend_lists(&word_lists_by_file);

        assert_eq!(
            blended_list,
            ["one", "two", "three", "four", "five", "six", "eight", "ten"].to_vec()
        );
    }

    #[test]
    fn can_concatenate_multiple_lists() {
        let word_lists_by_file = vec![
            vec!["one".to_string(), "three".to_string(), "five".to_string()],
            vec![
                "two".to_string(),
                "four".to_string(),
                "six".to_string(),
                "eight".to_string(),
                "ten".to_string(),
            ],
        ];
        let concatenated_list = concatenate_lists(word_lists_by_file);

        assert_eq!(
            concatenated_list,
            ["one", "three", "five", "two", "four", "six", "eight", "ten"].to_vec()
        );
    }
}
