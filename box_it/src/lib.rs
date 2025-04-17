pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut result = Vec::new();

    for num_str in s.split_whitespace() {
        if num_str.ends_with('k') {
            let num_without_k = &num_str[0..num_str.len() -1];
            if let Ok(num) = num_without_k.parse::<f32>() {
                result.push((num * 1000.0) as u32);
            }
        } else {
            // Parsee directly as f32 and convert to u32
            if let Ok(num) = num_str.parse::<f32>() {
                result.push(num as u32);
            }
        }
    }

    // Box the vector and return it
    Box::new(result)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    // Unboxing the vector by dereferencing the Box
    *a
}