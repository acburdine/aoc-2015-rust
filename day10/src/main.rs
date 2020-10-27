const INPUT: &str = "1113222113";

fn process(s: String) -> String {
    let mut cur_char: char = ' ';
    let mut cur_count: u32 = 0;

    let mut new_string = String::new();

    for c in s.chars() {
        if c != cur_char {
            if cur_count > 0 {
                new_string.push_str(&cur_count.to_string());
                new_string.push(cur_char);
            }

            cur_count = 1;
            cur_char = c;
        } else {
            cur_count += 1;
        }
    }

    if cur_count > 0 {
        new_string.push_str(&cur_count.to_string());
        new_string.push(cur_char);
    }

    new_string
}

fn main() {
    let mut result: String = INPUT.to_string();

    let mut i = 0;
    while i < 50 {
        if i == 40 {
            println!(
                "length of result after 40 times: {}",
                result.chars().count()
            );
        }

        result = process(result);
        i += 1;
    }

    println!(
        "length of result after 50 times: {}",
        result.chars().count()
    );
}
