fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut reversed = 0;
    let mut number = x;
    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number /= 10;
    }

    x == reversed
}

fn main() {
    println!("{}", is_palindrome(121));
}

#[test]
fn test() {
    let x = -123;
    let res = false;
    assert_eq!(is_palindrome(x), res);
    let x = 12321;
    let res = true;
    assert_eq!(is_palindrome(x), res);
}
