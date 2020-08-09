mod list_manipulation_tests {
    use tidy::*;

    fn make_lists() -> (Vec<String>, Vec<String>) {
        (
            vec![
                "  zookeeper",
                "apple",
                "CharLie",
                "app",
                "tea",
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
        let new_list = tidy_list(make_lists().0, false, false, false, false, None);
        assert!(new_list.contains(&"tea".to_string()));
        assert!(new_list.contains(&"apple".to_string()));
        assert!(new_list.len() == make_lists().0.len() - 2);
    }

    #[test]
    fn can_sort_words_alphabetically() {
        let new_list = tidy_list(make_lists().1, true, false, true, true, None);
        assert!(new_list[0] == "acclaim".to_string());
        assert!(new_list.contains(&"ardor".to_string()));
        assert!(new_list[new_list.len() - 1] == "wizard".to_string());
    }

    #[test]
    fn removes_blank_lines() {
        let new_list = tidy_list(make_lists().1, false, false, true, false, None);
        assert!(new_list.len() == make_lists().1.len() - 2);
    }

    #[test]
    fn removes_starting_and_trailing_whitespace() {
        let new_list = tidy_list(make_lists().1, false, false, false, false, None);
        assert!(new_list.contains(&"wizard".to_string()));
    }

    #[test]
    fn does_not_remove_inner_spaces() {
        let new_list = tidy_list(make_lists().1, false, false, false, false, None);
        assert!(new_list.contains(&"h as spaces".to_string()));
    }

    #[test]
    fn can_remove_integers_from_words() {
        let new_list = tidy_list(make_lists().1, true, false, true, false, None);
        assert!(new_list.contains(&"active".to_string()));
    }

    #[test]
    fn can_remove_through_first_tab() {
        let new_list = tidy_list(make_lists().1, false, false, false, true, None);
        assert!(new_list.contains(&"active".to_string()));
        assert!(new_list.contains(&"acclaim".to_string()));
    }
    #[test]
    fn can_lowercase_words() {
        let new_list = tidy_list(make_lists().0, true, false, false, false, None);
        assert!(new_list.contains(&"charlie".to_string()));
        let new_list = tidy_list(make_lists().1, true, false, true, false, None);
        assert!(new_list.contains(&"vacation".to_string()));
        assert!(new_list.contains(&"ardor".to_string()));
    }

    #[test]
    fn can_remove_prefix_words() {
        let new_list = tidy_list(make_lists().0, true, true, false, false, None);
        assert!(!new_list.contains(&"station".to_string()));
        assert!(new_list.contains(&"stationary".to_string()));
        assert!(!new_list.contains(&"zoo".to_string()));
        assert!(new_list.contains(&"zookeeper".to_string()));
        assert!(new_list.contains(&"apple".to_string()));
    }

    #[test]
    fn can_remove_reject_words() {
        let words_to_reject: Vec<String> = vec!["mistake", "carnival"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let new_list = tidy_list(
            make_lists().1,
            true,
            true,
            false,
            false,
            Some(words_to_reject),
        );
        assert!(!new_list.contains(&"mistake".to_string()));
        assert!(!new_list.contains(&"carnival".to_string()));
        assert!(new_list.contains(&"wizard".to_string()));
    }
}
