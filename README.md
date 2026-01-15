# tree-bench

A benchmark suite for testing JMT (Jellyfish Merkle Tree) operations using Criterion.

## Benchmarks

This project includes benchmarks for various JMT operations:
- Insert operations: Measures performance when inserting different numbers of key-value pairs
- Get operations: Tests retrieval performance with pre-populated trees
- Update operations: Evaluates performance when updating existing keys

## Running Benchmarks

To run the benchmarks, use the following command:

```bash
cargo bench
```

This will execute all the benchmark tests and provide detailed performance metrics including:
- Average execution time
- Variance in measurements
- Statistical analysis of performance characteristics

## Requirements

- Rust toolchain (stable)
- Cargo

## Dependencies

- `jmt`: Jellyfish Merkle Tree implementation
- `criterion`: Statistics-driven microbenchmarking library
- `tempfile`: Secure temporary file and directory utilities
- `sha2`: SHA-2 hash function implementation
