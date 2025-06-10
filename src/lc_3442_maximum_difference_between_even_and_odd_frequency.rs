pub fn max_difference(s: String) -> i32 {
    // 100% runtime + 100% memory
    let mut dict = std::collections::HashMap::new();
    let mut odds = 0;
    let mut evens = i32::MAX;
    for c in s.chars() {
        dict.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }
    for i in dict.values().cloned() {
        if i % 2 == 0 && i < evens {
            evens = i;
        } else if i % 2 != 0 && i > odds {
            odds = i;
        }
    }
    if evens == i32::MAX {
        evens = 0;
    }
    odds - evens
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert_eq!(max_difference("aaaaabbc".to_string()), 3)
    }

    #[test]
    fn ex2() {
        assert_eq!(max_difference("abcabcab".to_string()), 1)
    }

    #[test]
    fn ex3() {
        assert_eq!(max_difference("mmsmsym".to_string()), -1)
    }

    #[test]
    fn ex4() {
        assert_eq!(max_difference("tzt".to_string()), -1)
    }
}
