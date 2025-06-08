pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    let mut arr = arr.clone();
    arr.sort();
    let length = arr.len();
    let mut count = 1;

    while count < length - 1 {
        if count + 1 < length {
            if (arr[count] - arr[count + 1]) == arr[0] - arr[1] {
                count += 1;
                continue;
            } else {
                return false;
            };
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert!(can_make_arithmetic_progression(vec![3, 5, 1]))
    }
}
