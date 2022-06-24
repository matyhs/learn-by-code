#[allow(dead_code)]
pub fn max_subarray_dac(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
    if start >= end {
        return nums[start];
    }

    let mid = (start + end) / 2;
    let left = max_subarray_dac(nums, start, mid);
    let right = max_subarray_dac(nums, mid + 1, end);
    let cross = max_subarray_cross(nums, start, mid, end);

    let result = std::cmp::max(left, right);

    std::cmp::max(result, cross)
}

fn max_subarray_cross(nums: &Vec<i32>, start: usize, mid: usize, end: usize) -> i32 {
    let mut left_sum = i32::MIN;
    let mut sum = 0;

    for i in (start..=mid).rev() {
        sum += nums[i];
        if sum > left_sum {
            left_sum = sum;
        }
    }

    sum = 0;
    let mut right_sum = i32::MIN;

    for i in mid + 1..=end {
        sum += nums[i];
        if sum > right_sum {
            right_sum = sum;
        }
    }

    left_sum + right_sum
}

#[allow(dead_code)]
pub fn max_subarray_kandane(nums: &Vec<i32>) -> i32 {
    let mut running_max = 0;
    let mut max = 0;

    for i in nums {
        running_max += *i;
        running_max = std::cmp::max(running_max, 0);
        max = std::cmp::max(max, running_max);
    }

    max
}
