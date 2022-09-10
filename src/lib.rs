use std::os::unix::prelude::{AsRawFd, FromRawFd, OwnedFd};

use bitflags::bitflags;

macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* )) => {{
        #[allow(unused_unsafe)]
        let result = unsafe { libc::$fn($($arg, )*) };

        if result == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(result)
        }
    }};
}

bitflags! {
    pub struct Interest: u32 {
        const READ = libc::EPOLLIN as u32;
        const WRITE = libc::EPOLLOUT as u32;
    }
}

pub struct Poll {
    registry: Registry,
}

pub struct Events {
    inner: Vec<libc::epoll_event>,
}

impl Poll {
    pub fn new() -> Result<Self, std::io::Error> {
        let fd = unsafe { OwnedFd::from_raw_fd(syscall!(epoll_create1(libc::O_CLOEXEC))?) };

        Ok(Poll {
            registry: Registry { fd },
        })
    }
}

pub struct Registry {
    fd: OwnedFd,
}
