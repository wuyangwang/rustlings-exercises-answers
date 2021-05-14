// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    if num == 1 {
        1
    } else {
        // 1*1*2*3*4
        (1..=num).fold(1, |a, n| a * n)
        // let mut a = 1;
        // for n in 1..=num {
        //     a = a * n
        // }
        // a
    }
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
