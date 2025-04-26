pub fn first_fifty_even_square() -> Vec<i32> {
    (0..)
    .filter(|&n| n % 2 == 0).take(50).map(|n| n * n).collect()
}
