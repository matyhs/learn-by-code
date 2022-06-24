#[cfg(test)]
mod tests {
    #[test]
    fn fibonacci_test() {
        assert_eq!(crate::fibonacci(50, &mut std::collections::HashMap::new()), 12586269025);
    }

    #[test]
    fn fibonacci_tabulation_test()
    {
        assert_eq!(crate::fibonacci_tabulation(50), 12586269025);
    }

    #[test]
    fn grid_traveller_test() {
        assert_eq!(crate::grid_traveller(18, 18, &mut std::collections::HashMap::new()), 2333606220);
    }

    #[test]
    fn can_sum_test() {
        assert_eq!(crate::can_sum(300, &vec![7, 14], &mut std::collections::HashMap::new()), false);
    }
}

pub fn fibonacci(n: usize, mapping: &mut std::collections::HashMap::<usize, usize>) -> usize {
    if n <= 2 {
        return 1;
    }

    if mapping.contains_key(&n) {
        return mapping[&n];
    }

    let first = fibonacci(n - 1, mapping);
    let second = fibonacci(n - 2, mapping);

    mapping.insert(n,  first + second );

    mapping[&n]
}

pub fn fibonacci_tabulation(n: usize) -> usize {
    let mut results= vec![0; n + 1];
    results[1] = 1;

    for i in 0..n + 1 {
        if i + 1 < n + 1
        {
            results[i + 1] += results[i];
        }
        
        if i + 2 < n + 1
        {
            results[i + 2] += results[i];
        }
    }

    results[n]
}

#[derive(Eq, Hash)]
pub struct Grid {
    width: usize,
    height: usize
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid { width, height }
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

pub fn grid_traveller(m: usize, n: usize, memo: &mut std::collections::HashMap::<Grid, usize>) -> usize {
    if m == 1 && n == 1 {
        return 1;
    }

    if m == 0 || n == 0 {
        return 0;
    }
    
    let grid = Grid::new(m, n);

    if memo.contains_key(&grid) {
        return memo[&grid];
    }

    let first = grid_traveller(m - 1, n, memo);
    let second = grid_traveller(m, n - 1, memo);

    memo.insert(grid, first + second);

    return first + second;
}

pub fn can_sum(target: usize, numbers: &Vec<usize>, memo: &mut std::collections::HashMap::<usize, bool>) -> bool {
    if target == 0 {
        return true;
    }

    if memo.contains_key(&target) {
        return memo[&target];
    }

    for num in numbers {
        match target.checked_sub(*num) {
            Some(i) => {
                if can_sum(i, &numbers, memo) {
                    memo.insert(target, true);
                    return memo[&target];
                }
            },
            None => ()
        };
    }

    memo.insert(target, false);
    false
}
