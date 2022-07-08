// https://learnku.com/docs/rust-lang/2018/ch02-00-guessing-game-tutorial/4497
use rand::Rng;  // trait
use std::cmp::Ordering;
use std::io;  // `use` input std io crate

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("un-expect value: {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Welcome to Guessing Number Game");

    // generate rand number
    let rand_num = rand::thread_rng().gen_range(1..=100);

    println!("rand number is {}", rand_num);

    loop {
        println!("Please input a number(1~100):");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("unable read number");
        // 类型转化，trim 提出换行和空白符，异常自动退出
        // let guess: i32 = guess.trim().parse().expect("please input a number!");
        let guess: Guess = match guess.trim().parse(){
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("please input a number!");
                continue;
            }
        };
        println!("Your input number is {}", guess.value);
    
        match guess.value().cmp(&rand_num) {
            Ordering::Equal => {
                println!("ok!");
                break;
            },
            Ordering::Less => println!("Less then random number"),
            Ordering::Greater => println!("Greater then random number"),
        }
    }
}
