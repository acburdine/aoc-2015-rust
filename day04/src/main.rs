extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

// references https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7 for solution info
fn main() {
    let input = "yzbqklnj".as_bytes();
    let mut hasher = Md5::new();
    let mut part_one = false;

    for i in 0..std::u64::MAX {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        if !part_one && output[..2] == [0, 0] && output[2] <= 0x0F {
            println!("{}", i);
            part_one = true;
        }

        if output[..3] == [0, 0, 0] {
            println!("{}", i);
            break;
        }

        hasher.reset();
    }
}
