pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut count = 0;
    let length = digits.len();
    let mut carry_over = false;
    for i in digits.iter_mut().rev() {
        match i {
            9 => *i = 0,
            _ => *i += 1,
        }
        if *i == 0 {
            count += 1;
            if count == length {
                carry_over = true;
            }
            continue;
        } else {
            break;
        }
    }
    if carry_over {
        digits.insert(0, 1);
    }
    digits
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4])
    }

    #[test]
    fn ex2() {
        assert_eq!(plus_one(vec![9]), vec![1, 0])
    }
}
