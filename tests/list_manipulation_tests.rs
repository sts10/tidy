mod list_manipulation_tests {
    use tidy::*;

    fn make_lists() -> (Vec<String>, Vec<String>) {
        (
            vec![
                "  zookeeper",
                "apple",
                "cHarLie",
                "app",
                "tea",
                "addiction",
                "zoo",
                "stationary ",
                "tea",
                "station",
                "apple",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
            vec![
                "  wizard  ",
                "ardoR",
                "tea",
                "   ",
                "11225	active",
                "11152	acclaim",
                "11156	word	tabs",
                "19-6-8 clad",
                "be",
                "I",
                "vAcation",
                "take",
                "world999",
                "",
                "mistake",
                "tee",
                "post-modern",
                "13910 word with spaces in it",
                "  h as spaces ",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
        )
    }

    #[test]
    fn can_remove_duplicate_words() {
        let this_tidy_request = TidyRequest {
            list: make_lists().0,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"tea".to_string()));
        assert!(new_list.contains(&"apple".to_string()));
        assert!(new_list.len() == make_lists().0.len() - 2);
    }

    #[test]
    fn can_sort_words_alphabetically() {
        let this_tidy_request = TidyRequest {
            list: make_lists().0,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list[0] == "addiction".to_string());
        assert!(new_list.contains(&"station".to_string()));
        assert!(new_list[new_list.len() - 1] == "zookeeper".to_string());
    }

    #[test]
    fn removes_blank_lines() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.len() == make_lists().1.len() - 2);
    }

    #[test]
    fn removes_starting_and_trailing_whitespace() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"wizard".to_string()));
    }

    #[test]
    fn does_not_remove_inner_spaces() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"h as spaces".to_string()));
    }

    #[test]
    fn can_delete_integers_from_words() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_integers: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"active".to_string()));
    }

    #[test]
    fn can_delete_nonalphanumeric_from_words() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_nonalphanumeric: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"1968clad".to_string()));
        assert!(new_list.contains(&"take".to_string()));
    }

    #[test]
    fn can_delete_through_first_tab() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_through_first_tab: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"active".to_string()));
        assert!(new_list.contains(&"acclaim".to_string()));
        // Only remove through FIRST tab
        assert!(new_list.contains(&"word\ttabs".to_string()));
    }

    #[test]
    fn can_delete_through_first_space() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_through_first_space: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"clad".to_string()));
        // Check that it only removes characters through first space, rather than just
        // between first space and second space, for example
        assert!(new_list.contains(&"word with spaces in it".to_string()));
        // If has a leading space, just removes the first leading space
        // Tidy trims whitespace AFTER the delete_through_character function,
        // so that doesn't affect this example
        // (Maybe should trim leading whitespace first though...)
        assert!(new_list.contains(&"h as spaces".to_string()));
    }

    #[test]
    fn can_lowercase_words() {
        let this_tidy_request = TidyRequest {
            list: make_lists().0,
            to_lowercase: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"charlie".to_string()));
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            to_lowercase: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"vacation".to_string()));
        assert!(new_list.contains(&"ardor".to_string()));
    }

    #[test]
    fn can_remove_prefix_words() {
        let this_tidy_request = TidyRequest {
            list: make_lists().0,
            should_remove_prefix_words: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"station".to_string()));
        assert!(new_list.contains(&"stationary".to_string()));
        assert!(!new_list.contains(&"zoo".to_string()));
        assert!(new_list.contains(&"zookeeper".to_string()));
        assert!(new_list.contains(&"apple".to_string()));
    }

    #[test]
    fn can_remove_words_with_nonalphanumeric_characters() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_remove_nonalphanumeric: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"19-6-8 clad".to_string()));
        assert!(new_list.contains(&"world999".to_string()));
        assert!(new_list.contains(&"take".to_string()));
    }

    #[test]
    fn can_remove_words_with_integers() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_remove_integers: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"19-6-8 clad".to_string()));
        assert!(!new_list.contains(&"world999".to_string()));
        assert!(new_list.contains(&"be".to_string()));
        assert!(new_list.contains(&"I".to_string()));
    }
    #[test]
    fn can_remove_words_shorter_than_a_specified_minimum_length() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            minimum_length: Some(3),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"I".to_string()));
        assert!(!new_list.contains(&"be".to_string()));
        assert!(new_list.contains(&"tea".to_string()));
        assert!(new_list.contains(&"mistake".to_string()));
    }

    #[test]
    fn can_remove_reject_words() {
        let words_to_reject: Vec<String> = vec!["mistake", "carnival"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            reject_list: Some(words_to_reject),
            to_lowercase: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"mistake".to_string()));
        assert!(!new_list.contains(&"carnival".to_string()));
        assert!(new_list.contains(&"wizard".to_string()));
    }

    #[test]
    fn can_remove_all_words_not_on_approved_list_words() {
        let approved_words: Vec<String> = vec!["take", "vacation", "airplane"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            approved_list: Some(approved_words),
            to_lowercase: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"take".to_string()));
        assert!(new_list.contains(&"vacation".to_string()));
        assert!(!new_list.contains(&"carnival".to_string()));
        assert!(!new_list.contains(&"wizard".to_string()));
        assert!(!new_list.contains(&"airplane".to_string()));
    }

    #[test]
    fn can_remove_specified_homophones() {
        let homophone1 = ("be".to_string(), "bee".to_string());
        let homophone2 = ("right".to_string(), "write".to_string());
        let homophone3 = ("tea".to_string(), "tee".to_string());
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            homophones_list: Some(vec![homophone1, homophone2, homophone3]),
            to_lowercase: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"tea".to_string()));
        assert!(!new_list.contains(&"tee".to_string()));
        assert!(new_list.contains(&"mistake".to_string()));
        assert!(new_list.contains(&"be".to_string()));
    }
}
