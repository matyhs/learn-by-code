pub fn calculate_gcd_euclid(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }

    calculate_gcd_euclid(b % a, a)
}

pub fn calculate_modular_multiplicative_inverse_naive(a: usize, m: usize) -> usize {
    for i in 1..m {
        if (a * i) % m == 1 {
            return i;
        }
    }

    1
}