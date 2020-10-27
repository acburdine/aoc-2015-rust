const INPUT: &str = "cqjxjnds";

fn password_valid(s: &String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut has_straight = false;
    let mut dbl_count = 0;

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        if i == 0 || i == chars.len() - 1 {
            i += 1;
            continue;
        }

        let c2: char = *chars.get(i + 1).unwrap();
        let c_back: char = *chars.get(i - 1).unwrap();

        if c2 == c && c_back != c {
            dbl_count += 1;
        }

        if c > c_back && c2 > c && !has_straight {
            has_straight = (c as u8) - (c_back as u8) == 1 && (c2 as u8) - (c as u8) == 1;
        }

        i += 1;
    }

    has_straight && dbl_count >= 2
}

#[test]
fn test_password_valid() {
    assert_eq!(false, password_valid(&String::from("hijklmmn")));
    assert_eq!(false, password_valid(&String::from("abbceffg")));
    assert_eq!(false, password_valid(&String::from("abbcegjk")));
    assert_eq!(true, password_valid(&String::from("abcdffaa")));
    assert_eq!(true, password_valid(&String::from("ghjaabcc")));
}

const MAX_CHAR: u8 = 'z' as u8;
const MIN_CHAR: u8 = 'a' as u8;

fn inc_password(s: String) -> String {
    let chars: Vec<char> = s.chars().rev().collect();
    let mut new_password = String::new();

    let mut carryover = true; // set carry to true initially so the first char gets bumped
    for c in chars {
        let mut i = c as u8;

        if carryover {
            i += 1;
            carryover = false;
        }

        if i > MAX_CHAR {
            carryover = true;
            i = MIN_CHAR + (i - MAX_CHAR - 1);
        }

        new_password.push(i as char);
    }

    new_password.chars().rev().collect::<String>()
}

#[test]
fn test_inc_password() {
    assert_eq!(String::from("abd"), inc_password(String::from("abc")));
    assert_eq!(String::from("aca"), inc_password(String::from("abz")));
    assert_eq!(String::from("baa"), inc_password(String::from("azz")));
}

fn next_valid_password(p: String) -> String {
    let mut password = p;

    while !password_valid(&password) {
        password = inc_password(password);
    }

    return password;
}

#[test]
fn test_next_valid_password() {
    assert_eq!(
        String::from("abcdffaa"),
        next_valid_password(String::from("abcdefgh"))
    );
    assert_eq!(
        String::from("ghjaabcc"),
        next_valid_password(String::from("ghijklmn"))
    );
}

fn main() {
    let mut password = next_valid_password(INPUT.to_string());
    println!("next valid password: {}", password);

    password = next_valid_password(inc_password(password));
    println!("next valid password: {}", password);
}
