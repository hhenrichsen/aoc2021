pub fn day3_a(input: &[&str]) -> i32 {
    let counts = count(input);
    // Assemble most common binary string
    let mut digit_str = String::from("");
    let mut inverse_str = String::from("");
    for count in counts {
        if count >= input.len() / 2 {
            digit_str.push('1');
            inverse_str.push('0');
        }
        else {
            digit_str.push('0');
            inverse_str.push('1');
        }
    }
    // Parse string
    let digits = i32::from_str_radix(digit_str.as_str(), 2).expect(&*format!("Failed to parse {}", digit_str));
    let inverse = i32::from_str_radix(inverse_str.as_str(), 2).expect(&*format!("Failed to parse {}", inverse_str));
    return digits * inverse;
}

pub fn day3_b(input: &[&str]) -> i32 {
    let spaces = input[0].len();
    let mut curr = input.into_iter().cloned().collect::<Vec<&str>>();
    let mut layer = 0;
    let mut len = curr.len();
    while len > 1 && layer < spaces {
        let counts = count_in_space(curr.as_slice(), layer);
        let most_common = get_most_char(counts);
        let mut next: Vec<&str> = Vec::new();
        for line in curr {
            if line.as_bytes()[layer] as char == most_common {
                next.push(line);
            }
        }
        len = next.len();
        curr = next;
        layer += 1;
    }
    if curr.len() != 1 {
        panic!("Did not narrow to one result")
    }
    let result1 = curr[0];

    let mut curr = input.into_iter().cloned().collect::<Vec<&str>>();
    let mut layer = 0;
    let mut len = curr.len();
    while len > 1 && layer < spaces {
        let counts = count_in_space(curr.as_slice(), layer);
        let least_common = get_least_char(counts);
        let mut next: Vec<&str> = Vec::new();
        for line in curr {
            if line.as_bytes()[layer] as char == least_common {
                next.push(line);
            }
        }
        len = next.len();
        curr = next;
        layer += 1;
    }
    if curr.len() != 1 {
        panic!("Did not narrow to one result")
    }
    let result2 = curr[0];

    let result1 = i32::from_str_radix(&*result1, 2).expect(format!("Failed to parse {}", result1).as_ref());
    let result2 = i32::from_str_radix(&*result2, 2).expect(format!("Failed to parse {}", result2).as_ref());
    return result1 * result2;
}

fn count_in_space(input: &[&str], space: usize) -> (usize, usize) {
    let mut count = (0, 0);
    for line in input {
        if line.as_bytes()[space] as char == '0' {
            count.0 += 1
        } else {
            count.1 += 1
        }
    }
    return count
}

fn get_most_char(counts: (usize, usize)) -> char {
    if counts.1 >= counts.0 { '1' } else { '0' }
}

fn get_least_char(counts: (usize, usize)) -> char {
    if counts.1 > 0 && counts.1 < counts.0 { '1' } else { '0' }
}

fn count(input: &[&str]) -> Vec<usize> {
    // Find number of buckets needed
    let spaces = input[0].len();
    // Fill buckets with 0s
    let mut counts: Vec<usize> = Vec::with_capacity(spaces);
    counts.resize(spaces, 0);

    for line in input {
        for (i, ch) in line.chars().into_iter().enumerate() {
            counts[i] += if ch == '0' { 0 } else { 1 }
        }
    }
    return counts;
}