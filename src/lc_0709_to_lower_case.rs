pub fn to_lower_case(s: String) -> String {
    s.to_lowercase()
}

#[cfg(test)]
mod test {
    use crate::to_lower_case;

    #[test]
    fn ex1() {
        assert_eq!(to_lower_case("HELLO".to_string()), "hello".to_string())
    }
}
