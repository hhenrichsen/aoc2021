/// parses commands into deltas
pub fn prep_day2(command: &str) -> (i32, i32) {
    // Pull direction and count from the command
    if let [dir, raw_count] = command.split_whitespace().into_iter().collect::<Vec<&str>>()[..] {
        let count: i32 = raw_count.parse::<i32>().expect(raw_count);
        // Parse into dx/dy pair
        return if dir == "forward" {
            (count, 0)
        } else if dir == "up" {
            (0, -count)
        } else if dir == "down" {
            (0, count) // Down is "deeper" so positive
        } else {
            (0, 0)
        }
    // If the split doesn't do the thing I want it to, die
    } else {panic!("Invalid command format")}
}

pub fn day2_a(values: &[(i32, i32)]) -> i32 {
    // Sum the tuples
    let (total_x, total_y) = values.into_iter()
        .fold((0, 0), |(current_x, current_depth), (delta_x, delta_depth)| {
            (current_x + delta_x, current_depth + delta_depth)
        });
    // Return product of summed tuples
    return total_x * total_y;
}

pub fn day2_b(values: &[(i32, i32)]) -> i32 {
    // Sum the tuples, keeping track of a third aim param that's used to determine depth
    let (total_x, total_y, _) = values.into_iter()
        .fold((0, 0, 0), |(current_x, current_depth, current_aim), (delta_x, delta_aim)| {
            (current_x + delta_x, current_depth + (current_aim * delta_x), current_aim + delta_aim)
        });
    // Return product of summed tuples
    return total_x * total_y;
}
