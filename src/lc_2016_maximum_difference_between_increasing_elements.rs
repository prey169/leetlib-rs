pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut sum = -1;
    let mut max = i32::MIN;
    for i in nums {
        if i > max && max != i32::MAX && i != min {
            max = i
        };
        if i < min {
            if (max != i32::MIN || min != i32::MAX) && sum < (max - min) {
                sum = max - min
            }
            min = i;
            max = i32::MIN + 1;
        };
        if max > min && sum < (max - min) {
            sum = max - min;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(maximum_difference(vec![7, 1, 5, 4]), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(maximum_difference(vec![9, 4, 3, 2]), -1);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            maximum_difference(vec![87, 68, 91, 86, 58, 63, 43, 98, 6, 40]),
            55
        );
    }

    #[test]
    fn ex4() {
        assert_eq!(
            maximum_difference(vec![
                999, 997, 980, 976, 948, 940, 938, 928, 924, 917, 907, 907, 881, 878, 864, 862,
                859, 857, 848, 840, 824, 824, 824, 805, 802, 798, 788, 777, 775, 766, 755, 748,
                735, 732, 727, 705, 700, 697, 693, 679, 676, 644, 634, 624, 599, 596, 588, 583,
                562, 558, 553, 539, 537, 536, 509, 491, 485, 483, 454, 449, 438, 425, 403, 368,
                345, 327, 287, 285, 270, 263, 255, 248, 235, 234, 224, 221, 201, 189, 187, 183,
                179, 168, 155, 153, 150, 144, 107, 102, 102, 87, 80, 57, 55, 49, 48, 45, 26, 26,
                23, 15
            ]),
            -1
        );
    }
}
