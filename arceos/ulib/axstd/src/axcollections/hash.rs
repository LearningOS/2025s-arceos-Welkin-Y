use alloc::string::String;
use core::hash::{Hash, Hasher};

pub struct DefaultHasher;

impl DefaultHasher {
    /// Creates a new `DefaultHasher`.
    ///
    /// This hasher is not guaranteed to be the same as all other
    /// `DefaultHasher` instances, but is the same as all other `DefaultHasher`
    /// instances created through `new` or `default`.
    #[must_use]
    pub const fn new() -> DefaultHasher {
        DefaultHasher
    }
}

impl Default for DefaultHasher {
    /// Creates a new `DefaultHasher` using [`new`].
    /// See its documentation for more.
    ///
    /// [`new`]: DefaultHasher::new
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}

impl Hasher for DefaultHasher {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        println!("Hashing: {:?}", msg);
        todo!("Not implemented yet");
    }

    #[inline]
    fn finish(&self) -> u64 {
        //TODO: implement a real hash function
        println!("Finishing hash");
        todo!("Not implemented yet");
        0
    }
}
