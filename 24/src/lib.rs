pub mod matrixwalkerer;

pub fn count_digits(num: usize) -> u32 {
    num.ilog10() + 1
}
pub fn split_number(num: usize) -> (usize, usize) {
    let digits = count_digits(num);
    assert!(digits % 2 == 0);
    let mul = 10_usize.pow(digits / 2);
    let upper = num / 10_usize.pow(digits / 2);
    let lower = num % (upper * mul);
    (upper, lower)
}
#[cfg(test)]
mod tests {
    use crate::split_number;

    use crate::count_digits;

    #[test]
    fn count_digits_works() {
        for num in 1..10000 {
            assert_eq!(count_digits(num), num.to_string().len() as u32);
        }
    }
    #[test]
    fn split_number_works() {
        let stone = 1234;
        assert_eq!(split_number(stone), (12, 34))
    }
}
