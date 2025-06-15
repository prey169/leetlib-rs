pub fn max_diff(num: i32) -> i32 {
    fn find_min(char_num: Vec<char>) -> i32 {
        let mut new_char: Vec<char> = vec![];
        let mut changed_char: Option<char> = None;
        let mut char_num = char_num.iter();
        let mut count = 0;
        let mut min_char = '1';
        while let Some(c) = char_num.next() {
            if changed_char.is_none() {
                match *c {
                    '2'..='9' => {
                        if count != 0 {
                            min_char = '0';
                        }
                        changed_char = Some(*c);
                        new_char.push(min_char)
                    }
                    _ => new_char.push(*c),
                }
            } else {
                if c == &changed_char.unwrap() {
                    new_char.push(min_char);
                } else {
                    new_char.push(*c)
                }
            }
            count += 1;
        }
        println!("{:?}", new_char);
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
        println!("{:?}", new_char);

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
        assert_eq!(max_diff(555), 888);
    }

    #[test]
    fn ex2() {
        assert_eq!(max_diff(9), 8);
    }

    #[test]
    fn ex3() {
        assert_eq!(max_diff(123456), 820000)
    }
}
