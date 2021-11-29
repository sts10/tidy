mod list_information_tests {
    use tidy::*;

    #[test]
    fn can_calculate_entropy_per_word_of_generated_list() {
        assert_eq!(calc_entropy(7_776), 12.92481250360578);
        assert_eq!(calc_entropy(16_103), 13.97504186800953);
        assert_eq!(calc_entropy(18_318), 14.160974374927937);
    }

    #[test]
    fn can_calculate_assumed_entropy_per_letter_of_generated_list() {
        let list: Vec<String> = vec!["to", "canopy", "cold", "seasons", "fire", "Christmas"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(assumed_entropy_per_letter(&list), 1.292481250360578);
    }
}
