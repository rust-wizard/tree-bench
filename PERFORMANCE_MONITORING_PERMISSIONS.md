# Resolving Performance Monitoring Permissions Error

This document explains the `perf_event_paranoid` error and how to resolve it when using performance profiling tools like perf and flamegraph.

## Understanding the Error

The error message indicates that your system's `perf_event_paranoid` setting is set to 4, which is more restrictive than the default. This setting controls access to performance monitoring and observability operations in Linux:

- `-1`: Allow use of (almost) all events by all users (most permissive)
- `0`: Default - Disallow raw and ftrace function tracepoint access
- `1`: Disallow CPU event access
- `2`: Disallow kernel profiling
- `3`: Disallow use of unprivileged eBPF (on newer kernels)
- `4`: Disallow use of perf_event_open syscall by unprivileged users (your current setting)

With a setting of 4, only processes with specific Linux capabilities (CAP_PERFMON, CAP_SYS_PTRACE, or CAP_SYS_ADMIN) can use performance monitoring tools.

## Solutions

### Option 1: Temporarily Lower the Setting (Recommended for Testing)

Run this command as root or with sudo to temporarily change the setting:

```bash
sudo sysctl -w kernel.perf_event_paranoid=1
```

This allows CPU event access but still restricts kernel profiling. You can use values from -1 to 2 depending on your needs:

- `kernel.perf_event_paranoid=-1`: Most permissive, allows almost all events
- `kernel.perf_event_paranoid=0`: Default, disallows raw tracepoints
- `kernel.perf_event_paranoid=1`: Disallows CPU events (still allows basic profiling)
- `kernel.perf_event_paranoid=2`: Disallows kernel profiling

### Option 2: Make the Setting Permanent

To make the change permanent across reboots, add the setting to `/etc/sysctl.conf`:

1. Edit the sysctl configuration:
```bash
sudo nano /etc/sysctl.conf
```

2. Add this line:
```
kernel.perf_event_paranoid=1
```

3. Apply the changes:
```bash
sudo sysctl -p
```

### Option 3: Run Profiling Tools with Elevated Privileges

As a temporary workaround, you can run your profiling tools with sudo:

```bash
sudo cargo flamegraph --bench jmt_benchmark
```

⚠️ **Warning**: Running cargo with sudo is generally not recommended for security reasons, as it runs build scripts with elevated privileges.

### Option 4: Use Alternative Profiling Methods

If you cannot change the system settings, consider alternative profiling approaches:

1. **Use built-in Criterion.rs profiling** - The HTML reports already provide performance insights
2. **Use Rust-specific profilers** that may work within the current restrictions
3. **Profile in a virtual machine or container** where you control the settings

## Security Considerations

The `perf_event_paranoid=4` setting is often applied for security reasons because:

- Performance monitoring can be used for side-channel attacks (like Spectre/Meltdown)
- It prevents unauthorized profiling of system processes
- It limits access to sensitive performance data

Before changing this setting, consider:
- Your security requirements
- The environment where the system is running
- Whether the profiling is needed for development only

## Recommended Approach

For development environments, setting `kernel.perf_event_paranoid=1` is usually appropriate as it balances functionality with security:

```bash
# Temporary change
sudo sysctl -w kernel.perf_event_paranoid=1

# Permanent change
echo "kernel.perf_event_paranoid=1" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

After applying one of these solutions, you should be able to use flamegraph and other profiling tools without encountering the permission error.
