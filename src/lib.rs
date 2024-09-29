//! Bindins for the [LIKWID](https://hpc.fau.de/research/tools/likwid/) marker API.
//!
//! See to the [LIKWID documentation](https://github.com/RRZE-HPC/likwid/wiki/TutorialMarkerC) for
//! a guide on how to use it with `likwid-perfctr`

use std::ffi::{c_char, CStr};

extern "C" {
    fn likwid_markerInit();
    fn likwid_markerThreadInit();
    fn likwid_markerClose();

    fn likwid_markerRegisterRegion(tag: *const c_char) -> i32;
    fn likwid_markerStartRegion(tag: *const c_char) -> i32;
    fn likwid_markerStopRegion(tag: *const c_char) -> i32;
    fn likwid_markerResetRegion(tag: *const c_char) -> i32;

    fn likwid_markerNextGroup();

    fn likwid_markerWriteFile(marker_file: *const c_char) -> i32;

    fn likwid_pinProcess(processor_id: i32) -> i32;
    fn likwid_pinThread(processor_id: i32) -> i32;
}

/// Initialize LIKWID's marker API
///
/// Must be called in serial region of the application to set up basic data
/// structures of LIKWID. Reads environment variables:
/// - LIKWID_MODE (access mode)
/// - LIKWID_MASK (event bitmask)
/// - LIKWID_EVENTS (event string)
/// - LIKWID_THREADS (cpu list separated by ,)
/// - LIKWID_GROUPS (amount of groups)
pub fn init() {
    #[cfg(feature = "enable")]
    unsafe {
        likwid_markerInit();
    }
}

/// Must be called in serial region of the application.
/// It gathers all data of regions and writes them out to a file
/// (filepath in env variable LIKWID_FILEPATH).
pub fn close() {
    #[cfg(feature = "enable")]
    unsafe {
        likwid_markerClose();
    }
}

/// Must be called in serial region of the application.
/// It gathers all data of regions and writes them out to file.
pub fn write_file(marker_file: &CStr) -> Result<(), i32> {
    #[cfg(feature = "enable")]
    unsafe {
        match likwid_markerWriteFile(marker_file.as_ptr()) {
            0 => Ok(()),
            err => Err(err),
        }
    }
}

/// Initialize LIKWID's marker API for the current thread
///
/// Must be called in parallel region of the application to set up basic data structures of LIKWID.
/// Before you can call likwid_markerThreadInit() you have to call likwid_markerInit().
pub fn thread_init() {
    #[cfg(feature = "enable")]
    unsafe {
        likwid_markerThreadInit();
    }
}

/// Register a measurement region
///
/// Initializes the hashTable entry in order to reduce execution time of
/// likwid_markerStartRegion().
pub fn register_region(tag: &CStr) -> Result<(), i32> {
    #[cfg(feature = "enable")]
    unsafe {
        match likwid_markerRegisterRegion(tag.as_ptr()) {
            0 => Ok(()),
            err => Err(err),
        }
    }
}

/// Start a measurement region
///
/// Reads the values of all configured counters and saves the results under the
/// name given in regionTag. Must be called on every thread that is to be measured,
/// e.g. if the code to be measured is run in a parallel region, this function must
/// also be called in a parallel region (typically the same parallel region as the
/// measured code). If this function is to be called multiple times in one parallel
/// region, place a barrier ("#pragma omp barrier" or similar) before each call to
/// likwid_markerStartRegion
pub fn start_region(tag: &CStr) -> Result<(), i32> {
    #[cfg(feature = "enable")]
    unsafe {
        match likwid_markerStartRegion(tag.as_ptr()) {
            0 => Ok(()),
            err => Err(err),
        }
    }
}

/// Stop a measurement region
///
/// Reads the values of all configured counters and saves the results under the
/// name given in regionTag. The measurement data of the stopped region gets summed
/// up in global region counters. Must be called on every thread that is to be
/// measured, e.g. if the code to be measured is run in a parallel region, this
/// function must also be called in a parallel region (typically the same parallel
/// region as the measured code). If this function is called multiple times in one
/// parallel region, place a barrier ("#pragma omp barrier" or similar) after each
/// call to likwid_markerStopRegion
pub fn stop_region(tag: &CStr) -> Result<(), i32> {
    #[cfg(feature = "enable")]
    unsafe {
        match likwid_markerStopRegion(tag.as_ptr()) {
            0 => Ok(()),
            err => Err(err),
        }
    }
}

/// Reset a measurement region
///
/// Reset the values of all configured counters and timers.
pub fn reset_region(tag: &CStr) -> Result<(), i32> {
    #[cfg(feature = "enable")]
    unsafe {
        match likwid_markerResetRegion(tag.as_ptr()) {
            0 => Ok(()),
            err => Err(err),
        }
    }
}

/// Switch to next group to measure
///
/// Should be called in a serial region of code. If it is to be called from inside
/// a parallel region, ensure only one thread runs it by using "#pragma omp single"
/// or similar. Additionally, if this function is called in a parallel region,
/// ensure that the serial regions is preceeded by a barrier ("#pragma omp barrier"
/// or similar) to prevent race conditions.
pub fn next_group() {
    #[cfg(feature = "enable")]
    unsafe {
        likwid_markerNextGroup();
    }
}

/// Pin the current process to the given CPU ID. The process cannot be scheduled to
/// another CPU after pinning but the pinning can be changed anytime with this
/// function.
pub fn pin_process(processor_id: i32) -> Result<(), i32> {
    #[cfg(feature = "enable")]
    unsafe {
        match likwid_pinProcess(processor_id) {
            1 => Ok(()),
            err => Err(err),
        }
    }
}

/// Pin the current thread to the given CPU ID. The thread cannot be scheduled to
/// another CPU after pinning but the pinning can be changed anytime with this
/// function
pub fn pin_thread(processor_id: i32) -> Result<(), i32> {
    #[cfg(feature = "enable")]
    unsafe {
        match likwid_pinThread(processor_id) {
            1 => Ok(()),
            err => Err(err),
        }
    }
}

#[test]
fn call_likwid_fn() {
    init();
}
