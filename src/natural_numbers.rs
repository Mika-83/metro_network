fn factorial(n: i32) -> i32 {
    let res = match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n-1),
    };
    res
}

fn power(i: i32, j: i32) -> i32 {
    let res = match j {
        0 => 1,
        1 => i,
        _ => i * power(i, j-1),
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorial_1() {
        assert_eq!(factorial(0), 1)
    }
    #[test]
    fn test_factorial_2() {
        assert_eq!(factorial(1), 1)
    }
    #[test]
    fn test_factorial_3() {
        assert_eq!(factorial(2), 2)
    }
    #[test]
    fn test_factorial_4() {
        assert_eq!(factorial(10), 3628800)
    }
    #[test]
    fn test_power_1() {
        assert_eq!(power(3, 0), 1)
    }
    #[test]
    fn test_power_2() {
        assert_eq!(power(3, 1), 3)
    }
    #[test]
    fn test_power_3() {
        assert_eq!(power(3, 2), 9)
    }
}