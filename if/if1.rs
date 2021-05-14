// if1.rs

pub fn bigger(a: i32, b: i32) -> i32 {
    if (a > b) {
        a
    } else {
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(8, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
