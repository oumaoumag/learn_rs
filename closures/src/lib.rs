fn first_fifty_even_square() -> Vec<i32> {
    (0..).step_by(2)
        .take(50)
        .map(|n| n * n)
        .collect()
}