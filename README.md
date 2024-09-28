## Rust bindings for LIKWID 

This library allows controlling [LIKWID](https://hpc.fau.de/research/tools/likwid/) performance monitoring
through its [Marker API](https://github.com/RRZE-HPC/likwid/wiki/TutorialMarkerC).

### Documentation

This crate just provides bindings for a handful of functions, refer to LIKWID documentation on how they work.

Note: all functions take null-terminated C-Strings, which you can create with `c"hello"` literals in Rust.

The cargo feature `enable` is enabled by default, disabling it turns all functions into no-ops. 
To disable it, use this in your `Cargo.toml`

```toml
likwid-marker = { version = "0.1.0", features = [] }
```

Now all LIKWID calls are disabled by default, and you can enable them by passing `--features likwid-marker/enable` to cargo.

### Safety

None of the functions exported by this crate are marked `unsafe`, as using them the wrong way only impacts measurements
and does not make the program unsound (as far as I can tell, if that's wrong please let me know).

### Contributing

Only the most common functions are exported from this crate, if you need more feel free to add them and open a merge request.
