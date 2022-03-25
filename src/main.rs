use std::collections::HashMap;

fn main() {
    println!("Welcome to the LNL!");
}

fn iterative_nth_fibonacci(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for _ in 0..n {
        let c: u128 = a;
        a = b;
        b = a + c;
    }

    a
}

fn recursive_nth_fibonacci(n: u32) -> u128 {
    if n <= 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return recursive_nth_fibonacci(n - 1) + recursive_nth_fibonacci(n - 2);
}

fn _dynamic_nth_fibonacci(n: u32, series: &mut HashMap<u32, u128>) -> u128 {
    if n <= 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    match series.get(&n) {
        Some(k) => *k,
        None => {
            let a = _dynamic_nth_fibonacci(n - 1, series);
            let b = _dynamic_nth_fibonacci(n - 2, series);
            series.insert(n, a + b);
            *series.get(&n).unwrap()
        }
    }
}

fn dynamic_nth_fibonacci(n: u32) -> u128 {
    let mut series: HashMap<u32, u128> = HashMap::new();

    _dynamic_nth_fibonacci(n, &mut series)
}

#[cfg(test)]
mod tests {
    use super::{iterative_nth_fibonacci, dynamic_nth_fibonacci, recursive_nth_fibonacci};

    #[test]
    fn test_linear_nth_fibonacci() {
        assert_eq!(iterative_nth_fibonacci(0), 0);
        assert_eq!(iterative_nth_fibonacci(1), 1);
        assert_eq!(iterative_nth_fibonacci(2), 1);
        assert_eq!(iterative_nth_fibonacci(3), 2);
        assert_eq!(iterative_nth_fibonacci(4), 3);
        assert_eq!(iterative_nth_fibonacci(100), 354224848179261915075);
        assert_eq!(iterative_nth_fibonacci(185), 205697230343233228174223751303346572685);
    }

    #[test]
    #[should_panic]
    fn test_linear_nth_fibonacci_186() {
        iterative_nth_fibonacci(186);
    }

    #[test]
    fn test_recursive_nth_fibonacci() {
        assert_eq!(recursive_nth_fibonacci(0), 0);
        assert_eq!(recursive_nth_fibonacci(1), 1);
        assert_eq!(recursive_nth_fibonacci(2), 1);
        assert_eq!(recursive_nth_fibonacci(3), 2);
        assert_eq!(recursive_nth_fibonacci(4), 3);
        // Commented these out because the CI credit usage would be enormous
        // assert_eq!(recursive_nth_fibonacci(100), 354224848179261915075);
        // assert_eq!(recursive_nth_fibonacci(185), 205697230343233228174223751303346572685);
    }

    // #[test]
    // #[should_panic]
    // fn test_recursive_nth_fibonacci_186() {
    //     recursive_nth_fibonacci(186);
    // }

    #[test]
    fn test_dynamic_nth_fibonacci() {
        assert_eq!(dynamic_nth_fibonacci(0), 0);
        assert_eq!(dynamic_nth_fibonacci(1), 1);
        assert_eq!(dynamic_nth_fibonacci(2), 1);
        assert_eq!(dynamic_nth_fibonacci(3), 2);
        assert_eq!(dynamic_nth_fibonacci(4), 3);
        assert_eq!(dynamic_nth_fibonacci(100), 354224848179261915075);
        assert_eq!(dynamic_nth_fibonacci(185), 205697230343233228174223751303346572685);
        assert_eq!(dynamic_nth_fibonacci(186), 332825110087067562321196029789634457848);
    }

    #[test]
    #[should_panic]
    fn test_dynamic_nth_fibonacci_186() {
        dynamic_nth_fibonacci(187);
    }
}