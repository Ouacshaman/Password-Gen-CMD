//Password Generator
use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Hello, world!");

    let mut i=0;
    let length = config.length.parse().unwrap();
    let low_alph = ('a'..='z').map(|c| c as char).collect::<String>();
    let up_alph = ('A'..='Z').map(|c| c as char).collect::<String>();
    let comb_alph = low_alph.chars().chain(up_alph.chars()).collect::<String>();
    let symbols = "!@#$%^&*()-_+={}[]|\\:;'<,>.?/".to_string();
    let mut rng = rand::thread_rng();

    let mut password = " ".to_string();
    while i < length{
        password += &comb_alph[rng.gen_range(0..5)..rng.gen_range(6..10)];
        password += &symbols[rng.gen_range(0..5)..rng.gen_range(6..10)];
        i+=1;
        let random_integer: u32 = rng.gen_range(0..101);
        password += random_integer.to_string().as_str();
    }
    print!("{:?}",password);

}

struct Config{
    length: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 2{
            panic!("not enough arguments");
        }
        let length = args[1].clone();

        Config {length}
    }
}


