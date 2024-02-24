mod pruning_tests {
    use tidy::display_information::uniquely_decodable::is_uniquely_decodable;
    use tidy::*;

    #[test]
    fn can_resist_pruning_a_list_that_is_already_uniquely_decodable() {
        let list: Vec<String> = vec!["101", "00", "0001", "1"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let this_tidy_request = TidyRequest {
            list: list.clone(),
            should_schlinkert_prune: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert_eq!(list, new_list);
    }

    #[test]
    fn can_run_schlinkert_prune_on_reversed_list_if_it_saves_more_words() {
        let list: Vec<String> = vec![
            "news",
            "paper",
            "newspaper",
            "donkey",
            "newsdonkey",
            "ghost",
            "newsghost",
            "radish",
            "newsradish",
        ]
        .iter()
        .map(|w| w.to_string())
        .collect();

        let this_tidy_request = TidyRequest {
            list: list,
            should_schlinkert_prune: true,
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        // If Schlinkert prune was done in forwards, only
        // 5 words would be saved. But if we Schlinkert
        // prune the reversed list, we save 8 words.
        assert!(new_list.len() == 8);
        // And now let's confirm that the new list is indeed
        // uniquely decodable, at least as far as Tidy is able
        // to confirm.
        assert!(is_uniquely_decodable(&new_list));
    }
}
