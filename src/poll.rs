use std::os::unix::prelude::{AsRawFd, FromRawFd, OwnedFd};

use crate::{err, event, registry, sys};

pub struct Poll {
    registry: registry::Registry,
}

impl Poll {
    pub fn new() -> err::Result<Self> {
        let fd = unsafe { OwnedFd::from_raw_fd(sys::syscall!(epoll_create1(libc::O_CLOEXEC))?) };

        Ok(Poll {
            registry: registry::Registry { fd },
        })
    }

    pub fn poll(
        &mut self,
        events: &mut event::Events,
        timeout: std::time::Duration,
    ) -> err::Result<()> {
        let ready_count = sys::syscall!(epoll_wait(
            self.registry.fd.as_raw_fd(),
            events.as_mut_ptr(),
            events.capacity() as i32,
            timeout.as_millis() as i32
        ))?;

        unsafe { events.set_len(ready_count as usize) };

        Ok(())
    }

    pub fn registry(&self) -> &registry::Registry {
        &self.registry
    }
}
