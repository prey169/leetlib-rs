impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.to_string().split_whitespace().last().unwrap().len() as i32
    }
}
