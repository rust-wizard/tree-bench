# Understanding Your JMT Benchmark Report

This document explains the benchmark report you received from running the JMT (Jellyfish Merkle Tree) benchmarks.

## What is Being Tested

The benchmark measures the performance of a Jellyfish Merkle Tree implementation (`jmt` crate) for different operations:

1. **jmt_insert**: Measures insertion performance with 10, 100, and 1000 entries
2. **jmt_get**: Measures retrieval performance (not shown in your report)
3. **jmt_update**: Measures update performance (not shown in your report)

## Report Breakdown

### Test Setup
- **Backend**: Since Gnuplot wasn't found, the benchmark used the plotters backend for visualization
- **Samples**: Each test ran 100 measurements to collect performance data
- **Test Sizes**: 10, 100, and 1000 key-value pairs were tested

### Performance Results

#### jmt_insert/insert/10
```
time: [17.862 µs 17.885 µs 17.912 µs]
```
- **Average Insertion Time**: ~17.9 microseconds per operation for 10 entries
- **Range**: Operations took between 17.862µs and 17.912µs
- **Outliers**: 12 outliers found (2% high mild, 10% high severe)

#### jmt_insert/insert/100
```
time: [182.45 µs 182.69 µs 182.98 µs]
```
- **Average Insertion Time**: ~182.7 microseconds total for 100 entries (~1.83µs per entry)
- **Range**: Operations took between 182.45µs and 182.98µs
- **Outliers**: 14 outliers found (3% high mild, 11% high severe)

#### jmt_insert/insert/1000
```
time: [1.7970 ms 1.7983 ms 1.7998 ms]
```
- **Average Insertion Time**: ~1.8 milliseconds total for 1000 entries (~1.8µs per entry)
- **Range**: Operations took between 1.797ms and 1.7998ms
- **Outliers**: 6 outliers found (3% high mild, 3% high severe)
- **Warning**: The benchmark indicates that 100 samples in 5 seconds is insufficient for this larger dataset

### Outlier Analysis

- **High Mild Outliers**: Measurements that are significantly slower than typical but still within reasonable bounds
- **High Severe Outliers**: Measurements that are substantially slower than typical, possibly indicating system interference (GC, context switching, etc.)

### Performance Scaling

Looking at the results:
- 10 entries: ~17.9µs total → ~1.79µs per entry
- 100 entries: ~182.7µs total → ~1.83µs per entry  
- 1000 entries: ~1.8ms total → ~1.8µs per entry

The per-entry performance remains relatively stable as the dataset grows, suggesting good scalability characteristics for the JMT implementation.

### Warning Explanation

The warning for the 1000-entry test suggests that the benchmark duration might be insufficient for reliable statistical analysis. The recommendation is to either:
- Increase the target time to 9.1 seconds
- Enable flat sampling
- Reduce the sample count to 50

This would provide more accurate results for the larger dataset.

## How to Interpret These Results

1. **Lower is Better**: The time measurements represent how long operations take - lower values indicate better performance
2. **Consistency**: Look at the range between minimum and maximum values - a narrow range indicates consistent performance
3. **Scalability**: Compare performance across different dataset sizes to understand how the algorithm scales
4. **Reliability**: Consider outlier percentages - higher outlier counts might indicate inconsistent performance

## Potential Improvements

To get more reliable results for the 1000-entry test, you might want to modify the benchmark settings in `benches/jmt_benchmark.rs` to allow for longer measurement periods for larger datasets.
