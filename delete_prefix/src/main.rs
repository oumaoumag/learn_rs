use delete_prefix::delete_prefix;

fn main() {
    print!("{:?}", delete_prefix("ab", "abcdefghiklmnop"));
    print!("{:?}", delete_prefix("x", "abcdefghiklmnop"));
}