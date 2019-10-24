use std::cmp::Ordering;
use std::cmp;
mod quicksort;// a
fn collatz(c: i64) -> i64{
    if c%2 == 0 {
       return c/2
    }
    return 3*c + 1

}
// b, c
fn b(num: i64, i: i64) -> i64 {
    if num != 1 {
        let coll = collatz(num);

        return b(coll, i+1);

    }
    return i;
}

// c
fn c(mut number: i64) -> (i32, i64) {
    let mut iterator = 0;
    let mut max = number;
    while number != 1 {
        let coll = collatz(number);
        if max < coll {
            max = coll;
        }
        number = coll;
        iterator += 1;

    }
    (iterator, max)
}


fn main() -> () {
    let mut s = [3, 6, 7, 5, 2, 1, 4, 8];
    quicksort::qs2(&mut s);
    let mut p = [3, 6, 7, 5, 2, 1, 4, 8];
    let mut t = [3, 6, 7, 5, 2, 1, 4, 8];

    quicksort::qs(&mut p);
    quicksort::qs_generic(&mut t, &compare);

    dbg!(p);
    println!("{:?}", s);
    println!("{:?}", p);

}

fn compare(num1: &i32, num2: &i32) -> Ordering {
    if num1 > num2 {
        return Ordering::Greater;
    }
    else if num1 < num2{ return Ordering::Less; }
    return Ordering::Equal;
}
