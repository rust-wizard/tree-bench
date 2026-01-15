# Visualizing Benchmark Tests in Rust with Criterion.rs

This guide explains various approaches to visualize benchmark test results in your Rust project using Criterion.rs.

## Overview

Criterion.rs provides several built-in visualization options for your benchmark results. The benchmark data is stored in `target/criterion/` and can be viewed in multiple formats.

## Method 1: HTML Reports (Built-in)

Criterion.rs automatically generates HTML reports that provide rich visualizations:

1. **Run your benchmarks:**
   ```bash
   cargo bench
   ```

2. **Locate the HTML reports:**
   The reports are generated in `target/criterion/` directory:
   ```
   target/criterion/
   ├── jmt_insert/
   │   ├── report/
   │   │   ├── index.html      # Summary report for jmt_insert benchmark
   │   │   └── ...
   │   └── ...
   └── ...
   ```

3. **Open the main summary report in your browser:**
   ```bash
   open target/criterion/jmt_insert/report/index.html
   ```
   
   Or open individual parameter reports:
   ```bash
   open target/criterion/jmt_insert/insert/report/index.html
   open target/criterion/jmt_insert/insert/10/report/index.html
   open target/criterion/jmt_insert/insert/100/report/index.html
   open target/criterion/jmt_insert/insert/1000/report/index.html
   ```

## Method 2: Installing Gnuplot for Enhanced Plots

The benchmark output mentioned "Gnuplot not found, using plottars backend". Installing Gnuplot will provide enhanced visualization capabilities:

1. **Install Gnuplot:**
   - On Ubuntu/Debian: `sudo apt-get install gnuplot`
   - On macOS with Homebrew: `brew install gnuplot`
   - On Windows: Download from https://sourceforge.net/projects/gnuplot/

2. **Re-run benchmarks:**
   ```bash
   cargo bench
   ```

   Now Criterion.rs will use Gnuplot for generating plots, which typically produces higher quality visualizations.

## Method 3: Custom Visualization Script

You can create a script to parse the benchmark data and create custom visualizations:

```python
#!/usr/bin/env python3
"""
Custom visualization script for Criterion.rs benchmark data
"""
import json
import matplotlib.pyplot as plt
import os
from pathlib import Path

def parse_criterion_data():
    """Parse benchmark data from target/criterion directory"""
    base_path = Path("target") / "criterion"
    
    if not base_path.exists():
        print("No benchmark data found. Run 'cargo bench' first.")
        return {}
    
    benchmark_results = {}
    
    for benchmark_dir in base_path.iterdir():
        if benchmark_dir.is_dir() and benchmark_dir.name != "report":
            benchmark_name = benchmark_dir.name
            
            # Find all parameter subdirectories
            for param_dir in benchmark_dir.iterdir():
                if param_dir.is_dir():
                    param_name = param_dir.name
                    
                    # Load the estimates.json file
                    estimates_file = param_dir / "base" / "estimates.json
                    if estimates_file.exists():
                        with open(estimates_file, 'r') as f:
                            data = json.load(f)
                            
                        # Extract mean time estimate
                        mean_estimate = data.get('mean', {}).get('point_estimate', 0)
                        
                        if benchmark_name not in benchmark_results:
                            benchmark_results[benchmark_name] = {}
                        
                        benchmark_results[benchmark_name][param_name] = mean_estimate
    
    return benchmark_results

def create_visualization(data):
    """Create a bar chart visualization of benchmark results"""
    if not data:
        print("No data to visualize")
        return
    
    for benchmark_name, results in data.items():
        if results:
            # Convert to microseconds for readability if needed
            labels = []
            values = []
            
            for param, time_ns in results.items():
                labels.append(f"{benchmark_name}/{param}")
                values.append(time_ns / 1000)  # Convert nanoseconds to microseconds
            
            plt.figure(figsize=(10, 6))
            bars = plt.bar(labels, values)
            plt.title(f"Benchmark Results: {benchmark_name}")
            plt.ylabel("Time (microseconds)")
            plt.xticks(rotation=45, ha="right")
            
            # Add value labels on bars
            for bar, value in zip(bars, values):
                plt.text(bar.get_x() + bar.get_width()/2, bar.get_height(),
                        f'{value:.2f}μs', ha='center', va='bottom')
            
            plt.tight_layout()
            plt.savefig(f"{benchmark_name}_visualization.png", dpi=300, bbox_inches='tight')
            plt.show()

if __name__ == "__main__":
    data = parse_criterion_data()
    create_visualization(data)
```

## Method 4: Using cargo-show-asm for Performance Analysis

For deeper performance analysis, you can use `cargo-show-asm` to visualize assembly code:

```bash
# Install cargo-show-asm
cargo install cargo-show-asm

# View assembly for specific functions
cargo asm --release [function-name]
```

## Method 5: Flame Graphs with flamegraph

Another visualization approach is using flame graphs to see where time is spent in your code. Flame graphs provide a visual representation of function call stacks and CPU time distribution.

### Installing flamegraph:
```bash
cargo install flamegraph
```

### Basic flame graph generation:
```bash
# Generate a flame graph for your benchmark
cargo flamegraph --bench jmt_benchmark
```

### More specific flame graph options:
```bash
# Generate flame graph for a specific benchmark function
cargo flamegraph --bench jmt_benchmark -- jmt_insert

# Generate flame graph with specific parameters
cargo flamegraph --bench jmt_benchmark --bin jmt_benchmark
```

### Alternative approach using perf (Linux):
If the direct integration with Criterion doesn't work well, you can use perf with flamegraph:

1. **Build your benchmark in release mode:**
   ```bash
   cargo bench --bench jmt_benchmark -- --profile-time=5
   ```

2. **Find the benchmark binary:**
   ```bash
   find target -name "*jmt_benchmark*" -type f -executable
   ```

3. **Run the binary with perf and generate a flame graph:**
   ```bash
   # Record performance data
   perf record -g target/release/deps/jmt_benchmark-<hash> --bench
   # Generate the flame graph
   perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg
   ```

### Interpreting Flame Graphs:
- Wider sections represent functions that consume more CPU time
- Colors are typically random and don't have meaning
- Y-axis represents the call stack depth
- X-axis represents the percentage of CPU time spent in each function

Flame graphs are especially useful for identifying performance bottlenecks and understanding the call hierarchy in your code during benchmark execution.

## Method 6: Continuous Benchmark Tracking

For ongoing visualization of performance trends:

1. **Install critcmp:**
   ```bash
   cargo install critcmp
   ```

2. **Compare benchmark runs:**
   ```bash
   # Run benchmarks and save results
   cargo bench --save-baseline baseline
   
   # Make changes to your code
   
   # Run benchmarks again
   cargo bench --save-baseline new_version
   
   # Compare results
   critcmp baseline new_version
   ```

## Best Practices for Visualization

1. **Run benchmarks multiple times** to ensure consistency of results
2. **Use release builds** (`--release`) for accurate performance measurements
3. **Minimize system load** during benchmarking for more reliable results
4. **Track performance regressions** by comparing against previous versions
5. **Document your methodology** for reproducing benchmark results

## Viewing Your Current Benchmark Data

Since you already have benchmark data, you can view it immediately:

1. Run: `cargo bench` (to ensure latest data is generated)
2. Open: `target/criterion/jmt_insert/report/index.html` in your browser
3. Navigate to specific benchmark reports like:
   - `target/criterion/jmt_insert/insert/report/index.html` (overview of all parameter sizes)
   - `target/criterion/jmt_insert/insert/10/report/index.html` (for 10 entries)
   - `target/criterion/jmt_insert/insert/100/report/index.html` (for 100 entries)
   - `target/criterion/jmt_insert/insert/1000/report/index.html` (for 1000 entries)

The HTML reports include interactive charts, statistical information, and regression analysis that are very helpful for understanding performance characteristics.
