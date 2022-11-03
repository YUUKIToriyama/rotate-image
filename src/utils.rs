pub fn average(a: u8, b: u8) -> u8 {
    if a > b {
        a - (a - b) / 2
    } else {
        b - (b - a) / 2
    }
}

#[cfg(test)]
mod utils_test {
    use super::*;

    #[test]
    fn get_average() {
        assert_eq!(average(0, 10), 5);
        assert_eq!(average(10, 0), 5);
        assert_eq!(average(255, 100), 178);
        assert_eq!(average(100, 255), 178);
    }
}
