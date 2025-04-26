pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_curry() {
        let add10 = add_curry(10);
        let add20 = add_curry(20);
        let add30 = add_curry(30);

        assert_eq!(add10(5), 15);
        assert_eq!(add20(195), 215);
        assert_eq!(add30(5696), 5726);
    }
}