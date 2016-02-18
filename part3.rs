use std::io::prelude::*;
use std::io;

enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}

use self::SomethingOrNothing::*;

fn read_vec() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::<i32>::new();

    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.trim().parse::<i32>() {
            Ok(num) => {
                vec.push(num)
            },
            Err(_) => {
                println!("What did I say about numbers?")
            }
        }
    }

    vec
}

fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            Something(n) => {
                e.min(n)
            }
        });
    }
    min
}

trait Minimum: Copy {
    fn min(self, b: Self) -> Self;
}

impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

trait Print {
    fn print(self);
}

impl Print for SomethingOrNothing<i32> {
    fn print(self) {
        match self {
            Nothing => println!("Nothing"),
            Something(n) => println!("Number: {}", n),
        }
    }
}

impl Print for SomethingOrNothing<f32> {
    fn print(self) {
        match self {
            Nothing => println!("Nothing"),
            Something(n) => println!("Number: {}", n),
        }
    }
}

pub fn main(){
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
