#[cfg(test)]
mod tests {
    #[test]
    fn get_sign_bit_works() {
        let value = crate::decode_f32(-42.42);
        assert_eq!(crate::get_sign_bit(value), 1);
    }

    #[test]
    fn get_exponent_works() {
        let value = crate::decode_f32(-42.42);
        assert_eq!(crate::get_exponent(value), 5);
    }
}

pub fn decode_f32(n: f32) -> u32 {
    n.to_bits()
}

pub fn get_sign_bit(n: u32) -> u32 {
    n >> 31
}

pub fn get_exponent(n: u32) -> u32 {
    let remove_mantissa = n >> 23;
    let exponent = remove_mantissa & 0xff;
    exponent - 127
}

pub fn get_mantissa(n: u32) -> u32 {
    
}
