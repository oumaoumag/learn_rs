fn main() {
    let v = "Hello, Rust!";
    let index = 5;

    let (fst_part, snd_part) = v.split_at(index);

    println!("First part; {}", fst_part);
    println!("Second part; {}", snd_part);
}