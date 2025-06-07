pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut new_word = "".to_string();
    let mut mut_word1 = word1.chars();
    let mut mut_word2 = word2.chars();

    loop {
        let char1 = mut_word1.next();
        let char2 = mut_word2.next();

        if let Some(c) = char1 {
            new_word.push(c);
        }
        if let Some(c) = char2 {
            new_word.push(c);
        }
        if char1.is_none() && char2.is_none() {
            break;
        }
    }
    new_word
}

#[cfg(test)]
mod test {
    use crate::merge_alternately;

    #[test]
    fn ex1() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        )
    }
}
