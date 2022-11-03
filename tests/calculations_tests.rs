mod calculation_tests {
    use dicetailor::*;

    #[test]
    fn can_calculate_a_logarithm_accurately() {
        assert_eq!(log_base(6, 7776.0), 5.0);
        assert_eq!(log_base(2, 16.0), 4.0);
    }
    #[test]
    fn can_calculate_a_word_list_loss_amount() {
        assert_eq!(get_loss(6, 8000), 8000 - 7776);
        assert_eq!(get_loss(6, 7776), 0);
    }
}
