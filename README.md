# rust-special-functions
A library of special-function implementations for Rust.
This is in VERY early stages of development, not intended for use at the moment.

The goal is to have a pure rust library with a full suite of implementations for most known special functions for both real and complex types.  Generally, the goal is to have full precision results where possible (though "correct rounding" in all cases may be infeasible).  Where this goal is not achieved, it will be documented.

The primary focus will be on standard double-precision floating-point types (f64 and types derived from that).  For special-functions with only low-precision implementations available, there will be f32 support only.  Although internally some higher-precision types are used, these are not planned to be supported for external use, and there is no current plan for building an arbitrary-precision library.
