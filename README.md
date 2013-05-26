This is a small (under-300-line) stub that you can import into your Rust
projects to create almost completely freestanding Rust programs. The only
dependencies are the following libc functions: `malloc`, `free`, `abort`,
`memcpy`, and `memcmp`. See `hello.rs` for an example of usage.

With a small modification, you could implement syscalls (via inline assembly)
and an allocator yourself to avoid the dependencies on these functions. This
was not done in the interests of simplicity.

Unfortunately, a couple of runtime functions still have to be around, although
they are never called in the resulting programs. They could be stubbed out.
This should be considered a Rust bug.

Garbage collection, tasks, and failure will not work. All the other language
features, including unique pointers, references, and closures *do* work, so
programs such as sprocketnes that do not use garbage collection should be
fine.

The Rust standard library cannot be used in this mode, but splitting out the
Rust standard library into a portion that can be used in freestanding mode
would be an interesting and valuable project (IMO).

As with the rest of Rust, this should not be considered production-ready.

