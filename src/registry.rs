use std::os::unix::prelude::{AsRawFd, FromRawFd, OwnedFd};

use crate::{err, interest, source, sys, token};

pub struct Registry {
    pub(crate) fd: OwnedFd,
}

impl Registry {
    pub fn register<S>(
        &self,
        source: &mut S,
        token: token::Token,
        interests: interest::Interest,
    ) -> err::Result<()>
    where
        S: source::Source,
    {
        source.register(self, token, interests)
    }

    pub fn reregister<S>(
        &self,
        source: &mut S,
        token: token::Token,
        interests: interest::Interest,
    ) -> err::Result<()>
    where
        S: source::Source,
    {
        source.reregister(self, token, interests)
    }

    pub fn deregister<S>(&self, source: &mut S) -> err::Result<()>
    where
        S: source::Source,
    {
        source.deregister(self)
    }

    pub fn try_clone(&self) -> err::Result<Registry> {
        let dup_fd = unsafe {
            OwnedFd::from_raw_fd(sys::syscall!(fcntl(
                self.fd.as_raw_fd(),
                libc::F_DUPFD_CLOEXEC,
                3
            ))?)
        };

        Ok(Registry { fd: dup_fd })
    }
}
