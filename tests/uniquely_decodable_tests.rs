mod uniquely_decodable_tests {
    use tidy::display_information::uniquely_decodable::is_uniquely_decodable;

    #[test]
    fn can_determine_a_list_with_prefix_words_is_not_uniquely_decodable() {
        let list: Vec<String> = vec!["news", "newspaper", "paper", "elephant"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert!(!is_uniquely_decodable(&list));

        let list2: Vec<String> = vec![
            "spill".to_string(),
            "sun".to_string(),
            "moved".to_string(),
            "spills".to_string(),
            "unmoved".to_string(),
        ];
        assert!(!is_uniquely_decodable(&list2));
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
        assert!(is_uniquely_decodable(&list));
    }

    #[test]
    fn can_determine_binary_code_with_a_suffix_code_is_not_uniquely_decodable() {
        let list: Vec<String> = vec!["02", "12", "120", "20", "21"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(!is_uniquely_decodable(&list));
    }

    #[test]
    fn given_a_series_of_binary_codes_can_determine_which_are_uniquely_decodable() {
        let list: Vec<String> = vec!["0", "10", "110", "111"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(is_uniquely_decodable(&list));

        let list: Vec<String> = vec!["0", "10", "010", "101"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(!is_uniquely_decodable(&list));

        let list: Vec<String> = vec!["0", "01", "011", "0111"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(is_uniquely_decodable(&list));

        // '0, 1, 00, 11' is not an uniquely decodable code
        let list: Vec<String> = vec!["0", "1", "00", "11"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(!is_uniquely_decodable(&list));
    }

    #[test]
    fn knows_that_a_fixed_length_code_is_uniquely_decodable() {
        let list: Vec<String> = vec![
            "buoy", "cote", "dads", "duel", "gale", "life", "lurk", "peer", "rain", "tong",
        ]
        .iter()
        .map(|w| w.to_string())
        .collect();
        assert!(is_uniquely_decodable(&list));
    }
}
