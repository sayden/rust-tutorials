fn work_on_vector(v: &Vec<i32>) {
    for e in v {
        println!("{}", e);
    }
}

fn vec_min(v: &Vec<i32>) -> Option<i32> {
    use std::cmp;

    let mut min = None;

    for e in v.iter() {
        min = Some(match min {
            None => *e,
            Some(n) => cmp::min(n, *e)
        });
    }
    min
}

fn vec_inc(v: &mut Vec<i32>) {
    for e in v.iter_mut() {
        *e += 1;
    }
}

fn main(){
    let v = vec![1,2,3,4];
    let first = &v[0];
    work_on_vector(&v);
    work_on_vector(&v);
    println!("The first element is {}", *first);

    let mut vv = v.to_owned();
    vec_inc(&mut vv);
    work_on_vector(&vv);
    work_on_vector(&v);
    let res = vec_min(&v);
    match res {
        Some(n) => println!("Min number is {}", n),
        _ => println!("Res was nothing"),
    }
}
