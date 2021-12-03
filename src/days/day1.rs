pub(crate) fn day1_a(input: &[i32]) -> i32 {
    // Split input into 2-item overlapping pairs
    let pairs = input.windows(2).into_iter();

    // If the second item in the pair is larger, add 1.
    pairs.fold(0, |a, it| {
        a + it.into_iter().reduce(|&a, &b| if a < b { &1 } else { &0 }).unwrap()
    })
}

pub(crate) fn day1_b(input: &[i32]) -> i32 {
    // Split things into 3-item windows
    let windows = input.windows(3).into_iter();

    // Sum each of the windows and collect the sums
    let rolling_sums = windows
        .map(|current| {
            current.into_iter().sum()
        })
        .into_iter()
        .collect::<Vec<i32>>();

    // Do the same thing we did above
    day1_a(rolling_sums.as_slice())
}
