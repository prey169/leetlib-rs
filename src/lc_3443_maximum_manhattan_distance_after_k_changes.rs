pub fn max_distance(s: String, k: i32) -> i32 {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut ans = 0;
    let mut steps_taken = 0;
    for c in s.chars() {
        steps_taken += 1;
        match c {
            'N' => y += 1,

            'W' => x -= 1,

            'E' => x += 1,
            _ => y -= 1,
        };

        let total_dist = x.abs() + y.abs();
        let remaining_k = steps_taken - total_dist;
        if remaining_k <= 2 * k {
            ans = ans.max(total_dist + remaining_k)
        } else {
            ans = ans.max(total_dist + 2 * k)
        }
    }
    ans
}

// pub fn max_distance(s: String, k: i32) -> i32 {
//     fn get_all_distances(mut c: &mut [char]) -> Vec<(i32, i32)> {
//         let mut result = vec![];
//         println!("{:?}", c);
//         if c.is_empty() {
//             return result;
//         } else {
//             match *c.iter().next().unwrap() {
//                 'N' => {
//                     result.push((0, 1));
//                     result.extend(get_all_distances(c.split_off_mut(1..).unwrap_or_default()));
//                 }
//                 'W' => {
//                     result.push((-1, 0));
//                     result.extend(get_all_distances(c.split_off_mut(1..).unwrap_or_default()))
//                 }
//                 'E' => {
//                     result.push((1, 0));
//                     result.extend(get_all_distances(c.split_off_mut(1..).unwrap_or_default()))
//                 }
//                 _ => {
//                     result.push((0, -1));
//                     result.extend(get_all_distances(c.split_off_mut(1..).unwrap_or_default()))
//                 }
//             }
//         }
//         result
//     }
//     let mut dict = std::collections::HashMap::new();
//     dict.insert('W', 0);
//     dict.insert('E', 0);
//     dict.insert('S', 0);
//     dict.insert('N', 0);
//     let mut c: Vec<char> = s.chars().collect();
//     let mut n_first = false;
//     let mut s_first = false;
//     let mut e_first = false;
//     let mut w_first = false;
//     for c in s.chars() {
//         if !(n_first || s_first) {
//             if c == 'N' {
//                 n_first = true;
//             } else if c == 'S' {
//                 s_first = true;
//             }
//         }
//         if !(w_first || e_first) {
//             if c == 'W' {
//                 w_first = true;
//             } else if c == 'E' {
//                 e_first = true;
//             }
//         }
//         dict.entry(c).and_modify(|count| *count += 1).or_insert(1);
//     }
//     // let z = get_all_distances(&mut c);
//     // .iter()
//     // .map(|(x, y)| (x + y).abs())
//     // .collect();
//     println!("{:?}", dict);
//     let mut sum: (i32, i32) = (0, 0);
//     let mut total;
//     let mut max_total = 0;
//     let mut count = 0;

//     for ch in c {
//         let i = match ch {
//             'N' => (0, 1),

//             'W' => (-1, 0),

//             'E' => (1, 0),
//             _ => (0, -1),
//         };
//         println!("{count}, {k}");
//         if sum.1 >= 0
//             && (dict[&'N'] > dict[&'S'] || (dict[&'N'] == dict[&'S'] && s_first))
//             && ch == 'S'
//             && count != k
//         {
//             sum.1 -= i.1;
//             count += 1;
//         } else if sum.1 <= 0
//             && (dict[&'S'] > dict[&'N'] || (dict[&'N'] == dict[&'S'] && n_first))
//             && ch == 'N'
//             && count != k
//         {
//             sum.1 -= i.1;
//             count += 1;
//         } else {
//             sum.1 += i.1;
//         }
//         if sum.0 <= 0
//             && (dict[&'W'] > dict[&'E'] || (dict[&'W'] == dict[&'E'] && w_first))
//             && ch == 'E'
//             && count != k
//         {
//             sum.0 -= i.0;
//             count += 1;
//         } else if sum.0 >= 0
//             && (dict[&'E'] > dict[&'W'] || (dict[&'W'] == dict[&'E'] && e_first))
//             && ch == 'W'
//             && count != k
//         {
//             sum.0 -= i.0;
//             count += 1;
//         } else {
//             sum.0 += i.0;
//         }
//         total = sum.0.abs() + sum.1.abs();
//         if max_total < total {
//             max_total = total;
//         }
//         println!("{i:?} sum {:?}, total {total}", sum);
//     }
//     // for i in z {
//     //     if ((i.0 < 0 && sum.0 > 0) || i.0 > 0 && sum.0 < 0) && count != k {
//     //         sum.0 -= i.0;
//     //         count += 1;
//     //     } else {
//     //         sum.0 += i.0;
//     //     }

//     //     if ((i.1 < 0 && sum.1 > 0) || i.1 > 0 && sum.1 < 0) && count != k {
//     //         count += 1;
//     //         sum.1 -= i.1
//     //     } else {
//     //         sum.1 += i.1;
//     //     }
//     //     total = (sum.0).abs() + (sum.1).abs();
//     //     if max_total < total {
//     //         max_total = total;
//     //     }
//     //     println!("{:?}, {total}", sum);
//     // }
//     max_total
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_distance("NWSE".to_string(), 1), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(max_distance("NSWWEW".to_string(), 3), 6);
    }

    #[test]
    fn ex3() {
        assert_eq!(max_distance("NEES".to_string(), 1), 4);
    }

    #[test]
    fn ex4() {
        assert_eq!(max_distance("NSES".to_string(), 1), 4);
    }

    #[test]
    fn ex5() {
        assert_eq!(max_distance("WEWE".to_string(), 1), 3);
    }

    #[test]
    fn ex6() {
        assert_eq!(max_distance("EWEW".to_string(), 1), 3);
    }

    #[test]
    fn ex7() {
        assert_eq!(max_distance("EWWE".to_string(), 1), 3);
    }

    #[test]
    fn ex8() {
        assert_eq!(max_distance("EWEW".to_string(), 1), 3);
    }
}
// let mut k = k;
// let mut sum = 0;
// let mut rolling_sum = 0;
// for c in s.chars() {
//     println!("{c}");
//     match c {
//         'N' | 'E' => {
//             if rolling_sum == 0 {
//                 sum += 1;
//             } else {
//                 rolling_sum -= 1;
//             }
//         }
//         _ => {
//             if k == 0 {
//                 rolling_sum -= 1;
//             } else {
//                 k -= 1;
//                 if rolling_sum == 0 {
//                     sum += 1;
//                 } else {
//                     rolling_sum -= 1
//                 }
//             }
//         }
//     }
//     println!("{sum}, {rolling_sum}");
// }
// sum
