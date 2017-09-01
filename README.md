This crate implements a type `ByteSliceIter` that reads bytes from a reader and allows iterating
over them as slices with a maximum length, similar to the [`chunks`] method on slices.

It is implemented as a [`FallibleStreamingIterator`] so that it can reuse its buffer and not
allocate for each chunk. (That trait is re-exported here for convenience.)

# Example
```rust,skt-main
use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};

let bytes = b"0123456789abcdef0123456789abcdef";
// Iterate over the bytes in 8-byte chunks.
let mut iter = ByteSliceIter::new(&bytes[..], 8);
while let Some(chunk) = iter.next()? {
    println!("{:?}", chunk);
}
```

# License

`read-byte-slice` is distributed under the terms of both the MIT license and
the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

[`chunks`]: https://doc.rust-lang.org/std/primitive.slice.html#method.chunks
[`FallibleStreamingIterator`]: https://docs.rs/fallible-streaming-iterator/*/fallible_streaming_iterator/trait.FallibleStreamingIterator.html
