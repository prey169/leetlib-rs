use std::collections::HashMap;

pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    fn pick_box(mut n: i32) -> i32 {
        let mut sum = 0;
        while n != 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }
    let mut boxes: HashMap<i32, u32> = HashMap::new();
    let numbers: Vec<i32> = (low_limit..=high_limit).collect();
    numbers.iter().for_each(|x| {
        let boxx = pick_box(*x);
        *boxes.entry(boxx).or_insert(0) += 1;
    });
    *boxes.values().max().unwrap() as i32
}

#[cfg(test)]
mod test {
    use crate::count_balls;

    #[test]
    fn ex1() {
        assert_eq!(count_balls(1, 10), 2)
    }

    #[test]
    fn ex2() {
        assert_eq!(count_balls(5, 15), 2)
    }

    #[test]
    fn ex3() {
        assert_eq!(count_balls(19, 28), 2)
    }
}
