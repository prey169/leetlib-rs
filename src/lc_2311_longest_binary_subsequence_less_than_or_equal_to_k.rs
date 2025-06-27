pub fn longest_subsequence(s: String, k: i32) -> i32 {
    let mut count = 0;
    let mut value = 0;
    for ch in s.bytes().rev() {
        if ch == b'0' {
            count += 1
        } else if count < 30 {
            let bit = 1 << count;
            if value + bit <= k {
                value += bit;
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::longest_subsequence;

    #[test]
    fn ex1() {
        assert_eq!(longest_subsequence("1001010".to_string(), 5), 5);
    }

    #[test]
    fn ex2() {
        assert_eq!(longest_subsequence("000101010011011001011101111000111111100001011000000100010000111100000011111001000111100111101001111001011101001011011101001011011001111111010011100011110111010000010000010111001001111101100001111".to_string(), 300429827), 108);
    }
}
// use std::collections::VecDeque;
// let bits: Vec<char> = s.chars().rev().collect();
// let mut count = 0;
// let mut k_crossed = false;
// let mut ans_bits: VecDeque<i32> = VecDeque::new();
// for bit in bits.iter() {
//     if *bit == '1' {
//         if !k_crossed {
//             ans_bits.push_front(1);
//         }
//         if !k_crossed
//             && ans_bits
//                 .clone()
//                 .into_iter()
//                 .fold(0, |acc, digit| (acc << 1) + digit)
//                 <= k
//         {
//             count += 1;
//             continue;
//         } else if !k_crossed {
//             k_crossed = true;
//             ans_bits.remove(0);
//         }
//     } else {
//         ans_bits.push_front(0);
//         count += 1;
//     }
// }
// count
