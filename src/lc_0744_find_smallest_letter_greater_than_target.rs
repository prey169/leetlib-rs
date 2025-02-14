pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut left: i32 = 0;
    let mut right: i32 = (letters.len() - 1) as i32;
    let max: i32 = right;
    let mut mid: i32;
    let mut mid_value: char;

    while left <= right {
        mid = left + (right - left) / 2;
        mid_value = letters[mid as usize];

        if mid_value == target {
            if mid == max {
                return letters[0];
            } else {
                while mid <= max {
                    if letters[mid as usize] == target {
                        mid += 1;
                    } else {
                        return letters[mid as usize];
                    }
                }
                return letters[0];
            };
        } else if mid_value < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    if left > max {
        return letters[0];
    };
    letters[left as usize]
}

#[cfg(test)]
mod test {
    use super::next_greatest_letter;

    #[test]
    fn ex1() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    }

    #[test]
    fn ex2() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    }

    #[test]
    fn ex3() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'z'), 'c');
    }

    #[test]
    fn ex4() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'd'), 'f');
    }

    #[test]
    fn ex5() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'g'), 'j');
    }

    #[test]
    fn ex6() {
        assert_eq!(next_greatest_letter(vec!['e', 'e', 'g', 'g'], 'g'), 'e');
    }
}
