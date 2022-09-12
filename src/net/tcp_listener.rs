use std::os::unix::prelude::AsRawFd;

use crate::{err, interest, net, registry, source, sys, token};

pub struct TcpListener(std::net::TcpListener);
impl TcpListener {
    pub fn bind<A>(addr: A) -> err::Result<Self>
    where
        A: std::net::ToSocketAddrs,
    {
        let listener = std::net::TcpListener::bind(addr)?;

        TcpListener::from_std(listener)
    }

    pub fn accept(&self) -> err::Result<(net::TcpStream, std::net::SocketAddr)> {
        let (stream, addr) = self.0.accept()?;

        Ok((net::TcpStream::from_std(stream)?, addr))
    }

    pub fn from_std(listener: std::net::TcpListener) -> err::Result<TcpListener> {
        listener.set_nonblocking(true)?;

        Ok(TcpListener(listener))
    }

    pub fn local_addr(&self) -> err::Result<std::net::SocketAddr> {
        Ok(self.0.local_addr()?)
    }

    pub fn set_ttl(&self, ttl: u32) -> err::Result<()> {
        Ok(self.0.set_ttl(ttl)?)
    }

    pub fn ttl(&self) -> err::Result<u32> {
        Ok(self.0.ttl()?)
    }

    pub fn take_error(&self) -> err::Result<Option<std::io::Error>> {
        Ok(self.0.take_error()?)
    }
}

impl source::Source for TcpListener {
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
