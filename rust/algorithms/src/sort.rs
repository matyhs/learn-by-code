#[allow(dead_code)]
pub fn quick_sort(nums: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let pivot = nums[end];
    let pivot_index = partition(nums, pivot, start, end);

    match pivot_index.checked_sub(1) {
        Some(i) => quick_sort(nums, start, i),
        None => ()
    };

    match pivot_index.checked_add(1) {
        Some(i) => quick_sort(nums, i, end),
        None => ()
    };
}

fn partition(nums: &mut Vec<i32>, pivot: i32, start: usize, end: usize) -> usize {
    let mut start_index = start;
    let mut index = start;

    while index < end {
        if nums[index] < pivot {
            swap(nums, index, start_index);
            start_index += 1;
        }

        index += 1;
    }

    swap(nums, end, start_index);
    start_index
}

fn swap(nums: &mut Vec<i32>, from_idx: usize, to_idx: usize) {
    let temp = nums[from_idx];
    nums[from_idx] = nums[to_idx];
    nums[to_idx] = temp;
}

#[allow(dead_code)]
pub fn insertion_sort(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        let key = nums[i];
        let mut j = i - 1;
        let mut stopped = false;
        while key < nums[j] {
            nums[j + 1] = nums[j];
            if let Some(n) = j.checked_sub(1) {
                j = n;
            } else {
                stopped = true;
                break;
            }
        }

        if stopped {
            nums[j] = key;
        } else {
            nums[j + 1] = key;
        }
    }
}

#[allow(dead_code)]
pub fn merge_sort(nums: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mid = (start + end) / 2;

    merge_sort(nums, start, mid);
    merge_sort(nums, mid + 1, end);
    merge(nums, start, mid, end);
}

fn merge(nums: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    let mut r_hand = Vec::<i32>::new();
    let mut l_hand = Vec::<i32>::new();

    for i in start..=mid {
        l_hand.push(*nums.get(i).unwrap());
    }

    for i in mid + 1..=end {
        r_hand.push(*nums.get(i).unwrap());
    }

    let mut l_hand_idx = 0;
    let mut r_hand_idx = 0;

    for i in start..=end {
        if l_hand_idx >= l_hand.len() {
            nums[i] = r_hand[r_hand_idx];
            r_hand_idx += 1;
        } else if r_hand_idx >= r_hand.len() {
            nums[i] = l_hand[l_hand_idx];
            l_hand_idx += 1;
        } else if l_hand[l_hand_idx] < r_hand[r_hand_idx] {
            nums[i] = l_hand[l_hand_idx];
            l_hand_idx += 1;
        } else {
            nums[i] = r_hand[r_hand_idx];
            r_hand_idx += 1;
        }
    }
}

#[allow(dead_code)]
pub fn bubble_sort(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        for j in (i..nums.len()).rev() {
            if nums[j] < nums[j - 1] {
                let temp = nums[j];
                nums[j] = nums[j - 1];
                nums[j - 1] = temp;
            }
        }
    }
}