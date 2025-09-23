use alloc::alloc::GlobalAlloc;
use core::alloc::Layout;

#[global_allocator]
static ALLOCATOR: GlobalAllocator = GlobalAllocator;

struct GlobalAllocator;

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!("layout={layout:?}")
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!()
    }
}
