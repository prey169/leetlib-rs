pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
    let mut count = 0;
    let length = baskets.len();

    for fruit in fruits {
        let mut extras = 1;
        for i in 0..length {
            if fruit <= baskets[i] {
                baskets[i] = 0;
                extras = 0;
                break;
            }
        }
        count += extras;
    }
    count
}

#[cfg(test)]
mod test {
    use super::num_of_unplaced_fruits;

    #[test]
    fn ex1() {
        assert_eq!(num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]), 1);
    }

    #[test]
    fn ex2() {
        assert_eq!(num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]), 0);
    }
}
