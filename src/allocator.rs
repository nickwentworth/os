use crate::{mutex::Mutex, println};
use core::{alloc::GlobalAlloc, ptr::null_mut};

type LockedLLA = Mutex<Option<LinkedListAllocator>>;

#[global_allocator]
static ALLOCATOR: LockedLLA = Mutex::new(None);

const HEAP_START: *mut u8 = 0xFFFF_0000_4000_0000 as *mut u8;
const HEAP_SIZE: usize = 1024 * 1024;

pub fn init_global_allocator() {
    println!("Initializing kernel heap...");

    let mut guard = ALLOCATOR.lock();

    match guard.as_ref() {
        Some(_) => println!("Kernel already initialized!"),
        None => {
            let mut allocator = LinkedListAllocator::new(HEAP_START, HEAP_SIZE);
            unsafe { allocator.init() };
            *guard = Some(allocator);
            println!("Kernel heap initialized!");
        }
    }
}

struct LinkedListAllocator {
    heap_start: *mut u8,
    heap_size: usize,
    free_head: *mut FreeNode,
}

// safe to Send, internal pointers will only be used by this allocator
unsafe impl Send for LinkedListAllocator {}

impl LinkedListAllocator {
    /// Create a new `LinkedListAllocator`, specifying an area for the heap
    fn new(heap_start: *mut u8, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_size,
            free_head: null_mut(),
        }
    }

    /// Initialize the area of memory to be used
    ///
    /// This initialization involves direct writing to a given pointer,
    /// so this function should only be called once per allocator
    unsafe fn init(&mut self) {
        let free_head = FreeNode::new(self.heap_size, null_mut());
        self.heap_start.cast::<FreeNode>().write(free_head);
        self.free_head = self.heap_start.cast();
    }
}

unsafe impl GlobalAlloc for LockedLLA {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut guard = self.lock();
        let allocator = guard.as_mut().expect("Allocator was not initialized");
        println!("Allocating {} bytes", layout.size());

        let mut prev = null_mut::<FreeNode>();
        let mut curr = allocator.free_head;

        while !curr.is_null() {
            let node = curr.as_ref().unwrap();

            // TODO: properly handle alignment and any other Layout specific things
            if node.size >= layout.size() {
                let next_ptr = curr.add(layout.size());

                let next_node = FreeNode::new(node.size - layout.size(), node.next);
                next_ptr.write(next_node);

                if prev.is_null() {
                    // this is the first iteration, so curr is the allocator's head node
                    allocator.free_head = next_ptr;
                } else {
                    prev.as_mut().unwrap().next = next_ptr;
                }

                return curr.cast();
            } else {
                prev = curr;
                curr = node.next;
            }
        }

        panic!("No free regions large enough to fit allocation!");
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut guard = self.lock();
        let allocator = guard.as_mut().expect("Allocator was not initialized");
        println!("Deallocating {} bytes", layout.size());

        // write a free chunk to the given address, pointing to the existing allocator's head
        let node = FreeNode::new(layout.size(), allocator.free_head);
        ptr.cast::<FreeNode>().write(node);
        allocator.free_head = ptr.cast::<FreeNode>();

        // TODO: look to combine free regions if memory directly after ptr is also free
    }
}

/// An element of the `LinkedListAllocator`, stored implicitly in free sections of the heap.
struct FreeNode {
    size: usize,
    next: *mut Self,
}

impl FreeNode {
    fn new(size: usize, next: *mut Self) -> Self {
        Self { size, next }
    }
}
