fn main() {
    println!("Hello, world!");
}

pub fn reverse(x: i32) -> i32 {
    let mut y: i32 = x;
    let mut r: i32 = 0;

    loop {
        r += y % 10;
        y /= 10;

        if y == 0 {
            break;
        }

        if r > 214748364/*7*/ || r < -214748364/*8*/ {
            return 0;
        }

        r *= 10;
    }

    r
}

#[test]
fn reverse_123() {
    let expected = 321;
    let actual = reverse(123);

    assert_eq!(expected, actual);
}

#[test]
fn reverse_neg123() {
    let expected = -321;
    let actual = reverse(-123);

    assert_eq!(expected, actual);
}

#[test]
fn reverse_120() {
    let expected = 21;
    let actual = reverse(120);

    assert_eq!(expected, actual);
}

#[test]
fn reverse_0() {
    let expected = 0;
    let actual = reverse(0);

    assert_eq!(expected, actual);
}

#[test]
fn reverse_2147483647() {
    let expected = 0;
    let actual = reverse(2147483647);

    assert_eq!(expected, actual);
}

#[test]
fn reverse_neg2147483648() {
    let expected = 0;
    let actual = reverse(-2147483648);

    assert_eq!(expected, actual);
}
