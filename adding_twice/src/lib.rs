pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}


pub fn twice<F>(f: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    move |x| f(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twice() {
        let add10 = add_curry(10);
        let value = twice(add10);
        assert_eq!(value(7), 27);

        let add20 = add_curry(20);
        let value = twice(add20);
        assert_eq!(value(7), 47);

        let add30 = add_curry(30);
        let value = twice(add30);
        assert_eq!(value(7), 67);

        let neg = add_curry(-32);
        let value = twice(neg);
        assert_eq!(value(7), -57);
    }
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