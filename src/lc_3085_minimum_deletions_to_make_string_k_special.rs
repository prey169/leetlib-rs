use std::collections::HashMap;
pub fn minimum_deletions(word: String, k: i32) -> i32 {
    let mut dict: HashMap<char, i32> = std::collections::HashMap::new();
    for c in word.chars() {
        *dict.entry(c).or_default() += 1;
    }
    let min = dict.values().min().unwrap();
    let max = dict.values().max().unwrap();
    if max - min <= k {
        0
    } else {
        dict.values()
            .map(|&value| {
                dict.values()
                    .filter(|&&other| other != value)
                    .fold(0, |acc, &other| {
                        acc + if value > other {
                            other
                        } else if k + value < other {
                            other - (value + k)
                        } else {
                            0
                        }
                    })
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(minimum_deletions("aabcaba".to_string(), 0), 3)
    }
}
