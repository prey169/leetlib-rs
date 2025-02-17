pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut position = 0;
    let mut count: usize;
    let needed = needle.len();
    let max = haystack.len() - 1;
    let needle: Vec<char> = needle.chars().collect();
    let haystack: Vec<char> = haystack.chars().collect();
    if needed > haystack.len() {
        return -1;
    }
    'outer: for _i in haystack.clone() {
        count = 0;
        for j in needle.clone() {
            if (count + position) > max {
                break 'outer;
            }
            if haystack[count + position] == j {
                count += 1;
            } else {
                position += 1;
                continue 'outer;
            }
            if count == needed {
                return position as i32;
            }
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::str_str;

    #[test]
    fn ex1() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }

    #[test]
    fn ex2() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }
}
