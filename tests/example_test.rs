#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]

mod tests {

    use defmt::assert;
    use defmt::info;
    use defmt_rtt as _;
    use mimxrt685s_pac as pac;

    #[test]
    async fn test_example() {
        let p = pac::Peripherals::take().unwrap();

        info!("Example test");
        assert!(true);
    }

    #[test]
    #[should_panic]
    async fn test_example_panic() {
        let p = pac::Peripherals::take().unwrap();

        info!("Should panic");
        assert!(false);
    }
}
