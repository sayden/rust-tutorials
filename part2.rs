enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}

use self::SomethingOrNothing::*;

type NumberOrNothing = SomethingOrNothing<f32>;

pub trait Minimum: Copy {
    fn min(self, b: Self) -> Self;
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

impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

impl Minimum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

impl NumberOrNothing {
    pub fn print(self){
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is {}", n),
        }
    }
}

fn read_vec() -> Vec<f32> {
    vec![18.0, 5.0, 7.0, 3.0, 9.0, 27.0]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
