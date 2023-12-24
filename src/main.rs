use rand::Rng;
use std::{collections::HashMap, io};

fn dispersion(modu: u32, test: u32) -> HashMap<u32, u32> {
    let mut container = HashMap::new();
    if test > 0 && modu > 0 {
        for _i in 0..test {
            let result: u32 = rand::thread_rng().gen_range(1..=1_000_000) % modu;
            let count = container.entry(result).or_insert(0);
            *count += 1;
        }
    }
    return container;
}

fn map_print(map: HashMap<u32, u32>) {
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
}

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

    map_print(dispersion(modulus, repeat));
}
