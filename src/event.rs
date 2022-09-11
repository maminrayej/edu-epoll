pub struct Events {
    inner: Vec<libc::epoll_event>,
}

impl Events {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut libc::epoll_event {
        self.inner.as_mut_ptr()
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    // # Safety
    //
    // It's safe if the returned value by the kernel is correct.
    pub(crate) unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
    }
}
