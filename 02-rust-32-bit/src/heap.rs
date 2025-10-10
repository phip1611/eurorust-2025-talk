//! Module for the heap and the global Rust allocator.

#![allow(static_mut_refs)]

use alloc::vec;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::OnceCell;
use core::hint::black_box;
use core::ptr::NonNull;
use spin::Mutex as SpinMutex;
use talc::{ErrOnOom, Span, Talc};

/// Heap size of 32 MiB.
const HEAP_SIZE: usize = 0x2000000;

/// Heap backing memory backed into the kernel ELF.
#[used]
static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[global_allocator]
static HEAP_ALLOCATOR: Allocator = Allocator::new();

struct Allocator {
    inner: SpinMutex<OnceCell<Talc<ErrOnOom>>>,
}

impl Allocator {
    const fn new() -> Self {
        Self {
            inner: SpinMutex::new(OnceCell::new()),
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut lock = self.inner.lock();
        let talc = lock.get_mut().expect("should have been initialized");
        // SAFETY: The backing memory is valid.
        let alloc_ptr = unsafe { talc.malloc(layout).expect("should be able to allocate") };
        alloc_ptr.as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut lock = self.inner.lock();
        let talc = lock.get_mut().expect("should have been initialized");
        // SAFETY: The backing memory is valid.
        unsafe {
            talc.free(NonNull::new(ptr).unwrap(), layout);
        };
    }
}

/// Initializes the global allocator, i.e., the heap of the loader.
pub fn init() {
    HEAP_ALLOCATOR.inner.lock().get_or_init(|| {
        // SAFETY: We are protected by a lock and only do this once on valid
        // memory.
        let heap = unsafe { HEAP_MEM.as_mut_ptr() };

        let mut talc = Talc::new(ErrOnOom);
        let span = Span::from_base_size(heap.cast(), HEAP_SIZE);
        unsafe {
            talc.claim(span)
                .expect("span {span} should be valid memory")
        };
        talc
    });

    log::debug!(
        "initialized simple heap: allocations work: vec={:?}",
        black_box(vec![1, 2, 3])
    );
}
