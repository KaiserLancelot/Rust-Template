pub fn add(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_test() {
        assert_eq!(add(2, 2), 4);
    }
}
