pub fn make_fancy_string(s: String) -> String {
    let mut fancy_count = 0;
    let mut previous_fancy: char = 'a';
    let chars: Vec<char> = s.chars().into_iter().collect();
    let mut fancy = vec![];
    for c in chars {
        if previous_fancy == c && fancy_count > 1 {
            continue;
        } else if previous_fancy != c {
            fancy_count = 0;
            previous_fancy = c;
        }
        fancy.push(c);
        fancy_count += 1;
    }
    fancy.iter().collect()
}
#[cfg(test)]
mod test {
    use crate::make_fancy_string;

    #[test]
    fn ex1() {
        assert_eq!(
            make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        )
    }
}
