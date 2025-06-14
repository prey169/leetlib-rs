pub fn min_max_difference(num: i32) -> i32 {
    fn find_min(char_num: Vec<char>) -> i32 {
        let mut new_char: Vec<char> = vec![];
        let mut changed_char: Option<char> = None;
        let mut char_num = char_num.iter();
        while let Some(c) = char_num.next() {
            if changed_char.is_none() {
                match *c {
                    '1'..='9' => {
                        changed_char = Some(*c);
                        new_char.push('0')
                    }
                    _ => new_char.push(*c),
                }
            } else {
                if c == &changed_char.unwrap() {
                    new_char.push('0');
                } else {
                    new_char.push(*c)
                }
            }
        }
        let new_char = new_char.iter().collect::<String>();
        let num: i32 = new_char.parse().unwrap();
        num
    }

    fn find_max(char_num: Vec<char>) -> i32 {
        let mut new_char: Vec<char> = vec![];
        let mut changed_char: Option<char> = None;
        let mut char_num = char_num.iter();
        while let Some(c) = char_num.next() {
            if changed_char.is_none() {
                match *c {
                    '0'..='8' => {
                        changed_char = Some(*c);
                        new_char.push('9')
                    }
                    _ => new_char.push(*c),
                }
            } else {
                if c == &changed_char.unwrap() {
                    new_char.push('9');
                } else {
                    new_char.push(*c)
                }
            }
        }

        let new_char = new_char.iter().collect::<String>();
        let num: i32 = new_char.parse().unwrap();
        num
    }

    let char_num = num.to_string();
    let char_num: Vec<char> = char_num.chars().collect();
    find_max(char_num.clone()) - find_min(char_num)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(min_max_difference(11891), 99009);
    }

    #[test]
    fn ex2() {
        assert_eq!(min_max_difference(90), 99);
    }

    // #[test]
    // fn ex3() {
    //     assert_eq!(max_adjacent_distance(vec![-4, -2, -3]), 2)
    // }
}
