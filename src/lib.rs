use gcdx::gcdx;
use std::fmt::Display;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;

fn lcm<T>(a: T, b: T) -> T
where
    T: Rem<Output = T>
        + PartialOrd
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Display
        + Clone
        + gcdx::Zero,
{
    let tmp = a * b;
    let g = gcdx(&vec![a, b]).unwrap();
    tmp / g
}

pub fn lcmx<T>(values: &[T]) -> Option<T>
where
    T: Rem<Output = T>
        + PartialOrd
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Display
        + Clone
        + gcdx::Zero,
{
    if values.len() > 0 {
        let mut m = values[0];
        for i in 1..values.len() {
            m = lcm(m, values[i]);
            // println!("{}", m);
        }
        Some(m)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run() {
        let v = vec![1, 2, 3, 4];
        let l = lcmx(&v).unwrap();
        println!("{}", l);
        assert_eq!(l, 12);

        let v = vec![1, 2, 3, 99];
        let l = lcmx(&v).unwrap();
        println!("{}", l);
        assert_eq!(l, 198);

        let v = vec![1, 2];
        let l = lcmx(&v).unwrap();
        println!("{}", l);
        assert_eq!(l, 2);

        let v = vec![1];
        let l = lcmx(&v).unwrap();
        println!("{}", l);
        assert_eq!(l, 1);
    }
}
