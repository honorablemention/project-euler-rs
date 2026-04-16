# Project Euler #15 - RS

<p>Starting in the top left corner of a 2 by 2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.</p>

<p>How many such routes are there through a 20 by 20 grid?</p>

## Implementation Notes

This solution keeps both a top-down memoized recursive solver and a combinatorics solver for comparison.

The default executable uses the combinatorics solver:

```text
C(40, 20)
```

For a 20 by 20 grid, every route contains exactly 20 right moves and 20 down moves. That means each route is an ordering of 40 total moves where 20 positions are chosen for one direction.

The memoized recursive solution is kept purely as an example comparison of approach. It computes the number of paths from each coordinate once and stores those results in a `HashMap<(u32, u32), u64>`.

## Benchmarking

Criterion benchmarks live in `benches/lattice_paths.rs`.

Run them locally with:

```sh
cargo bench --bench lattice_paths
```

Run them in Docker with:

```sh
make bench EULER=015 BENCH=lattice_paths
```

The benchmark suite compares:

- `solve_with_memoization`
- `solve_with_combinatorics`
