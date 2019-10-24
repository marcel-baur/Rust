use std::cmp::Ordering;
use std::cmp;

pub fn qs(vec: &mut [i32]) {
    if vec.len() > 1  {
        let (mut pivot, mut hi) = (0, vec.len()-1);
        for _ in 0..vec.len()-1 {
            if vec[pivot] < vec[pivot+1] {
                vec.swap(pivot+1, hi);
                hi -= 1;
            } else {
                vec.swap(pivot, pivot+1);
                pivot += 1;
            }
        }
        qs(&mut vec[..pivot]);
        qs(&mut vec[pivot+1..]);
    }
}

pub fn qs2(vec: &mut [i32] ) {

    let len: usize = vec.len();
    if len <= 1 {
        return;
    }
    fn compare(num1: i32, num2: i32) -> Ordering {
        if num1 > num2 {
            return Ordering::Greater;
        }
        else if num1 < num2{ return Ordering::Less; }
        return Ordering::Equal;
    };

    let pivot: usize = 0;
    vec.swap(pivot, len / 2);

    let mut left: usize = 1;
    let mut right: usize = vec.len() - 1;

    loop {
        while left < len && compare(vec[left], vec[pivot]) != Ordering::Greater {
            left += 1
        }
        while right > 0 && compare(vec[right], vec[pivot]) != Ordering::Less {
            right -= 1
        }
        if left >= right {
            break;
        }
        vec.swap(left, right);
        left += 1;
        right -= 1;
    }
    vec.swap(pivot, right);
    qs2(&mut vec[0..cmp::min(left - 1, right)]);
    qs2(&mut vec[cmp::max(left, right + 1)..]);
}

