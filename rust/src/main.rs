use rand::Rng;
use std::env;

const DEFAULT_LENGTH: u32 = 12;
const POOL: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()";

fn main() {
    let pool_size = POOL.len();

    let mut rng = rand::thread_rng();
    let args: Vec<String> = env::args().collect();

    let password_length: u32 = if args.len() >= 2 {
        let length = args[1].trim().parse().expect("Please provide numeric length");
        if length < 1 || length > 9999 {
            DEFAULT_LENGTH
        } else {
            length
        }
    } else {
        DEFAULT_LENGTH
    };

    let mut password = String::from("");

    for _ in 0..password_length {
        let pool_index = rng.gen_range(0, pool_size);
        let rand_char = &POOL[pool_index..pool_index + 1];
        password.push_str(rand_char);
    }

    println!("{}", password);
}
