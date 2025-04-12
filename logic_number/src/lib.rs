pub fn number_logic(num: u32) -> bool {
        let num_str = num.to_string();

        let digit_count = num_str.len() as u32;
        let sum: u64 = num_str
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .map(|digit| digit.pow(digit_count))
            .sum();

        sum == num as u64
}