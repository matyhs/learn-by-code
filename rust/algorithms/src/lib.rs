mod search;
mod sort;
mod maxsubarray;
mod transpositioncipher;
mod encoding;
mod asymmetriccryptography;
mod numbertheory;

#[cfg(test)]
mod search_tests {
    #[test]
    fn binary_search_works() {
        let arrange = vec![2, 5, 6, 8, 9, 10 ];
        let target = 5;
        let low = 0;
        let high = arrange.len() - 1;
        assert_eq!(crate::search::binary_search(arrange, target, low, high), 1);
    }

    #[test]
    fn binary_search_iterative_works() {
        let arrange = vec![2, 5, 6, 8, 9, 10 ];
        let target = 5;
        assert_eq!(crate::search::binary_search_iterative(arrange, target), 1);
    }
}

#[cfg(test)]
mod sort_tests {
    #[test]
    fn quick_sort_works() {
        let mut numbers = vec![8, 4, 6, 5, 10, 9];
        crate::sort::quick_sort(&mut numbers, 0, 5);
        assert_eq!(numbers, vec![4, 5, 6, 8, 9, 10]);
    }

    #[test]
    fn insertion_sort_works() {
        let mut numbers = vec![8, 4, 6, 5, 10, 9];
        crate::sort::insertion_sort(&mut numbers);
        assert_eq!(numbers, vec![4, 5, 6, 8, 9, 10]);
    }

    #[test]
    fn merge_sort_works() {
        let mut numbers = vec![8, 4, 6, 5, 10, 9];
        crate::sort::merge_sort(&mut numbers, 0, 5);
        assert_eq!(numbers, vec![4, 5, 6, 8, 9, 10]);
    }

    #[test]
    fn bubble_sort_works() {
        let mut numbers = vec![8, 4, 6, 5, 10, 9];
        crate::sort::bubble_sort(&mut numbers);
        assert_eq!(numbers, vec![4, 5, 6, 8, 9, 10]); 
    }
}

#[cfg(test)]
mod maxsubarray_tests {
    #[test]
    fn maxsubarray_dac_works() {
        let numbers = vec![13, -3, 25, 20, -3, -16, -23];
        assert_eq!(crate::maxsubarray::max_subarray_dac(&numbers, 0, 6), 55);
    }

    #[test]
    fn max_subarray_kandane_works() {
        let numbers = vec![13, -3, 25, 20, -3, -16, -23];
        assert_eq!(crate::maxsubarray::max_subarray_kandane(&numbers), 55);
    }
}

#[cfg(test)]
mod transpositioncipher_tests {
    #[test]
    fn encrypt_scytale_cipher_works() {
        let message = "WEAREDISCOVEREDFLEEATONCE";
        assert_eq!(crate::transpositioncipher::encrypt_scytale_cipher(message, 3), "WOEEVEAEARRTEEODDNIFCSLEC");
    }

    #[test]
    fn encrypt_rail_fence_cipher_works() {
        let message = "WEAREDISCOVEREDFLEEATONCE";
        assert_eq!(crate::transpositioncipher::encrypt_rail_fence_cipher(message, 3), "WECRLTEERDSOEEFEAOCAIVDEN");
    }

    #[test]
    fn decrypt_rail_fence_cipher_works() {
        let message = "WECRLTEERDSOEEFEAOCAIVDEN";
        assert_eq!(crate::transpositioncipher::decrypt_rail_fence_cipher(message, 3), "WEAREDISCOVEREDFLEEATONCE");
    }

    #[test]
    fn encrypt_inverted_rail_fence_cipher() {
        let message = "WEAREDISCOVEREDFLEEATONCE";
        assert_eq!(crate::transpositioncipher::encrypt_inverted_rail_fence_cipher(message, 3), "WSACEOTVAEORRENDEFCLDEEEI");
    }
}

#[cfg(test)]
mod encoding_tests {
    #[test]
    fn base_62_encode_works() {
        let key: usize = 124323;
        assert_eq!(crate::encoding::base_62_encode(key), "WLD");
    }
}

#[cfg(test)]
mod math_tests {
    #[test]
    fn calculate_gcd_euclid_works() {
        assert_eq!(crate::numbertheory::calculate_gcd_euclid(7, 120), 1);
    }

    #[test]
    fn calculate_modular_multiplicative_inverse_works() {
        assert_eq!(crate::numbertheory::calculate_modular_multiplicative_inverse_naive(2, 7), 4);
    }
}

#[cfg(test)]
mod asymmetriccryptography_tests {
    #[test]
    fn generate_rsa_asymmetrickey() {
        let key = crate::asymmetriccryptography::generate_rsa_asymmetrickey(11, 13);
        assert_eq!(key.encryption_key, 7);
        assert_eq!(key.n, 143);
        assert_eq!(key.decryption_key, 103);
    }
}