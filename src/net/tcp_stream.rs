use std::os::unix::prelude::AsRawFd;

use crate::{err, interest, registry, source, sys, token};

pub struct TcpStream(std::net::TcpStream);

impl TcpStream {
    pub fn connect<A>(addr: A) -> err::Result<Self>
    where
        A: std::net::ToSocketAddrs,
    {
        let stream = std::net::TcpStream::connect(addr)?;

        Self::from_std(stream)
    }

    pub fn from_std(stream: std::net::TcpStream) -> err::Result<Self> {
        stream.set_nonblocking(true)?;

        Ok(TcpStream(stream))
    }

    pub fn local_addr(&self) -> err::Result<std::net::SocketAddr> {
        Ok(self.0.local_addr()?)
    }

    pub fn nodelay(&self) -> err::Result<bool> {
        Ok(self.0.nodelay()?)
    }

    pub fn peek(&self, buf: &mut [u8]) -> err::Result<usize> {
        Ok(self.0.peek(buf)?)
    }

    pub fn peek_addr(&self) -> err::Result<std::net::SocketAddr> {
        Ok(self.0.peer_addr()?)
    }

    pub fn set_nodelay(&self, nodelay: bool) -> err::Result<()> {
        Ok(self.0.set_nodelay(nodelay)?)
    }

    pub fn set_ttl(&self, ttl: u32) -> err::Result<()> {
        Ok(self.0.set_ttl(ttl)?)
    }

    pub fn shutdown(&self, how: std::net::Shutdown) -> err::Result<()> {
        Ok(self.0.shutdown(how)?)
    }

    pub fn take_error(&self) -> err::Result<Option<std::io::Error>> {
        Ok(self.0.take_error()?)
    }

    pub fn ttl(&self) -> err::Result<u32> {
        Ok(self.0.ttl()?)
    }
}

impl source::Source for TcpStream {
    fn register(
        &mut self,
        registry: &registry::Registry,
        token: token::Token,
        interests: interest::Interest,
    ) -> err::Result<()> {
        let mut event = libc::epoll_event {
            events: interests.bits(),
            u64: token.0,
        };

        sys::syscall!(epoll_ctl(
            registry.fd.as_raw_fd(),
            libc::EPOLL_CTL_ADD,
            self.0.as_raw_fd(),
            &mut event as *mut _
        ))?;

        Ok(())
    }

    fn reregister(
        &mut self,
        registry: &registry::Registry,
        token: token::Token,
        interests: interest::Interest,
    ) -> err::Result<()> {
        let mut event = libc::epoll_event {
            events: interests.bits(),
            u64: token.0,
        };

        sys::syscall!(epoll_ctl(
            registry.fd.as_raw_fd(),
            libc::EPOLL_CTL_MOD,
            self.0.as_raw_fd(),
            &mut event as *mut _
        ))?;

        Ok(())
    }

    fn deregister(&mut self, registry: &registry::Registry) -> err::Result<()> {
        sys::syscall!(epoll_ctl(
            registry.fd.as_raw_fd(),
            libc::EPOLL_CTL_MOD,
            self.0.as_raw_fd(),
            std::ptr::null_mut::<libc::epoll_event>()
        ))?;

        Ok(())
    }
}

impl std::io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl std::io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
