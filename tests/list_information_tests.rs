mod list_information_tests {
    use tidy::display_information::*;
    // use tidy::*;

    #[test]
    fn can_calculate_entropy_per_word_of_generated_list() {
        assert_eq!(calc_entropy_per_word(7_776), 12.92481250360578);
        assert_eq!(calc_entropy_per_word(16_103), 13.97504186800953);
        assert_eq!(calc_entropy_per_word(18_318), 14.160974374927937);
    }

    #[test]
    fn can_calculate_assumed_entropy_per_character_of_generated_list() {
        let list: Vec<String> = vec!["to", "canopy", "cold", "seasons", "fire", "Christmas"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(assumed_entropy_per_character(&list), 1.292481250360578);
    }

    #[test]
    fn can_calculate_mean_edit_distance() {
        let list: Vec<String> = vec![
            "bat", "cat", "rat", "hat", "mat", "tat", "fat", "oat", "pat", "sat", "vat",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        assert_eq!(find_mean_edit_distance(&list), 1.0);

        let list2: Vec<String> = vec!["abcd", "abce", "abxz"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(find_mean_edit_distance(&list2), 1.6666666666666667);

        let list3: Vec<String> = vec!["abcd", "abce", "abxz", "abpt"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(find_mean_edit_distance(&list3), (11.0 / 6.0) as f64);
    }

    #[test]
    fn can_find_first_different_character() {
        assert_eq!(
            find_first_different_character_zero_indexed("apple", "zebra"),
            0
        );
        assert_eq!(
            find_first_different_character_zero_indexed("berry", "bicker"),
            1
        );
        assert_eq!(
            find_first_different_character_zero_indexed("hello", "help"),
            3
        );
        assert_eq!(
            find_first_different_character_zero_indexed("radius", "radical"),
            4
        );
        assert_eq!(
            find_first_different_character_zero_indexed("zip", "zippy"),
            3
        );
        assert_eq!(
            find_first_different_character_zero_indexed("zippy", "zip"),
            3
        );
    }

    #[test]
    fn can_find_longest_shared_prefix_in_a_list() {
        let list: Vec<String> = vec![
            "to",
            "canopy",
            "cold",
            "academia",
            "academic",
            "seasons",
            "fire",
            "Christmas",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        assert_eq!(find_longest_shared_prefix(&list), 7);
    }
    #[test]
    fn can_get_shortest_word_length() {
        let list: Vec<String> = vec!["canopy", "to", "cold", "seasons", "fire", "Christmas"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(get_shortest_word_length(&list), 2);
    }
    #[test]
    fn can_get_mean_word_length() {
        let list: Vec<String> = vec!["canopy", "to", "cold", "seasons", "fire", "Christmas"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(mean_word_length(&list), 5.3333335);
    }
}
