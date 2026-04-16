use std::collections::HashMap;

fn paths(x: u32, y: u32, grid_size: u32, memo: &mut HashMap<(u32, u32), u64>) -> u64 {
    if x == grid_size && y == grid_size {
        return 1;
    }
    if x > grid_size || y > grid_size {
        return 0;
    }
    if let Some(result) = memo.get(&(x, y)) {
        return *result;
    }

    let result = paths(x + 1, y, grid_size, memo) + paths(x, y + 1, grid_size, memo);
    memo.insert((x, y), result);
    result
}

pub fn solve_with_memoization(grid_size: u32) -> u64 {
    let mut memo = HashMap::new();
    paths(0, 0, grid_size, &mut memo)
}

pub fn solve_with_combinatorics(grid_size: u64) -> u64 {
    let moves = grid_size * 2;
    let choose = grid_size;

    (1..=choose).fold(1, |paths, i| paths * (moves - choose + i) / i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memoization_counts_small_grids() {
        assert_eq!(solve_with_memoization(1), 2);
        assert_eq!(solve_with_memoization(2), 6);
        assert_eq!(solve_with_memoization(3), 20);
    }

    #[test]
    fn combinatorics_counts_small_grids() {
        assert_eq!(solve_with_combinatorics(1), 2);
        assert_eq!(solve_with_combinatorics(2), 6);
        assert_eq!(solve_with_combinatorics(3), 20);
    }

    #[test]
    fn solves_twenty_by_twenty_grid() {
        assert_eq!(solve_with_memoization(20), 137_846_528_820);
        assert_eq!(solve_with_combinatorics(20), 137_846_528_820);
    }

    #[test]
    fn memoized_and_combinatorics_match() {
        for size in 1..=20 {
            assert_eq!(
                solve_with_memoization(size),
                solve_with_combinatorics(size as u64)
            );
        }
    }
}
