//! [`ByteSliceIter`] reads bytes from a reader and allows iterating over them as slices with a
//! maximum length, similar to the [`chunks`] method on slices.
//!
//! It is implemented as a [`FallibleStreamingIterator`] so that it can reuse its buffer and not
//! allocate for each chunk. (That trait is re-exported here for convenience.)
//!
//! # Examples
//! ```
//! use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};
//! use std::fs::File;
//! # use std::io;
//! # fn foo() -> io::Result<()> {
//! let f = File::open("src/lib.rs")?;
//! // Iterate over the file's contents in 8-byte chunks.
//! let mut iter = ByteSliceIter::new(f, 8);
//! while let Some(chunk) = iter.next()? {
//!     println!("{:?}", chunk);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! [`ByteSliceIter`]: struct.ByteSliceIter.html
//! [`chunks`]: https://doc.rust-lang.org/std/primitive.slice.html#method.chunks
//! [`FallibleStreamingIterator`]: ../fallible_streaming_iterator/trait.FallibleStreamingIterator.html

extern crate fallible_streaming_iterator;

// re-export this so callers don't have to explicitly depend on fallible-streaming-iterator.
pub use fallible_streaming_iterator::FallibleStreamingIterator;
use std::cmp;
use std::io::{self, BufRead, BufReader, Read};

/// An iterator over byte slices from a `Read` that reuses the same buffer instead of allocating.
///
/// See the [crate documentation] for example usage.
///
/// [crate documentation]: index.html
pub struct ByteSliceIter<R>
where
    R: Read,
{
    inner: BufReader<R>,
    buf: Vec<u8>,
}

impl<R> ByteSliceIter<R>
where
    R: Read,
{
    /// Create a new `ByteSliceIter` that reads from `inner` and produces slices of length
    /// `chunk_len`. If `size` does not divide the total number of bytes read evenly the last
    /// chunk will not have length `size`.
    pub fn new(inner: R, size: usize) -> ByteSliceIter<R> {
        ByteSliceIter {
            inner: BufReader::new(inner),
            // It would be nice to not need the extra buffer here, but there isn't an API to
            // ask BufReader for its current buffer without reading more, and
            // `FallibleStreamingIterator::get` doesn't return a `Result`.
            buf: Vec::with_capacity(size),
        }
    }
}

impl<'a, R> FallibleStreamingIterator for ByteSliceIter<R>
where
    R: Read,
{
    type Item = [u8];
    type Error = io::Error;

    fn advance(&mut self) -> Result<(), io::Error> {
        if self.buf.len() > 0 {
            self.inner.consume(self.buf.len());
            self.buf.clear();
        }
        let buf = self.inner.fill_buf()?;
        let cap = self.buf.capacity();
        self.buf.extend_from_slice(
            &buf[..cmp::min(buf.len(), cap)],
        );
        Ok(())
    }

    fn get(&self) -> Option<&[u8]> {
        if self.buf.len() > 0 {
            Some(self.buf.as_slice())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
