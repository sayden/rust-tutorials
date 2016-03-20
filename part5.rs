#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>,
}


impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    pub fn from_vec(v: Vec<u64>) -> Self {
        BigInt { data: (&v).clone() }
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn non_zero(&self) -> usize {
        let mut size = 0;
        for &v in self.data.iter() {
            if v != 0 {
                size += 1;
            }
        }

        size
    }

    pub fn min(&self) -> u64 {
        let mut min = self.data[0];
        for &v in self.data.iter() {
            if v < min {
                min = v;
            }
        }
        min
    }

    pub fn max(&self) -> u64 {
        let mut max = 0;
        for &v in self.data.iter() {
            if v > max {
                max = v;
            }
        }

        max
    }
}


fn clone_demo() {
    let v = vec![1, 4, 5, 5, 6, 7, 0, 9, 8, 2, 0, 2];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v.clone());
    let l = BigInt::length(&b1);
    println!("Size is {}", l);
    println!("Non zeros are {}", BigInt::non_zero(&b2));
    println!("Lowest number is {}", BigInt::min(&b2));
    println!("Highest number is {}", BigInt::max(&b2));
}

fn main() {
    clone_demo();
}
