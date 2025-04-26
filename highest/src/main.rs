Here is a program to test your function.

use highest::*;

fn main() {
    let expected = [30, 500, 20, 70];
    let n = Numbers::new(&expected);
    println!("{:?}", n.list());
    println!("{:?}", n.highest());
    println!("{:?}", n.latest());
    println!("{:?}", n.highest_three());
}
