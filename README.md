## Rust bindings for LIKWID 

This library allows controlling [LIKWID](https://hpc.fau.de/research/tools/likwid/) performance monitoring
through its [Marker API](https://github.com/RRZE-HPC/likwid/wiki/TutorialMarkerC).

## Building

If the LIKWID library (`liblikwid.so`) is not where your system normally searches for dynamic libraries,
set the `LIKWID_LIB_DIR` environment variable to the path containing the `likwid` library.

You can run `cargo test` in this repo to check if linking to `liblikwid` is working.

The cargo feature `enable` is enabled by default, disabling it turns all functions into no-ops. 
To disable it, use this in your `Cargo.toml`

```toml
likwid-marker = { version = "0.1.1", default-features = false }
```

Now all LIKWID calls are disabled and `liblikwid` is not linked by default,
and you can enable them by passing `--features likwid-marker/enable` to cargo.

### Documentation

This crate just provides bindings for a handful of functions, refer to LIKWID documentation on how they work.

Note: all functions take null-terminated C-Strings, which you can create with `c"hello"` literals in Rust.

### Safety

None of the functions exported by this crate are marked `unsafe`, as using them the wrong way only impacts measurements
and does not make the program unsound (as far as I can tell, if that's wrong please let me know).

### Contributing

Only the most common functions are exported from this crate, if you need more feel free to add them and open a merge request.
