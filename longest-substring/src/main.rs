fn main() {
    let s1 = "abcabcbb".to_string();
    let s2 = "bbbbb".to_string();
    let s3 = "pwwkew".to_string();

    println!("{:?}", length_of_longest_substring(s1));
    println!("{:?}", length_of_longest_substring(s2));
    println!("{:?}", length_of_longest_substring(s3));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let length = s.len();

    if length == 0 {
        0
    }
    else
    {
        let bytes = s.as_bytes();
        let mut window = [0u8; 128];

        let mut left: usize = 0;
        let mut max_length: i32 = 0;

        // 依次找出各个无重复子字符串
        for right in 0..length {
            window[bytes[right] as usize] += 1;

            loop {
                if window[bytes[right] as usize] <= 1 {
                    break;  // 无重复字符
                }

                // 当前字符存在重复
                // 窗的左边右移
                window[bytes[left] as usize] -= 1;
                left += 1;
            }

            let len: i32 = (right - left + 1) as i32;

            if len > max_length {
                max_length = len;
            }
        }

        max_length
    }
}

#[test]
fn length_of_longest_substring_test1() {
    let s = "abcabcbb".to_string();

    assert_eq!(length_of_longest_substring(s), 3);
}

#[test]
fn length_of_longest_substring_test2() {
    let s = "bbbbb".to_string();

    assert_eq!(length_of_longest_substring(s), 1);
}

#[test]
fn length_of_longest_substring_test3() {
    let s = "pwwkew".to_string();

    assert_eq!(length_of_longest_substring(s), 3);
}