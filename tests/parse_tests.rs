mod parse_tests {
    use tidy::parsers::eval_list_length;

    #[test]
    fn can_parse_print_rand() {
        assert_eq!(eval_list_length("7776").unwrap(), 7776);
        assert_eq!(eval_list_length("6**5").unwrap(), 7776);
        assert_eq!(eval_list_length("10000").unwrap(), 10000);
        assert_eq!(eval_list_length("10**2").unwrap(), 100);
    }

    #[test]
    fn panics_when_noninteger_is_inputted_to_print_rand() {
        assert!(eval_list_length("four").is_err());
    }

    #[test]
    fn panics_when_too_many_exponents_inputted_to_print_rand() {
        assert!(eval_list_length("2**4**3").is_err());
    }
}
