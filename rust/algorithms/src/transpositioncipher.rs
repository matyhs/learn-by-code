#[allow(dead_code)]
pub fn encrypt_rail_fence_cipher(message: &str, cipher_key: usize) -> String {
    let mut cache = vec![vec![' '; message.len()]; cipher_key];
    let mut row = 0;
    let mut down = false;

    for i in 0..message.len() {
        if row == 0 || row == cipher_key - 1 {
            down = !down;
        }

        cache[row][i] = message.chars().nth(i).unwrap();

        if down {
            row += 1;
        }
        else {
            row -= 1;
        }
    }

    let mut result = String::new();
    
    for i in 0..cipher_key {
        for j in 0..message.len() {
            if cache[i][j] != ' ' {
                result.push(cache[i][j]);
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn encrypt_inverted_rail_fence_cipher(message: &str, cipher_key: usize) -> String {
    let mut cache = vec![vec![' '; message.len()]; cipher_key];
    let mut down = false;
    let mut row = 0;

    for i in 0..message.len() {
        if row == 0 || row == cipher_key - 1 {
            down = !down;
        }

        cache[row][i] = '*';

        if down {
            row += 1;
        } else {
            row -= 1;
        }
    }

    let mut curr_idx = 0;
    for i in 0..cipher_key {
        for j in 0..message.len() {
            if cache[i][j] == '*' {
                cache[i][j] = message.chars().nth(curr_idx).unwrap();
                curr_idx += 1;
            }
        }
    }
    
    let mut result = String::new();
    down = false;
    row = 0;

    for i in 0..message.len() {
        if row == 0 || row == cipher_key - 1 {
            down = !down;
        }

        result.push(cache[row][i]);

        if down {
            row += 1;
        } else {
            row -= 1;
        }
    }
    
    result
}

#[allow(dead_code)]
pub fn decrypt_rail_fence_cipher(message: &str, cipher_key: usize) -> String {
    let mut cache = vec![vec![' '; message.len()]; cipher_key];
    let mut row = 0;
    let mut down = false;

    for i in 0..message.len() {
        if row == 0 || row == cipher_key - 1 {
            down = !down;
        }

        cache[row][i] = '*';

        if down {
            row += 1;
        } else {
            row -= 1;
        }
    }

    let mut curr_idx = 0;

    for i in 0..cipher_key {
        for j in 0..message.len() {
            if cache[i][j] == '*' {
                cache[i][j] = message.chars().nth(curr_idx).unwrap();
                curr_idx += 1;
            }
        }
    }

    let mut result = String::new();
    down = false;
    row = 0;

    for i in 0..message.len() {
        if row == 0 || row == cipher_key - 1 {
            down = !down;
        }

        result.push(cache[row][i]);

        if down {
            row += 1;
        } else {
            row -= 1;
        }
    }

    result
}

#[allow(dead_code)]
pub fn encrypt_scytale_cipher(message: &str, cipher_key: usize) -> String {
    let mut result = String::new();
    let length = message.len();
    let col_count = length / cipher_key;
    
    for i in 0..col_count {
        let mut curr_step = col_count;
        let remainder = length % cipher_key;

        if i < remainder {
            curr_step += 1;
        }

        for j in (i..length).step_by(curr_step) {
            if let Some(c) = message.chars().nth(j) {
                result.push(c);
            } else {
                break;
            }
        }
    }

    result
}

// fn decrypt_scytale_cipher(message: &str, cipher_key: u32) -> &str {

// }