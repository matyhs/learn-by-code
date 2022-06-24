/// use bijective function
#[allow(dead_code)]
pub fn base_62_encode(key: usize) -> String {
    let base62_alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut result = String::new();
    let mut curr_idx;
    let mut curr_key = key;

    while curr_key > 0 {
        curr_idx = curr_key % 62;
        result.push(base62_alphabet.chars().nth(curr_idx).unwrap());
        println!("{}", format!("{} : {}", curr_idx, result));
        curr_key /= 62;
    }

    result.chars().rev().collect()
}