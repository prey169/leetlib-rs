pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let outer_max_length = matrix.len() as i32;
    let inner_len = matrix[0].len() as i32;

    let mut min = 0;
    let mut max = outer_max_length * inner_len - 1;
    let mut mid: i32;
    let mut mid_value: i32;

    while min <= max {
        mid = min + (max - min) / 2;
        mid_value = matrix[(mid / inner_len) as usize][(mid % inner_len) as usize];
        if mid_value == target {
            return true;
        } else if mid_value < target {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::search_matrix;

    #[test]
    fn ex1() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(search_matrix(vec![vec![1, 3]], 1), true);
    }

    #[test]
    fn ex4() {
        assert_eq!(search_matrix(vec![vec![1, 3]], 3), true);
    }

    #[test]
    fn ex5() {
        assert_eq!(search_matrix(vec![vec![1], vec![3], vec![5]], 5), true);
    }

    #[test]
    fn ex6() {
        assert_eq!(search_matrix(vec![vec![1]], 0), false);
    }
}
