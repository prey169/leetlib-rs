pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut ransom_note = ransom_note;
    for char in magazine.chars() {
        for (count, char2) in ransom_note.chars().enumerate() {
            if char == char2 {
                ransom_note.remove(count);
                break;
            }
        }
    }
    println!("{}", ransom_note);
    ransom_note.is_empty()
}

#[cfg(test)]
mod test {
    use crate::can_construct;

    #[test]
    fn ex1() {
        assert!(!can_construct("a".to_string(), "b".to_string()))
    }

    #[test]
    fn ex2() {
        assert!(!can_construct("aa".to_string(), "ab".to_string()))
    }

    #[test]
    fn ex3() {
        assert!(can_construct("aa".to_string(), "aab".to_string()))
    }

    #[test]
    fn ex4() {
        assert!(can_construct("aab".to_string(), "baa".to_string()))
    }
}
