fn sqrt(i: i32) -> i32 { i * i }

fn abs(i: i32) -> i32 { if i >= 0 { i } else { -i } }

enum NumberOrNothing{
    Number(i32),
    Nothing
}
use self::NumberOrNothing::{Number, Nothing};

fn number_or_default(n: NumberOrNothing, default: i32)->i32 {
    match n {
        Number(n) => n,
        _ => default,
    }
}

fn compute_stuff(x: i32) -> i32 {
    let y = { let z = x*x; z + 14 };
    y*y
}

fn vec_min(v: Vec<i32>) -> NumberOrNothing {
    fn min_i32(a: i32, b: i32) -> i32 {
        match a < b {
            true => a,
            _ => b
        }
    }

    let mut min = Nothing;

    for e in v {
        min = Number(match min{
            Nothing => e,
            Number(n) => min_i32(n, e)
        })
    }

    min
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        };
    }
}

fn vec_sum(vec: Vec<i32>) -> i32 {
    let mut sum = 0;

    for e in vec {
        sum += e
    }

    sum
}

fn read_vec() -> Vec<i32> {
    vec![18,5,7,2,9,27]
}
pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    let sum = vec_sum(read_vec());
    min.print();
    println!("The sum of the vec is {}", sum );
    println!("Square root of 5 is {}", sqrt(5));
    println!("Absolute number of -1 is {}", abs(-1));
    println!("Default number is {}", number_or_default(Nothing, 0));
    println!("Compute stuff result is {}", compute_stuff(3));
}
