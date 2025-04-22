#[no_std]
extern crate alloc;
use alloc::string::String;
use core::hash::{Hash, Hasher};

pub struct DefaultHasher {
    state: u64,
}

impl DefaultHasher {
    /// Creates a new `DefaultHasher`.
    ///
    /// This hasher is not guaranteed to be the same as all other
    /// `DefaultHasher` instances, but is the same as all other `DefaultHasher`
    /// instances created through `new` or `default`.
    #[must_use]
    pub const fn new() -> DefaultHasher {
        DefaultHasher { state: 0 }
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
    /// Dumb hash
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        for byte in msg {
            self.state = self
                .state
                .wrapping_mul(1145141919810)
                .wrapping_add(*byte as u64);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.state
    }
}
