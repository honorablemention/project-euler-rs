use euler014::{solve_with_iter, solve_with_loop};

#[test]
fn iter_finds_expected_answer() {
    assert_eq!(solve_with_iter(1_000_000), 837799);
}

#[test]
fn loop_finds_expected_answer() {
    assert_eq!(solve_with_loop(1_000_000), 837799);
}

#[test]
fn loop_finds_expected_small_answer() {
    assert_eq!(solve_with_loop(10), 9);
}
