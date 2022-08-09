mod list_manipulation_tests {
    use tidy::*;

    fn make_lists() -> (Vec<String>, Vec<String>, Vec<String>) {
        (
            vec![
                "  zookeeper",
                "apple",
                "cHarLie",
                "app",
                "tea",
                "addiction",
                "zoo",
                "keeper",
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
                "comma,203478",
                "“smart”",
                "‘quotes’",
                "  h as spaces ",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
            vec![
                "Normal",
                "the,2048",
                "اج 12",
                "11225	tab",
                "11152 space",
                "11156	word	tabs",
                "word-with-hypens",
                "Uppercase",
                "hello109823",
                "   ",
                "",
                "13910 word with spaces in it",
                "comma,203478",
                "京",
                "can't",
                "\"dumb quotes\"",
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
            sort_alphabetically: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list[0] == "addiction".to_string());
        assert!(new_list.contains(&"station".to_string()));
        assert!(new_list[new_list.len() - 1] == "zookeeper".to_string());
    }

    #[test]
    fn respect_option_to_not_sort_alphabetically() {
        let this_tidy_request = TidyRequest {
            list: make_lists().0,
            sort_alphabetically: false,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list[0] == "zookeeper".to_string());
        assert!(new_list.contains(&"apple".to_string()));
        assert!(new_list[new_list.len() - 1] == "station".to_string());
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
    fn can_take_first_3_elements() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            take_first: Some(4),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        println!("List length now {}", new_list.len());
        assert_eq!(new_list.len(), 4);
        assert_ne!(new_list.len(), 3);
        assert_ne!(new_list.len(), 15);
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
    fn can_straighten_quotes() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_straighten_quotes: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"\"smart\"".to_string()));
        assert!(new_list.contains(&"'quotes'".to_string()));
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
    fn can_remove_nonalphanumeric_words_from_list() {
        let this_tidy_request = TidyRequest {
            list: make_lists().2,
            should_remove_nonalphanumeric: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);

        assert!(new_list.contains(&"Uppercase".to_string()));
        assert!(new_list.contains(&"京".to_string()));
        assert!(new_list.contains(&"hello109823".to_string()));
        assert!(!new_list.contains(&"word-with-hypens".to_string()));
        assert!(!new_list.contains(&"comma,203478".to_string()));
        assert!(!new_list.contains(&"اج 12".to_string()));
    }

    #[test]
    fn can_remove_nonalphabetic_words_from_list() {
        let this_tidy_request = TidyRequest {
            list: make_lists().2,
            should_remove_nonalphabetic: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);

        assert!(new_list.contains(&"Uppercase".to_string()));
        assert!(new_list.contains(&"京".to_string()));
        assert!(!new_list.contains(&"hello109823".to_string()));
        assert!(!new_list.contains(&"word-with-hypens".to_string()));
        assert!(!new_list.contains(&"comma,203478".to_string()));
        assert!(!new_list.contains(&"اج 12".to_string()));
    }
    #[test]
    fn can_remove_non_latin_alphabetic_words_from_list() {
        let this_tidy_request = TidyRequest {
            list: make_lists().2,
            should_remove_non_latin_alphabetic: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);

        assert!(new_list.contains(&"Uppercase".to_string()));
        assert!(!new_list.contains(&"京".to_string()));
        assert!(!new_list.contains(&"hello109823".to_string()));
        assert!(!new_list.contains(&"word-with-hypens".to_string()));
        assert!(!new_list.contains(&"comma,203478".to_string()));
        assert!(!new_list.contains(&"اج 12".to_string()));
    }

    #[test]
    fn can_remove_non_ascii_words_from_list() {
        let this_tidy_request = TidyRequest {
            list: make_lists().2,
            should_remove_nonascii: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);

        assert!(new_list.contains(&"Uppercase".to_string()));
        assert!(new_list.contains(&"hello109823".to_string()));
        assert!(new_list.contains(&"word-with-hypens".to_string()));
        assert!(new_list.contains(&"comma,203478".to_string()));
        assert!(!new_list.contains(&"京".to_string()));
        assert!(!new_list.contains(&"اج 12".to_string()));
    }

    #[test]
    fn can_delete_before_first_tab() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_before_first_delimiter: Some('\t'),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"active".to_string()));
        assert!(new_list.contains(&"acclaim".to_string()));
        // Only remove through FIRST tab
        assert!(new_list.contains(&"word\ttabs".to_string()));
    }

    #[test]
    fn can_delete_before_first_space() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_before_first_delimiter: Some(' '),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"clad".to_string()));
        // Check that it only removes characters through first space, rather than just
        // between first space and second space, for example
        assert!(new_list.contains(&"word with spaces in it".to_string()));
        // Tidy trims leading whitespace first, so the "h"
        // will be cut here.
        assert!(new_list.contains(&"as spaces".to_string()));
    }
    #[test]
    fn can_delete_before_first_comma() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_before_first_delimiter: Some(','),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"203478".to_string()));
        assert!(new_list.contains(&"h as spaces".to_string()));
    }

    #[test]
    fn can_delete_after_first_tab() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_after_first_delimiter: Some('\t'),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"11225".to_string()));
        assert!(new_list.contains(&"11152".to_string()));
        // remove after FIRST tab
        assert!(new_list.contains(&"11156".to_string()));
    }
    #[test]
    fn can_delete_after_first_space() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_after_first_delimiter: Some(' '),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"19-6-8".to_string()));
        assert!(new_list.contains(&"13910".to_string()));
        assert!(new_list.contains(&"post-modern".to_string()));
        assert!(new_list.contains(&"comma,203478".to_string()));
    }
    #[test]
    fn can_delete_after_first_comma() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_after_first_delimiter: Some(','),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"comma".to_string()));
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
    fn can_remove_suffix_words() {
        let this_tidy_request = TidyRequest {
            list: make_lists().0,
            should_remove_suffix_words: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"keeper".to_string()));
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
    fn can_remove_words_with_nonalphabetic_characters() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_remove_nonalphabetic: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"19-6-8 clad".to_string()));
        assert!(!new_list.contains(&"world999".to_string()));
        assert!(!new_list.contains(&"world".to_string()));
        assert!(!new_list.contains(&"post-modern".to_string()));
        assert!(!new_list.contains(&"postmodern".to_string()));
        assert!(new_list.contains(&"take".to_string()));
        assert!(new_list.contains(&"wizard".to_string()));
        assert!(new_list.contains(&"vAcation".to_string()));
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
    fn can_remove_words_longer_than_a_specified_maximum_length() {
        let this_tidy_request = TidyRequest {
            list: make_lists().0,
            maximum_length: Some(7),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(!new_list.contains(&"addiction".to_string()));
        assert!(!new_list.contains(&"zookeeper".to_string()));
        assert!(!new_list.contains(&"stationary".to_string()));
        assert!(new_list.contains(&"tea".to_string()));
        assert!(new_list.contains(&"station".to_string()));
    }

    #[test]
    fn can_remove_words_longer_than_a_specified_maximum_length_after_deleting_integers() {
        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            should_delete_integers: true,
            maximum_length: Some(7),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"active".to_string()));
        assert!(new_list.contains(&"acclaim".to_string()));
        assert!(!new_list.contains(&"word with spacaes in it".to_string()));
    }

    #[test]
    fn can_gurantee_a_maximum_length_of_shared_prefix_for_autocomplete() {
        let this_tidy_request = TidyRequest {
            list: make_lists().0,
            maximum_shared_prefix_length: Some(3),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"zoo".to_string()));
        assert!(!new_list.contains(&"zookeeper".to_string()));
        assert!(new_list.contains(&"station".to_string()));
        assert!(!new_list.contains(&"stationary".to_string()));
        assert!(new_list.contains(&"app".to_string()));
        assert!(!new_list.contains(&"apple".to_string()));
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
        let approved_words: Vec<String> = vec!["take", "vAcation", "airplane"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let this_tidy_request = TidyRequest {
            list: make_lists().1,
            approved_list: Some(approved_words),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"take".to_string()));
        assert!(new_list.contains(&"vAcation".to_string()));
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
        assert!(new_list.contains(&"be".to_string()));
        assert!(!new_list.contains(&"bee".to_string()));
        assert!(new_list.contains(&"mistake".to_string()));
    }

    #[test]
    fn can_print_dice_rolls_of_base_6() {
        assert_eq!(print_as_dice(0, 6, 7776, false), "11111".to_string());
        assert_eq!(print_as_dice(7775, 6, 7776, false), "66666".to_string());
        assert_eq!(print_as_dice(2548, 6, 7776, false), "26555".to_string());
        assert_eq!(print_as_dice(2548, 6, 7000, false), "26555".to_string());
    }
    #[test]
    fn can_print_dice_rolls_of_base_2() {
        assert_eq!(
            print_as_dice(1, 2, 7776, false),
            "0000000000001".to_string()
        );
        assert_eq!(print_as_dice(127, 2, 128, false), "1111111".to_string());
    }
    #[test]
    fn can_print_dice_rolls_of_base_20() {
        assert_eq!(print_as_dice(1000, 20, 8000, false), "03-11-01".to_string());
        assert_eq!(print_as_dice(1000, 20, 8000, true), "2-a-0".to_string());
    }
}
