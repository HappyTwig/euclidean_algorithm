use std::io;
use rand::Rng;

fn main() {
    let rand_num = rand::thread_rng().gen_range(0,1001);
    println!("Random number: {}", rand_num);

    println!("Input the number");
    let mut input_num = String::new();
    io::stdin().read_line(&mut input_num).expect("Failed to read line");
    let input_num: u32 = input_num.trim().parse().expect("Please type a number!");
    println!("You input: {}", input_num);

    let mut a = 0;
    let mut b = 0;

    if input_num >= rand_num {
        a = input_num;
        b = rand_num;
    } else {
        a = rand_num;
        b = input_num;
    }

    let mut r = 0;
    r = a % b;

    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    println!("Answer: {}", b);
}
