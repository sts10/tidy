mod list_information_tests {
    use tidy::*;

    #[test]
    fn can_calculate_entropy_of_finished_list() {
        assert_eq!(calc_entropy(7_776), 12.92481250360578);
        assert_eq!(calc_entropy(16_103), 13.97504186800953);
        assert_eq!(calc_entropy(18_318), 14.160974374927937);
    }
}
