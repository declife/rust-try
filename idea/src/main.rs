mod data_type;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let rec = Rectangle {
        width: 23,
        height: 24
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_by_rec(&rec)
    );

    println!(
        "Rectangle is {:#?}",
        rec
    );

    println!("Guess the number! {}", rec.height);

    println!("Please input your guess:");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let guess = "Hi".trim();
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("{}", guess)
}


#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,

}

impl Rectangle {

    fn area(&self) -> u32 {
        (self.width * self.height) as u32
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
fn area(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn area_by_rec(rec: &Rectangle) -> u32 {
    (rec.height * rec.width) as u32
}

enum Test {
    OK,
    ERR,
}

impl Test {

    fn handle() {

    }

    fn hand_(&self) {

    }
}
