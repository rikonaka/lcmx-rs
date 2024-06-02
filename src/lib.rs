use gcdx::gcd_euclidean;

fn lcm(a: usize, b: usize) -> usize {
    let tmp = a * b;
    let g = gcd_euclidean(a, b).unwrap();
    tmp / g
}

pub fn lcmx(values: &[usize]) -> Option<usize>
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
    fn run() {
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
