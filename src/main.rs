use rand::Rng;
use std::io;

fn main() {
    println!("How random is the random crate?");
    println!("What is the modulus?");

    let mut mod_in = String::new();
    io::stdin()
        .read_line(&mut mod_in)
        .expect("Failure to read line.");

    let modulus: u32 = match mod_in.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("How many tests?");
    let mut repeat_in = String::new();
    io::stdin()
        .read_line(&mut repeat_in)
        .expect("Failure to read line");

    let repeat: u32 = match repeat_in.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut container: Vec<u32> = Vec::new();
    let mut i = 0;
    if modulus > 0 && repeat > 0 {
        while i < repeat {
            let rand_num: u32 = rand::thread_rng().gen_range(1..=1000000);
            container.push(rand_num % &modulus);
            i = i + 1;
        }
        i = 0;
        while i < modulus {
            let count = container.iter().filter(|&n| *n == i).count();
            println!("{}: {}", &i, &count);
            i = i + 1;
        }
    }
}
