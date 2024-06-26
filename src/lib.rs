use gcdx::gcd;
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
    let g = gcd(a, b);
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
    fn test_lcmx() {
        let v: Vec<usize> = vec![1, 2, 3, 4];
        let l = lcmx(&v).unwrap();
        // println!("{}", l);
        assert_eq!(l, 12);

        let v: Vec<usize> = vec![1, 2, 3, 99];
        let l = lcmx(&v).unwrap();
        // println!("{}", l);
        assert_eq!(l, 198);

        let v: Vec<usize> = vec![1, 2];
        let l = lcmx(&v).unwrap();
        // println!("{}", l);
        assert_eq!(l, 2);

        let v: Vec<usize> = vec![1];
        let l = lcmx(&v).unwrap();
        // println!("{}", l);
        assert_eq!(l, 1);
    }
}
