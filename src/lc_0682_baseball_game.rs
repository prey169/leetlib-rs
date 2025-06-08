pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut record: Vec<i32> = vec![];
    for i in operations.iter() {
        match i.as_str() {
            "+" => record.push(record[record.len() - 2] + record[record.len() - 1]),
            "C" => {
                if !record.is_empty() {
                    record.pop();
                }
            }
            "D" => record.push(record[record.len() - 1] * 2),
            _ => record.push(i.parse().unwrap()),
        }
    }
    record.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert_eq!(
            cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ]),
            30
        )
    }
}
