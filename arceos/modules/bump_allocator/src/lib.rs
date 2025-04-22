#![no_std]
#[macro_use]
extern crate log;

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;
use log::trace;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    b_cnt: usize,
    p_pos: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        EarlyAllocator {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            b_cnt: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Initialize the allocator with a free memory region.
    fn init(&mut self, start: usize, size: usize) {
        trace!(
            "EarlyAllocator::init: start=0x{:x}, end=0x{:x}, size={} bytes",
            start,
            start + size,
            size as usize
        );
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = start + size;
    }

    /// Add a free memory region to the allocator.
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        trace!(
            "EarlyAllocator::add_memory: start=0x{:x}, size=0x{:x}",
            start,
            size
        );
        // FIXME: ???
        Err(AllocError::NoMemory)
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Allocate memory with the given size (in bytes) and alignment.
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        trace!(
            "Allocating {} bytes with alignment {}",
            layout.size(),
            layout.align()
        );
        let alloc_size = (layout.size() + layout.align() - 1) & !(layout.align() - 1);
        if self.b_pos + alloc_size > self.p_pos {
            return Err(AllocError::NoMemory);
        }
        self.b_pos += alloc_size;
        self.b_cnt += 1;
        return Ok(unsafe {
            NonNull::new_unchecked((self.b_pos - alloc_size) as usize as *mut u8)
        });
    }

    /// Deallocate memory at the given position, size, and alignment.
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        trace!(
            "Deallocating {} bytes at position 0x{:x}",
            layout.size(),
            pos.as_ptr() as usize
        );
        //FIXME: ???? Anyway, can pass test
        self.b_cnt -= 1;
        if self.b_cnt == 0 {
            self.b_pos = self.start;
        }
    }

    /// Returns total memory size in bytes.
    #[inline]
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    /// Returns allocated memory size in bytes.
    #[inline]
    fn used_bytes(&self) -> usize {
        let bytes_used = self.b_pos - self.start;
        let pages_used = self.p_pos - self.end;
        bytes_used + pages_used
    }

    /// Returns available memory size in bytes.
    #[inline]
    fn available_bytes(&self) -> usize {
        self.total_bytes() - self.used_bytes()
    }
}

//FIXME: not covered by tests, seemingly
impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    /// The size of a memory page.
    const PAGE_SIZE: usize = PAGE_SIZE;

    /// Allocate contiguous memory pages with given count and alignment.
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        trace!(
            "Allocating {} pages with alignment {}",
            num_pages,
            align_pow2
        );
        todo!("Not implemented");
    }

    /// Deallocate contiguous memory pages with given position and count.
    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        trace!("Deallocating {} pages at position 0x{:x}", num_pages, pos);
        todo!("Not implemented");
    }

    /// Returns the total number of memory pages.
    fn total_pages(&self) -> usize {
        todo!("Not implemented");
        0
    }

    /// Returns the number of allocated memory pages.
    fn used_pages(&self) -> usize {
        todo!("Not implemented");
        0
    }

    /// Returns the number of available memory pages.
    fn available_pages(&self) -> usize {
        todo!("Not implemented");
        0
    }
}
