pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    //divideString -> int -> string -> char -> [String]
    //divide
    let chars: Vec<char> = s.chars().collect();
    let k = k as usize;

    chars
        .chunks(k)
        .map(|chunk| {
            let mut result: Vec<char> = chunk.to_vec();
            while result.len() < k {
                result.push(fill);
            }
            result.into_iter().collect::<String>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::divide_string;

    #[test]
    fn ex1() {
        assert_eq!(
            divide_string("abcdefghi".to_string(), 3, 'x'),
            ["abc", "def", "ghi"]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            divide_string("abcdefghij".to_string(), 3, 'x'),
            ["abc", "def", "ghi", "jxx"]
        );
    }
}
