#[allow(dead_code)]
pub fn binary_search(array: Vec<i32>, target: i32, low: usize, high: usize) -> i32 {
    if low > high {
        -1
    } else {
        let mid = low + (high - low) / 2 as usize;

        if target == array[mid] {
            return mid as i32;
        } else if target > array[mid] {
            return binary_search(array, target, mid + 1, high);
        } else {
            return binary_search(array, target, low, mid - 1);
        }
    }
}

#[allow(dead_code)]
pub fn binary_search_iterative(array: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
        let mut high = array.len() - 1;
        
        while low <= high {
            let mid = low + (high - low) / 2 as usize;
            
            if target == array[mid] {
                return mid as i32;
            } else if target < array[mid] {
                match mid.checked_sub(1) {
                    None => return -1,
                    Some(i) => high = i  
                };
            } else {
                match mid.checked_add(1) {
                    None => return -1,
                    Some(i) => low = i
                };
            }
        }
        
        -1
}