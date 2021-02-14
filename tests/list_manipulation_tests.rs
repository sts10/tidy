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
                "be",
                "I",
                "vAcation",
                "take",
                "",
                "mistake",
                "post-modern",
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
    fn can_remove_integers_from_words() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_remove_integers: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"active".to_string()));
    }

    #[test]
    fn can_remove_through_first_tab() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_remove_through_first_tab: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"active".to_string()));
        assert!(new_list.contains(&"acclaim".to_string()));
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
    fn can_remove_words_shorter_than_a_specified_minimum_length() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            minimum_length: Some(3),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"i".to_string()));
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
}
