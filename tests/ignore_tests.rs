mod ignore_tests {
    use tidy::*;

    fn make_list() -> Vec<String> {
        vec!["mA1,word1 mB1", "mA2,word2 mB2", "mA3,word3 mB3", "A,B,C"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn can_ignore_metadata_before_a_delimiter() {
        let this_tidy_request = TidyRequest {
            list: make_list(),
            ignore_before_delimiter: Some(','),
            maximum_length: Some(10),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        assert!(new_list.contains(&"mA1,word1 mB1".to_string()));
    }
    #[test]
    fn can_ignore_metadata_after_a_delimiter() {
        let this_tidy_request = TidyRequest {
            list: make_list(),
            ignore_after_delimiter: Some('s'),
            maximum_length: Some(10),
            ..Default::default()
        };
        let new_list = tidy_list(this_tidy_request);
        println!("{:?}", new_list);
        assert!(new_list.contains(&"mA1,word1 mB1".to_string()));
    }
}
