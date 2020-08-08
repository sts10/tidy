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
                "be",
                "vAcation",
                "take",
                "",
                "mistake",
                "post-modern",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
        )
    }

    #[test]
    fn can_remove_duplicate_words() {
        let new_list = tidy_list(make_lists().0, false, false);
        assert!(new_list.contains(&"tea".to_string()));
        assert!(new_list.contains(&"apple".to_string()));
        assert!(new_list.len() == make_lists().0.len() - 2);
    }

    #[test]
    fn can_sort_words_alphabetically() {
        let new_list = tidy_list(make_lists().1, true, false);
        assert!(new_list[0] == "ardor".to_string());
        assert!(new_list.contains(&"ardor".to_string()));
        assert!(new_list[new_list.len() - 1] == "wizard".to_string());
    }

    #[test]
    fn removes_blank_lines() {
        let new_list = tidy_list(make_lists().1, false, false);
        assert!(new_list.len() == make_lists().1.len() - 2);
    }

    #[test]
    fn can_lowercase_words() {
        let new_list = tidy_list(make_lists().0, true, false);
        assert!(new_list.contains(&"charlie".to_string()));
        let new_list = tidy_list(make_lists().1, true, false);
        assert!(new_list.contains(&"vacation".to_string()));
        assert!(new_list.contains(&"ardor".to_string()));
    }

    #[test]
    fn can_remove_prefix_words() {
        let new_list = tidy_list(make_lists().0, true, true);
        assert!(!new_list.contains(&"station".to_string()));
        assert!(new_list.contains(&"stationary".to_string()));
        assert!(!new_list.contains(&"zoo".to_string()));
        assert!(new_list.contains(&"zookeeper".to_string()));
        assert!(new_list.contains(&"apple".to_string()));
    }
}
