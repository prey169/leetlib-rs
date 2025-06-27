pub fn judge_circle(moves: String) -> bool {
    moves.chars().fold((0, 0), |(acc1, acc2), c| match c {
        'U' => (acc1, acc2 + 1),
        'D' => (acc1, acc2 + -1),
        'L' => (acc1 + -1, acc2),
        _ => (acc1 + 1, acc2),
    }) == (0, 0)
}

#[cfg(test)]
mod test {
    use super::judge_circle;

    #[test]
    fn ex1() {
        assert!(judge_circle("UD".to_string()));
    }

    #[test]
    fn ex2() {
        assert!(!judge_circle("LL".to_string()));
    }
}
