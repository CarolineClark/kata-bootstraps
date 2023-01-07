#[cfg(test)]
mod day11tests {
    use crate::day11::monkey_business;

    #[test]
    fn monkey_business_test() {
        let result = monkey_business();
        assert_eq!(result, 0);
    }
}