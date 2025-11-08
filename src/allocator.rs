use core::ptr;

pub struct LinkedListAllocator {
    heap_start: *mut u8,
    heap_size: usize,
    free_head: *mut FreeNode,
}

// safe to Send, internal pointers will only be used by this allocator
unsafe impl Send for LinkedListAllocator {}

impl LinkedListAllocator {
    pub unsafe fn new(heap_start: *mut u8, heap_size: usize) -> Self {
        let free_head = FreeNode::new(heap_size, ptr::null_mut());
        heap_start.cast::<FreeNode>().write(free_head);

        Self {
            heap_start,
            heap_size,
            free_head: heap_start.cast(),
        }
    }

    // TODO: improve alloac/dealloc to actually honor layout things, like alignment

    pub unsafe fn alloc(&mut self, layout: core::alloc::Layout) -> Result<*mut u8, &'static str> {
        let mut prev = ptr::null_mut::<FreeNode>();
        let mut curr = self.free_head;

        while !curr.is_null() {
            let node = curr.as_ref().unwrap();

            // TODO: properly handle alignment and any other Layout specific things
            if node.size >= layout.size() {
                let next_ptr = curr.add(layout.size());

                let next_node = FreeNode::new(node.size - layout.size(), node.next);
                next_ptr.write(next_node);

                if prev.is_null() {
                    // this is the first iteration, so curr is the allocator's head node
                    self.free_head = next_ptr;
                } else {
                    prev.as_mut().unwrap().next = next_ptr;
                }

                return Ok(curr.cast());
            } else {
                prev = curr;
                curr = node.next;
            }
        }

        Err("No free regions large enough to fit allocation!")
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: core::alloc::Layout) {
        // write a free chunk to the given address, pointing to the existing allocator's head
        let node = FreeNode::new(layout.size(), self.free_head);
        ptr.cast::<FreeNode>().write(node);
        self.free_head = ptr.cast::<FreeNode>();
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
