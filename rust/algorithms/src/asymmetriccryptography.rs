pub struct AsymmetricKey {
    pub encryption_key: usize,
    pub decryption_key: usize,
    /// n is the value representing the first part of the public key.
    pub n: usize
}

impl AsymmetricKey {
    pub fn new(e: usize, d: usize, n: usize) -> AsymmetricKey {
        AsymmetricKey { encryption_key: e, decryption_key: d, n: n }
    } 
}

#[allow(dead_code)]
/// Rivest-Shamir-Adleman
/// m and n should be random prime integers
pub fn generate_rsa_asymmetrickey(p: usize, q: usize) -> AsymmetricKey {
    // Step 1: calculate for the first part of the public key, n
    let n: usize = p * q;
    // Step 2: calculate for phi using Euler's totient function. phi represents the number of co-prime numbers from 1 to n
    let phi: usize = (p - 1) * (q - 1);
    // Step 3: pick a co-prime number between 1 and phi of n using Euclid's algorithm (GCD). This will serve as the second part of the public key.
    let mut e: usize = 2;
    while e < phi {
        if crate::numbertheory::calculate_gcd_euclid(e, phi) == 1 {
            break;
        } else {
            e += 1;
        }
    }
    // Step 4: calculate for the private key by finding for the modular multiplicative inverse.
    let d: usize = crate::numbertheory::calculate_modular_multiplicative_inverse_naive(e, phi);
    
    AsymmetricKey::new(e, d, n)
}