pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    };
    let mut all_one_steps = 2;
    let mut all_two_steps = 1;
    let mut total_steps = 0;

    for _i in 2..n {
        total_steps = all_two_steps + all_one_steps;
        all_two_steps = all_one_steps;
        all_one_steps = total_steps;
    }
    total_steps
}
