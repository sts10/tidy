mod uniquely_decodable_tests {
    use tidy::display_information::uniquely_decodable::check_decodability;

    fn make_list() -> Vec<String> {
        vec!["news", "newspaper", "paper", "elephant"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn can_determine_a_list_with_prefix_words_is_not_uniquely_decodable() {
        let list = make_list();
        assert!(!check_decodability(&list));

        let list2: Vec<String> = vec![
            "spill".to_string(),
            "sun".to_string(),
            "moved".to_string(),
            "spills".to_string(),
            "unmoved".to_string(),
        ];
        assert!(!check_decodability(&list2));
    }

    #[test]
    fn can_determine_that_a_list_is_uniquely_decodable() {
        let list: Vec<String> = vec![
            "excursion",
            "friday",
            "gyration",
            "natural",
            "pentagon",
            "sheath",
            "silver",
            "starless",
            "underling",
            "unmarked",
            "untaxed",
            "zippy",
        ]
        .iter()
        .map(|w| w.to_string())
        .collect();
        assert!(check_decodability(&list));
    }

    #[test]
    fn can_determine_binary_code_with_a_suffix_code_is_not_uniquely_decodable() {
        let list: Vec<String> = vec!["02", "12", "120", "20", "21"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(!check_decodability(&list));
    }
}
